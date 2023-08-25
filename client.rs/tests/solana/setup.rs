use std::{
    ffi::OsStr,
    process::{Command, Stdio},
    thread::sleep,
    time::Duration,
};

use anchor_client::solana_sdk::transaction::Transaction;
use anchor_client::solana_sdk::{program_pack::Pack, signature::Keypair};
use anchor_client::{
    solana_client::{
        client_error::ClientErrorKind, nonblocking::rpc_client::RpcClient, rpc_request::RpcError,
    },
    solana_sdk::native_token::LAMPORTS_PER_SOL,
    solana_sdk::pubkey::Pubkey,
    solana_sdk::{
        signature::Signer,
        system_instruction::{create_account, create_account_with_seed},
    },
};

use hapi_core::client::implementations::solana::get_network_account;

use spl_associated_token_account::{create_associated_token_account, get_associated_token_address};
use spl_token::solana_program::instruction::Instruction;

use spl_token::instruction::{initialize_mint, mint_to};

use super::{fixtures::*, validator_utils::*};
use crate::cmd_utils::*;

pub struct Setup {
    pub data: TestData,
    cli: RpcClient,
    provider_url: String,
}

impl Setup {
    pub async fn new() -> Setup {
        let data = TestData::default();
        shut_down_existing_validator();
        start_validator();

        let provider_url = format!("http://localhost:{VALIDATOR_PORT}");
        let cli = RpcClient::new(provider_url.clone());

        let setup = Self {
            data,
            cli,
            provider_url,
        };
        setup.setup_wallets().await;
        setup.prepare_validator().await;

        setup
    }

    async fn airdrop(&self, wallet: &Pubkey) {
        let amount = LAMPORTS_PER_SOL * 10;

        self.cli
            .request_airdrop(&wallet, amount)
            .await
            .expect("Failed to airdrop");

        loop {
            if self.cli.get_balance(&wallet).await.unwrap() >= amount {
                break;
            }
            sleep(Duration::from_millis(100));
        }
    }

    async fn setup_wallets(&self) {
        println!("==> Creating mint account");
        let payer = &self.data.get_wallet("admin").keypair;

        self.airdrop(&payer.pubkey()).await;
        // self.create_mint().await;

        println!("==> Preparing wallets");
        for (key, wallet) in &self.data.wallets {
            if !key.eq(&"mint") {
                let owner_keypair = &wallet.keypair;

                self.airdrop(&owner_keypair.pubkey()).await;
                // self.create_ata(owner_keypair).await;
            }
        }
    }

    async fn prepare_validator(&self) {
        println!("==> Deploying the contract");

        let program_dir = &self.data.program_dir;
        let admin_keypair = &self.data.get_wallet("admin").path;
        let program_keypair = &self.data.program_keypair_path;

        ensure_cmd(
            Command::new("anchor")
                .args([
                    "deploy",
                    "--program-keypair",
                    program_keypair,
                    "--provider.wallet",
                    &admin_keypair,
                    "--program-name",
                    PROGRAM_NAME,
                ])
                .env("ANCHOR_WALLET", admin_keypair)
                .stdout(Stdio::null())
                .current_dir(program_dir),
        )
        .unwrap();

        println!("==> Creating network for tests");

        ensure_cmd(
            Command::new("npm")
                .args(["run", "create-network", NETWORK])
                .stdout(Stdio::null())
                .env("ANCHOR_WALLET", admin_keypair)
                .current_dir(program_dir),
        )
        .unwrap();

        let program_id = HAPI_CORE_PROGRAM_ID
            .parse::<Pubkey>()
            .expect("Invalid program id");
        let network_account = get_network_account(NETWORK, &program_id).expect("Invalid network");

        loop {
            match self.cli.get_account(&network_account).await {
                Ok(_) => {
                    println!("==> Network created");
                    break;
                }
                Err(e) => {
                    if let ClientErrorKind::RpcError(RpcError::ForUser(_)) = *e.kind() {
                        tokio::time::sleep(std::time::Duration::from_millis(500)).await;
                        continue;
                    } else {
                        panic!("{}", e);
                    }
                }
            }
        }

        println!("==> Successful setup");
    }

    async fn create_acc_instruction(
        &self,
        account_keypair: &Keypair,
        payer_keypair: &Keypair,
        size: usize,
    ) -> Instruction {
        let account_pubkey = account_keypair.pubkey();
        let payer_account_pubkey = payer_keypair.pubkey();

        let mint_account_rent = self
            .cli
            .get_minimum_balance_for_rent_exemption(size)
            .await
            .unwrap();

        create_account(
            &payer_account_pubkey,
            &account_pubkey,
            mint_account_rent as u64,
            size as u64,
            &spl_token::id(),
        )
    }

