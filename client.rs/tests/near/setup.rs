use dirs;

use std::{
    ffi::OsStr,
    process::{Command, Output},
    thread,
    time::Duration,
};

use super::util::wait_for_port;

pub struct Setup {
    pub token_contract: String,
    pub contract_address: String,
    port: u16,
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
        const PORT: u16 = 3030;

        let mut docker_run_counter = 0;
        let mut docker_output: String = String::new();

        // check if docker is running
        while docker_output.len() == 0 {
            docker_output = Command::new("docker")
                .args(["ps", "-a"])
                .output()
                .expect("Failed to execute command")
                .stdout
                .iter()
                .map(|&x| x as char)
                .collect::<String>()
                .trim()
                .to_string();

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

        println!("==> Deploying contracts");

        let output = Command::new("near")
            .args([
                "deploy",
                "--accountId",
                &hapi_account,
                "--wasmFile=../near/res/hapi_core_near.wasm",
            ])
            .output()
            .expect("Failed to execute command");

        for line in output.stderr.iter() {
            print!("{}", *line as char);
        }

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
                "{{\"owner_id\": \"{}\", \"total_supply\": \"10000000000000000000000000\"}}",
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

        exec_near_cmd(["call", &token_account, "ft_transfer_call", &format!("{{\"receiver_id\": \"{}\", \"amount\": \"10000000000000000000000000\", \"msg\": \"\"}}", &hapi_account), "--accountId", &token_account]);
        exec_near_cmd(["call", &token_account, "ft_transfer_call", &format!("{{\"receiver_id\": \"{}\", \"amount\": \"10000000000000000000000000\", \"msg\": \"\"}}", &reporter_account), "--accountId", &token_account]);

        Setup {
            token_contract: token_account,
            contract_address: hapi_account,
            network: "near".to_string(),
            provider_url: format!("http://127.0.0.1:{PORT}"),
            port: PORT,
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
}

fn wrap_cmd(command: &mut Command) -> anyhow::Result<CmdOutput> {
    let output = command.output()?;

    println!(
        "Exec: {} {}",
        command.get_program().to_string_lossy(),
        command
            .get_args()
            .map(|s| format!("\"{}\"", s.to_string_lossy()))
            .collect::<Vec<_>>()
            .join(" ")
    );

    let stderr = String::from_utf8_lossy(&output.stderr);
    let stdout = String::from_utf8_lossy(&output.stdout);

    Ok(CmdOutput {
        success: output.status.success(),
        stdout: stdout.trim().to_owned(),
        stderr: stderr.trim().to_owned(),
    })
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

    let target_path = home_dir
        .join(".near-credentials")
        .join("local")
        .join("validator_key.json");

    let output = Command::new("docker")
        .args([
            "cp",
            "node_master:/srv/near/validator_key.json",
            target_path.to_str().expect("Failed to convert path to str"),
        ])
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success(), "Failed to copy credentials");

    let target_path = home_dir.join(".near").join("validator_key.json");

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

    output
}
