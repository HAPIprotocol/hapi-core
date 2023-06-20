use ethers::{
    providers::{Http, Middleware, Provider},
    types::{Block, Transaction, H256},
};
use std::{
    env,
    ffi::OsStr,
    process::{Command, Stdio},
    str::FromStr,
    thread::sleep,
    time::Duration,
};

pub const PUBLIC_KEY_1: &str = "0xf39fd6e51aad88f6f4ce6ab8827279cfffb92266";
pub const PRIVATE_KEY_1: &str =
    "0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80";
pub const REPORTER_UUID_1: &str = "0ca77383-0d2a-4090-98c9-31b69e105b12";

pub const PUBLIC_KEY_2: &str = "0x70997970c51812dc3a010c7d01b50e0d17dc79c8";
pub const PRIVATE_KEY_2: &str =
    "0x59c6995e998f97a5a0044966f0945389dc9e86dae88c7a8412f4603b6b78690d";
pub const REPORTER_UUID_2: &str = "2ef0a8f9-66c8-4be2-981a-b9236bb43f61";

pub struct Setup {
    pub token_contract: String,
    port: u16,
    pub contract_address: String,
    network: String,
    provider_url: String,
}

#[derive(Debug)]
pub struct CmdOutput {
    pub success: bool,
    pub stdout: String,
    pub stderr: String,
}

impl Default for Setup {
    fn default() -> Self {
        Self::new()
    }
}

impl Setup {
    pub fn new() -> Setup {
        let port = get_port();

        println!("[{port}] ==> Setting up the environment");

        env::set_var("PRIVATE_KEY", PRIVATE_KEY_1);

        println!("[{port}] ==> Compiling the contract");
        ensure_cmd(
            Command::new("npm")
                .args(["run", "build"])
                .current_dir("../evm"),
        )
        .unwrap();

        println!("[{port}] ==> Checking if the node is running");
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
            println!("[{port}] ==> Killing the node: {pid}");
            ensure_cmd(
                Command::new("kill")
                    .args([&pid.to_string()])
                    .stderr(Stdio::null()),
            )
            .unwrap();
        }

        println!("[{port}] ==> Starting the node");
        Command::new("npx")
            .args(["hardhat", "node", "--port", &port.to_string()])
            .current_dir("../evm")
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
            .expect("Failed to execute command");

        println!("[{port}] ==> Waiting for the node to start");
        sleep(Duration::from_millis(1000));

        println!("[{port}] ==> Deploying the contract");
        ensure_cmd(
            Command::new("npm")
                .args(["run", "deploy"])
                .env("HARDHAT_NETWORK", "localhost")
                .env("HARDHAT_LOCALHOST_URL", format!("http://127.0.0.1:{port}"))
                .current_dir("../evm"),
        )
        .unwrap();

        println!("[{port}] ==> Deploying the test token contract");
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
        println!("[{}] ==> {message}", self.port);
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

fn ensure_cmd(command: &mut Command) -> anyhow::Result<()> {
    let output = command.output();

    println!("Exec: {:?}", command);

    if let Err(e) = output {
        panic!("Failed to execute command: {e}");
    }

    let output = output.unwrap();

    let stderr = String::from_utf8_lossy(&output.stderr);
    let stdout = String::from_utf8_lossy(&output.stdout);

    if !stderr.trim().is_empty() {
        println!("STDERR:\n{stderr}");
    }
    if !stdout.trim().is_empty() {
        println!("STDOUT:\n{stdout}");
    }

    if !output.status.success() {
        return Err(anyhow::anyhow!("Failed to execute command {:?}", command));
    }

    Ok(())
}

fn wrap_cmd(command: &mut Command) -> anyhow::Result<CmdOutput> {
    let output = command.output()?;

    println!("Exec: {:?}", command);

    let stderr = String::from_utf8_lossy(&output.stderr);
    let stdout = String::from_utf8_lossy(&output.stdout);

    Ok(CmdOutput {
        success: output.status.success(),
        stdout: stdout.trim().to_owned(),
        stderr: stderr.trim().to_owned(),
    })
}