    async fn create_mint(&self) {
        let mint_account_keypair = &self.data.get_wallet("mint").keypair;
        let payer_account_keypair = &self.data.get_wallet("admin").keypair;
        let mint_account_pubkey = mint_account_keypair.pubkey();
        let payer_account_pubkey = payer_account_keypair.pubkey();

        let create_account_instruction = self
            .create_acc_instruction(
                &mint_account_keypair,
                &payer_account_keypair,
                spl_token::state::Mint::LEN,
            )
            .await;

        let initialize_mint_instruction = initialize_mint(
            &spl_token::id(),
            &mint_account_pubkey,
            &payer_account_pubkey,
            None,
            9,
        )
        .unwrap();

        let recent_blockhash = self.cli.get_latest_blockhash().await.unwrap();

        let transaction = Transaction::new_signed_with_payer(
            &vec![create_account_instruction, initialize_mint_instruction],
            Some(&payer_account_pubkey),
            &[payer_account_keypair, mint_account_keypair],
            recent_blockhash,
        );

        self.cli
            .send_and_confirm_transaction_with_spinner(&transaction)
            .await
            .expect("Failed to create mint");
    }

    async fn create_ata(&self, owner_keypair: &Keypair) {
        let mint_address = &self.data.get_wallet("mint").keypair.pubkey();
        let payer_account = &self.data.get_wallet("admin").keypair;
        let owner_pubkey = owner_keypair.pubkey();

        let recent_blockhash = self.cli.get_latest_blockhash().await.unwrap();

        let create_ata_instruction =
            create_associated_token_account(&owner_pubkey, &owner_pubkey, mint_address);

        let create_ata_tx = Transaction::new_signed_with_payer(
            &[create_ata_instruction],
            Some(&owner_pubkey),
            &[owner_keypair],
            recent_blockhash,
        );

        self.cli
            .send_and_confirm_transaction_with_spinner(&create_ata_tx)
            .await
            .expect("Failed to create ATA");

        // TODO: fix this
        // let ata = get_associated_token_address(&owner_pubkey, mint_address);

        // let mint_instruction = mint_to(
        //     &spl_token::id(),
        //     &mint_address,
        //     &ata,
        //     &owner_pubkey,
        //     &[&payer_account.pubkey()],
        //     100_000,
        // )
        // .unwrap();

        // let mint_transaction = Transaction::new_signed_with_payer(
        //     &[mint_instruction],
        //     Some(&payer_account.pubkey()),
        //     &[payer_account],
        //     recent_blockhash,
        // );

        // rpc_client
        //     .send_and_confirm_transaction_with_spinner(&mint_transaction)
        //     .expect("Failed to mint to ATA");
    }

    pub fn exec<I, S>(&self, args: I) -> anyhow::Result<CmdOutput>
    where
        I: IntoIterator<Item = S>,
        S: AsRef<OsStr>,
    {
        let contract_address = HAPI_CORE_PROGRAM_ID.to_string();
        let network = NETWORK.to_string();
        let provider_url = self.provider_url.clone();

        wrap_cmd(
            Command::new("./target/debug/hapi-core-cli")
                .args(args)
                .env("OUTPUT", "json")
                .env("CONTRACT_ADDRESS", contract_address)
                .env("NETWORK", network)
                .env("PROVIDER_URL", provider_url),
        )
    }

    pub fn is_tx_match(value: &serde_json::Value) -> bool {
        let signature = value
            .get("tx")
            .expect("`tx` key not found")
            .as_str()
            .expect("`tx` is not a string");

        bs58::decode(signature)
            .into_vec()
            .is_ok_and(|s| s.len() == 64)
    }

    pub fn print(&self, message: &str) {
        println!("==> {message} [{}]", VALIDATOR_PORT);
    }

    // pub async fn get_tx(&self, hash: &str) -> Transaction {
    //     let provider = Provider::<Http>::try_from(self.provider_url.clone()).unwrap();

    //     let tx_hash = ethers::types::H256::from_str(hash).expect("Expected valid transaction hash");
    //     let tx = provider
    //         .get_transaction(tx_hash)
    //         .await
    //         .expect("Could not retrieve transaction");

    //     tx.expect("Transaction not found")
    // }

    // pub async fn get_block(&self, hash: &str) -> Block<H256> {
    //     let provider = Provider::<Http>::try_from(self.provider_url.clone()).unwrap();

    //     let block_hash = ethers::types::H256::from_str(hash).expect("Expected valid block hash");
    //     let block = provider
    //         .get_block(block_hash)
    //         .await
    //         .expect("Could not retrieve block");

    //     block.expect("Block not found")
    // }

    // pub async fn get_tx_timestamp(&self, hash: &str) -> u64 {
    //     let tx = self.get_tx(hash).await;
    //     let block_hash = format!("{:.unwrap()}", tx.block_hash.expect("block hash is expected"));
    //     let block = self.get_block(&block_hash).await;
    //     block.timestamp.as_u64()
    // }
}
