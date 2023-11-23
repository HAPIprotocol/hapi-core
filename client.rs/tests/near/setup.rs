use dirs;
use regex::Regex;
use std::{
    env,
    ffi::OsStr,
    process::{Command, Output},
    thread,
    time::Duration,
};

use crate::{
    cmd_utils::{wrap_cmd, CmdOutput},
    util::wait_for_port,
};

pub struct Account {
    pub account: String,
    pub secret_key: String,
}

pub struct Setup {
    pub token_contract: String,
    pub contract_address: String,
    pub authority: Account,
    pub reporter: Account,
    pub network: String,
    provider_url: String,
}

impl Default for Setup {
    fn default() -> Self {
        Self::new()
    }
}

#[test]
fn setup_local_near_node() {
    Setup::new();
}

impl Setup {
    pub fn new() -> Setup {
        const PORT: u16 = 3030;

        let mut docker_run_counter = 0;
        let mut docker_output: String = String::new();

        // check if docker is running
        while docker_output.len() == 0 {
            let get_containers = Command::new("docker")
                .args(["ps", "-a"])
                .output()
                .expect("Failed to execute command");

            if get_containers.status.success() {
                docker_output =
                    String::from_utf8(get_containers.stdout).expect("Failed to decode output")
            } else {
                let err = String::from_utf8(get_containers.stderr).expect("Failed to decode error");

                if !err.contains("docker daemon running") {
                    panic!("Failed to check docker: {}", err);
                }
            }

            if docker_output.len() == 0 && docker_run_counter == 0 {
                println!("==> Opening docker");
                Command::new("open")
                    .args(["-a", "Docker"])
                    .output()
                    .expect("Failed to execute command");
            }

            thread::sleep(Duration::from_secs(1));
            docker_run_counter += 1;
            if docker_run_counter > 30 {
                panic!("Failed to run docker");
            }
        }

        // get containers list and check if node is running
        if docker_output.contains("node_master") {
            // remove node if it is running
            println!("==> Stopping a node");
            Command::new("docker")
                .args(["rm", "-f", "node_master"])
                .output()
                .expect("Failed to execute command")
                .stdout
                .iter()
                .map(|&x| x as char)
                .collect::<String>()
                .trim()
                .to_string();
        } else {
            println!("==> node_master is not running");
        }

        // run docker node
        println!("==> Running a node");
        Command::new("docker")
            .args([
                "run",
                "--name",
                "node_master",
                "-d",
                "-e",
                "INIT=1",
                "-p3030:3030",
                "nearprotocol/nearcore:1.35.0",
            ])
            .output()
            .expect("Failed to execute command");

        wait_for_port(PORT);

        println!("==> Node is running [{PORT}]");

        copy_credentials();

        println!("==> Creating accounts");

        let hapi_account = create_account("hapi");
        let token_account = create_account("token");
        let reporter_account = create_account("reporter");
        let authority_account = create_account("authority");

        let hapi_pk = get_secret_key(&hapi_account);
        let reporter_pk = get_secret_key(&reporter_account);
        let authority_pk = get_secret_key(&authority_account);

        env::set_var("PRIVATE_KEY", hapi_pk);
        env::set_var("ACCOUNT_ID", hapi_account.clone());

        println!("==> Deploying contracts");

        exec_near_cmd([
            "deploy",
            "--accountId",
            &hapi_account,
            "--wasmFile=../near/res/hapi_core_near.wasm",
        ]);

        exec_near_cmd([
            "call",
            &hapi_account,
            "initialize",
            "{}",
            "--accountId",
            &hapi_account,
        ]);

        exec_near_cmd([
            "deploy",
            "--accountId",
            &token_account,
            "--wasmFile",
            "../near/res/fungible_token.wasm",
        ]);

        exec_near_cmd([
            "call",
            &token_account,
            "new_default_meta",
            &format!(
                "{{\"owner_id\": \"{}\", \"total_supply\": \"1000000000000000000000000000\"}}",
                &token_account
            ),
            "--accountId",
            &token_account,
        ]);

        exec_near_cmd([
            "call",
            &token_account,
            "storage_deposit",
            "{}",
            "--accountId",
            &hapi_account,
            "--deposit",
            "0.00125",
        ]);

        exec_near_cmd([
            "call",
            &token_account,
            "storage_deposit",
            "{}",
            "--accountId",
            &reporter_account,
            "--deposit",
            "0.00125",
        ]);

        exec_near_cmd([
            "call",
            &token_account,
            "storage_deposit",
            "{}",
            "--accountId",
            &authority_account,
            "--deposit",
            "0.00125",
        ]);

        exec_near_cmd([
            "call",
            &token_account,
            "ft_transfer",
            &format!(
                "{{\"receiver_id\": \"{}\", \"amount\": \"20000000000000000000000000\"}}",
                &authority_account
            ),
            "--accountId",
            &token_account,
            "--depositYocto",
            "1",
        ]);
        exec_near_cmd([
            "call",
            &token_account,
            "ft_transfer",
            &format!(
                "{{\"receiver_id\": \"{}\", \"amount\": \"20000000000000000000000000\"}}",
                &reporter_account
            ),
            "--accountId",
            &token_account,
            "--depositYocto",
            "1",
        ]);

        Setup {
            token_contract: token_account,
            contract_address: hapi_account,
            authority: Account {
                account: authority_account,
                secret_key: authority_pk,
            },
            reporter: Account {
                account: reporter_account,
                secret_key: reporter_pk,
            },
            network: "near".to_string(),
            provider_url: format!("http://127.0.0.1:{PORT}"),
        }
    }

