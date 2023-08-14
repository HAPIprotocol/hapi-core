use ethers::{
    providers::{Http, Middleware, Provider},
    types::{Block, Transaction, H256},
};
use std::{
    env,
    ffi::OsStr,
    process::{Command, Stdio},
    str::FromStr,
};

use super::{fixtures::*, util::wait_for_port};
use crate::cmd_utils::*;

pub struct Setup {
    pub token_contract: String,
    pub contract_address: String,
    port: u16,
    network: String,
    provider_url: String,
}

impl Default for Setup {
    fn default() -> Self {
        Self::new()
    }
}

impl Setup {
    pub fn new() -> Setup {
        let port = get_port();

        println!("==> Setting up the environment [{port}]");

        env::set_var("PRIVATE_KEY", PRIVATE_KEY_1);

        println!("==> Compiling the contract [{port}]");
        ensure_cmd(
            Command::new("npm")
                .args(["run", "build"])
                .current_dir("../evm"),
        )
        .unwrap();

        println!("==> Checking if the node is running [{port}]");
        let maybe_pid: Option<u32> = Command::new("lsof")
            .args(["-t", "-i", &format!(":{port}")])
            .output()
            .expect("Failed to execute command")
            .stdout
            .iter()
            .map(|&x| x as char)
            .collect::<String>()
            .trim()
            .parse()
            .ok();

        if let Some(pid) = maybe_pid {
            println!("==> Killing the node: {pid} [{port}]");
            ensure_cmd(
                Command::new("kill")
                    .args([&pid.to_string()])
                    .stderr(Stdio::null()),
            )
            .unwrap();
        }

        println!("==> Starting the node [{port}]");
        Command::new("npx")
            .args(["hardhat", "node", "--port", &port.to_string()])
            .current_dir("../evm")
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
            .expect("Failed to execute command");

        println!("==> Waiting for the node to start [{port}]");
        wait_for_port(port);

        println!("==> Deploying the contract [{port}]");
        ensure_cmd(
            Command::new("npm")
                .args(["run", "deploy"])
                .env("HARDHAT_NETWORK", "localhost")
                .env("HARDHAT_LOCALHOST_URL", format!("http://127.0.0.1:{port}"))
                .current_dir("../evm"),
        )
        .unwrap();

        println!("==> Deploying the test token contract [{port}]");
        ensure_cmd(
            Command::new("npm")
                .args(["run", "deploy-test-token"])
                .env("HARDHAT_NETWORK", "localhost")
                .env("HARDHAT_LOCALHOST_URL", format!("http://127.0.0.1:{port}"))
                .current_dir("../evm"),
        )
        .unwrap();

        Setup {
            token_contract: "0xCf7Ed3AccA5a467e9e704C703E8D87F634fB0Fc9".to_string(),
            contract_address: "0x9fE46736679d2D9a65F0992F2272dE9f3c7fa6e0".to_string(),
            network: "ethereum".to_string(),
            provider_url: format!("http://127.0.0.1:{port}"),
            port,
        }
    }

    pub fn print(&self, message: &str) {
        println!("==> {message} [{}]", self.port);
    }

    pub fn exec<I, S>(&self, args: I) -> anyhow::Result<CmdOutput>
    where
        I: IntoIterator<Item = S>,
        S: AsRef<OsStr>,
    {
        let token = self.token_contract.clone();
        let contract_address = self.contract_address.clone();
        let network = self.network.clone();
        let provider_url = self.provider_url.clone();

        wrap_cmd(
            Command::new("./target/debug/hapi-core-cli")
                .args(args)
                .env("OUTPUT", "json")
                .env("TOKEN_CONTRACT", token)
                .env("CONTRACT_ADDRESS", contract_address)
                .env("NETWORK", network)
                .env("PROVIDER_URL", provider_url),
        )
    }

    pub async fn get_tx(&self, hash: &str) -> Transaction {
        let provider = Provider::<Http>::try_from(self.provider_url.clone()).unwrap();

        let tx_hash = ethers::types::H256::from_str(hash).expect("Expected valid transaction hash");
        let tx = provider
            .get_transaction(tx_hash)
            .await
            .expect("Could not retrieve transaction");

        tx.expect("Transaction not found")
    }

    pub async fn get_block(&self, hash: &str) -> Block<H256> {
        let provider = Provider::<Http>::try_from(self.provider_url.clone()).unwrap();

        let block_hash = ethers::types::H256::from_str(hash).expect("Expected valid block hash");
        let block = provider
            .get_block(block_hash)
            .await
            .expect("Could not retrieve block");

        block.expect("Block not found")
    }

    pub async fn get_tx_timestamp(&self, hash: &str) -> u64 {
        let tx = self.get_tx(hash).await;
        let block_hash = format!("{:?}", tx.block_hash.expect("block hash is expected"));
        let block = self.get_block(&block_hash).await;
        block.timestamp.as_u64()
    }
}

static mut PORT: u16 = 8545;

fn get_port() -> u16 {
    unsafe {
        PORT += 1;
        PORT
    }
}
