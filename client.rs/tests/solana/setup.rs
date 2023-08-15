use std::{
    ffi::OsStr,
    process::{Command, Stdio},
    thread::sleep,
    time::Duration,
};

use anchor_client::solana_sdk::transaction::Transaction;
use anchor_client::solana_sdk::{program_pack::Pack, signature::Keypair};
use anchor_client::{
    solana_client::rpc_client::RpcClient,
    solana_sdk::native_token::LAMPORTS_PER_SOL,
    solana_sdk::pubkey::Pubkey,
    solana_sdk::{
        signature::Signer,
        system_instruction::{create_account, create_account_with_seed},
    },
};

use spl_associated_token_account::{create_associated_token_account, get_associated_token_address};
use spl_token::solana_program::instruction::Instruction;

use spl_token::instruction::{initialize_mint, mint_to};

use super::fixtures::*;
use crate::cmd_utils::*;

const VALIDATOR_PORT: u16 = 8899;
const PROGRAM_NAME: &str = "hapi_core_solana";

pub struct Setup {
    pub data: TestData,
    cli: RpcClient,
}

fn get_validator_pid() -> Option<u32> {
    Command::new("lsof")
        .args(["-t", "-i", &format!(":{VALIDATOR_PORT}")])
        .output()
        .expect("Failed to execute command")
        .stdout
        .iter()
        .map(|&x| x as char)
        .collect::<String>()
        .trim()
        .parse()
        .ok()
}

fn shut_down_existing_validator() {
    if let Some(pid) = get_validator_pid() {
        println!("==> Killing the node: {pid} [{VALIDATOR_PORT}]");
        ensure_cmd(
            Command::new("kill")
                .args(["-9", &pid.to_string()])
                .stderr(Stdio::null()),
        )
        .unwrap();

        println!("==> Waiting for the node to shut down [{VALIDATOR_PORT}]");
        loop {
            if get_validator_pid().is_none() {
                println!("==> Node shut down [{VALIDATOR_PORT}]");
                break;
            }

            sleep(Duration::from_millis(100));
        }
    }
}

fn start_validator() {
    println!("==> Starting the validator [{VALIDATOR_PORT}]");
    Command::new("solana-test-validator")
        .args(["-r"])
        .stdout(Stdio::null())
        .spawn()
        .expect("Failed to execute command");

    println!("==> Waiting for the validator to start");

    loop {
        if let Some(pid) = get_validator_pid() {
            println!("==> Validator started [{VALIDATOR_PORT}] with pid: {pid}");
            break;
        }

        sleep(Duration::from_millis(100));
    }
}

impl Setup {
    pub fn new() -> Setup {
        let data = TestData::default();
        shut_down_existing_validator();
        start_validator();

        let cli = RpcClient::new(format!("http://localhost:{VALIDATOR_PORT}"));

        let setup = Self { cli, data };
        setup.setup_wallets();
        setup.prepare_validator();

        setup
    }

    fn airdrop(&self, wallet: &Pubkey) {
        let amount = LAMPORTS_PER_SOL * 10;

        self.cli
            .request_airdrop(&wallet, amount)
            .expect("Failed to airdrop");

        loop {
            if self.cli.get_balance(&wallet).unwrap() >= amount {
                break;
            }
            sleep(Duration::from_millis(100));
        }
    }

    fn setup_wallets(&self) {
        println!("==> Creating mint account");
        let payer = &self.data.get_wallet("admin").keypair;

        self.airdrop(&payer.pubkey());
        self.create_mint();

        println!("==> Preparing wallets");
        for (key, wallet) in &self.data.wallets {
            if !key.eq(&"mint") {
                let owner_keypair = &wallet.keypair;

                self.airdrop(&owner_keypair.pubkey());
                self.create_ata(owner_keypair);
            }
        }
    }

    fn prepare_validator(&self) {
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

        println!("==> Successful setup");
    }

    fn create_acc_instruction(
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
            .unwrap();

        create_account(
            &payer_account_pubkey,
            &account_pubkey,
            mint_account_rent as u64,
            size as u64,
            &spl_token::id(),
        )
    }

    fn create_mint(&self) {
        let mint_account_keypair = &self.data.get_wallet("mint").keypair;
        let payer_account_keypair = &self.data.get_wallet("admin").keypair;
        let mint_account_pubkey = mint_account_keypair.pubkey();
        let payer_account_pubkey = payer_account_keypair.pubkey();

        let create_account_instruction = self.create_acc_instruction(
            &mint_account_keypair,
            &payer_account_keypair,
            spl_token::state::Mint::LEN,
        );

        let initialize_mint_instruction = initialize_mint(
            &spl_token::id(),
            &mint_account_pubkey,
            &payer_account_pubkey,
            None,
            9,
        )
        .unwrap();

        let recent_blockhash = self.cli.get_latest_blockhash().unwrap();

        let transaction = Transaction::new_signed_with_payer(
            &vec![create_account_instruction, initialize_mint_instruction],
            Some(&payer_account_pubkey),
            &[payer_account_keypair, mint_account_keypair],
            recent_blockhash,
        );

        self.cli
            .send_and_confirm_transaction_with_spinner(&transaction)
            .expect("Failed to create mint");
    }

    fn create_ata(&self, owner_keypair: &Keypair) {
        let mint_address = &self.data.get_wallet("mint").keypair.pubkey();
        let payer_account = &self.data.get_wallet("admin").keypair;
        let owner_pubkey = owner_keypair.pubkey();

        let recent_blockhash = self.cli.get_latest_blockhash().unwrap();

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
        // let token = self.token_contract.clone();
        // let contract_address = self.contract_address.clone();
        // let network = self.network.clone();
        // let provider_url = self.provider_url.clone();

        // wrap_cmd(
        //     Command::new("./target/debug/hapi-core-cli")
        //         .args(args)
        //         .env("OUTPUT", "json")
        //         .env("TOKEN_CONTRACT", token)
        //         .env("CONTRACT_ADDRESS", contract_address)
        //         .env("NETWORK", network)
        //         .env("PROVIDER_URL", provider_url),
        // )

        Ok(CmdOutput {
            success: true,
            stdout: "".to_string(),
            stderr: "".to_string(),
        })
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