    pub fn print(&self, message: &str) {
        println!("==> {message}");
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

    pub fn is_tx_match(value: &serde_json::Value) -> bool {
        Regex::new(r"[0-9a-zA-Z]{43,44}$").unwrap().is_match(
            value
                .get("tx")
                .expect("`tx` key not found")
                .as_str()
                .expect("`tx` is not a string"),
        )
    }
}

fn create_account(account: &str) -> String {
    let account = format!("{account}.test.near");

    assert!(
        Command::new("near")
            .args([
                "create-account",
                &account,
                "--masterAccount",
                "test.near",
                "--initialBalance",
                "10",
            ])
            .env("NEAR_ENV", "localnet")
            .output()
            .expect("Failed to execute command")
            .status
            .success(),
        "Failed to create account: {}",
        account
    );
    account
}

fn copy_credentials() {
    let home_dir = dirs::home_dir().expect("Unable to get home directory");

    let target_path = home_dir.join(".near-credentials").join("local");

    if !target_path.exists() {
        std::fs::create_dir_all(&target_path).expect("Failed to create directory");
    }

    let target_path = target_path.join("validator_key.json");

    let output = Command::new("docker")
        .args([
            "cp",
            "node_master:/srv/near/validator_key.json",
            target_path.to_str().expect("Failed to convert path to str"),
        ])
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success(), "Failed to copy credentials");

    let target_path = home_dir.join(".near");

    if !target_path.exists() {
        std::fs::create_dir_all(&target_path).expect("Failed to create directory");
    }

    let target_path = target_path.join("validator_key.json");

    let output = Command::new("docker")
        .args([
            "cp",
            "node_master:/srv/near/validator_key.json",
            target_path.to_str().expect("Failed to convert path to str"),
        ])
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success(), "Failed to copy credentials");
}

fn exec_near_cmd<I, S>(args: I) -> Output
where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
{
    let output = Command::new("near")
        .args(args)
        .env("NEAR_ENV", "localnet")
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success(), "Failed to build near cmd");

    println!("==> near cmd output: {:?}", output);

    output
}

fn get_secret_key(account_name: &String) -> String {
    let home_dir = dirs::home_dir().expect("Unable to get home directory");

    let target_path = home_dir
        .join(".near-credentials")
        .join("local")
        .join(format!("{account_name}.json"));

    let pk = near_crypto::InMemorySigner::from_file(&target_path).expect("Failed to read key file");
    pk.secret_key.to_string()
}
