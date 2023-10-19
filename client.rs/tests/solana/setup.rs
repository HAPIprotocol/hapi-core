use std::{ffi::OsStr, process::Command, str::FromStr, thread::sleep, time::Duration};

use anchor_client::{
    solana_client::{
        client_error::ClientErrorKind, nonblocking::rpc_client::RpcClient, rpc_request::RpcError,
    },
    solana_sdk::{
        native_token::LAMPORTS_PER_SOL,
        program_pack::Pack,
        pubkey::Pubkey,
        signature::{read_keypair_file, Keypair, Signature, Signer},
        system_instruction::create_account,
        transaction::Transaction,
    },
};

use hapi_core::client::implementations::solana::get_network_address;
use solana_transaction_status::UiTransactionEncoding;
use spl_associated_token_account::{
    get_associated_token_address, instruction::create_associated_token_account,
};
use spl_token::{
    instruction::{initialize_mint, mint_to},
    solana_program::instruction::Instruction,
};

use super::{fixtures::*, validator_utils::*};
use crate::cmd_utils::*;

pub struct Setup {
    pub authority: Keypair,
    pub publisher: Keypair,
    pub stake_mint: Keypair,
    pub reward_mint: Keypair,
    cli: RpcClient,
    provider_url: String,
}

impl Setup {
    pub async fn new() -> Setup {
        let dir = std::env::current_dir()
            .expect("Unable to detect directory")
            .to_string_lossy()
            .to_string();

        let authority = read_keypair_file(format!("{}/{}/{}", dir, KEYS_DIR, AUTHORITY_KEYPAIR))
            .expect("Invalid keypair");
        let publisher = read_keypair_file(format!("{}/{}/{}", dir, KEYS_DIR, PUBLISHER_KEYPAIR))
            .expect("Invalid keypair");
        let stake_mint = read_keypair_file(format!("{}/{}/{}", dir, KEYS_DIR, STAKE_MINT_KEYPAIR))
            .expect("Invalid keypair");
        let reward_mint =
            read_keypair_file(format!("{}/{}/{}", dir, KEYS_DIR, REWAED_MINT_KEYPAIR))
                .expect("Invalid keypair");

        shut_down_existing_validator();
        start_validator();

        let provider_url = format!("http://localhost:{VALIDATOR_PORT}");
        let cli = RpcClient::new(provider_url.clone());

        let setup = Self {
            authority,
            publisher,
            stake_mint,
            reward_mint,
            cli,
            provider_url,
        };

        setup.setup_wallets().await;
        prepare_validator(&dir, &setup.provider_url).await;
        setup.check_validator_setup().await;

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
        self.airdrop(&self.authority.pubkey()).await;
        self.airdrop(&self.publisher.pubkey()).await;

        println!("==> Creating mint accounts");
        self.create_mint(&self.stake_mint).await;
        self.create_mint(&self.reward_mint).await;

        println!("==> Preparing wallets");
        self.create_ata(&self.authority, &self.stake_mint.pubkey())
            .await;
        self.create_ata(&self.publisher, &self.stake_mint.pubkey())
            .await;
    }

    async fn check_validator_setup(&self) {
        let program_id = CONTRACT_ADDRESS
            .parse::<Pubkey>()
            .expect("Invalid program id");
        let network_account = get_network_address(NETWORK, &program_id)
            .expect("Invalid network")
            .0;

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

    async fn create_mint(&self, mint: &Keypair) {
        let payer_address = self.authority.pubkey();
        let create_account_instruction = self
            .create_acc_instruction(&mint, &self.authority, spl_token::state::Mint::LEN)
            .await;

        let initialize_mint_instruction =
            initialize_mint(&spl_token::id(), &mint.pubkey(), &payer_address, None, 9).unwrap();

        let recent_blockhash = self.cli.get_latest_blockhash().await.unwrap();

        let transaction = Transaction::new_signed_with_payer(
            &vec![create_account_instruction, initialize_mint_instruction],
            Some(&payer_address),
            &[&self.authority, &mint],
            recent_blockhash,
        );

        self.cli
            .send_and_confirm_transaction_with_spinner(&transaction)
            .await
            .expect("Failed to create mint");
    }

    async fn create_ata(&self, owner_keypair: &Keypair, mint_address: &Pubkey) {
        let payer_address = self.authority.pubkey();
        let owner_pubkey = owner_keypair.pubkey();
        let recent_blockhash = self.cli.get_latest_blockhash().await.unwrap();

        let create_ata_instruction = create_associated_token_account(
            &owner_pubkey,
            &owner_pubkey,
            mint_address,
            &spl_token::id(),
        );

        let create_ata_tx = Transaction::new_signed_with_payer(
            &[create_ata_instruction],
            Some(&owner_pubkey),
            &[owner_keypair],
            recent_blockhash,
        );

        let ata = get_associated_token_address(&owner_pubkey, mint_address);
        println!("==> Creating and minting to ATA: {}", ata);

        self.cli
            .send_and_confirm_transaction_with_spinner(&create_ata_tx)
            .await
            .expect("Failed to create ATA");

        let mint_instruction = mint_to(
            &spl_token::id(),
            mint_address,
            &ata,
            &payer_address,
            &[&payer_address],
            1_000_000_000,
        )
        .unwrap();

        let mint_transaction = Transaction::new_signed_with_payer(
            &[mint_instruction],
            Some(&payer_address),
            &[&self.authority],
            recent_blockhash,
        );

        self.cli
            .send_and_confirm_transaction_with_spinner(&mint_transaction)
            .await
            .expect("Failed to mint to ATA");
    }

    pub fn exec<I, S>(&self, args: I) -> anyhow::Result<CmdOutput>
    where
        I: IntoIterator<Item = S>,
        S: AsRef<OsStr>,
    {
        let contract_address = CONTRACT_ADDRESS.to_string();
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

    pub async fn get_tx_timestamp(&self, hash: &str) -> u64 {
        let tx = self
            .cli
            .get_transaction(
                &Signature::from_str(hash).expect("Invalid signature"),
                UiTransactionEncoding::Base64,
            )
            .await
            .expect("Failed to get transaction");

        tx.block_time.expect("Transaction not found") as u64
    }
}

impl Drop for Setup {
    fn drop(&mut self) {
        // shut_down_existing_validator();
    }
}
