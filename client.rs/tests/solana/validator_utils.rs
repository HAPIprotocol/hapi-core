use std::{
    process::{Command, Stdio},
    thread::sleep,
    time::Duration,
};

use super::fixtures::*;
use crate::cmd_utils::*;

pub fn get_validator_pid() -> Option<u32> {
    Command::new("sh")
        .arg("-c")
        .arg(format!(
            "lsof -i :{VALIDATOR_PORT} | grep LISTEN | awk '{{print $2}}'"
        ))
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

pub fn shut_down_existing_validator() {
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

pub fn start_validator() {
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

pub async fn prepare_validator(current_dir: &str, provider_url: &str) {
    let program_dir = format!("{}/{}", current_dir, PROGRAM_DIR);
    let admin_keypair = format!("{}/{}/{}", current_dir, KEYS_DIR, AUTHORITY_KEYPAIR);
    let program_keypair = format!("{}/{}/{}", current_dir, PROGRAM_DIR, HAPI_CORE_KEYPAIR);

    println!("==> Deploying the contract");

    ensure_cmd(
        Command::new("anchor")
            .args([
                "deploy",
                "--program-keypair",
                &program_keypair,
                "--provider.wallet",
                &admin_keypair,
                "--program-name",
                PROGRAM_NAME,
            ])
            .env("ANCHOR_WALLET", &admin_keypair)
            .stdout(Stdio::null())
            .current_dir(&program_dir),
    )
    .unwrap();

    println!("==> Creating network for tests");

    ensure_cmd(
        Command::new("npm")
            .args(["run", "create-network", NETWORK])
            .stdout(Stdio::null())
            .env("ANCHOR_PROVIDER_URL", &provider_url)
            .env("ANCHOR_WALLET", &admin_keypair)
            .env("CONTRACT_ADDRESS", CONTRACT_ADDRESS)
            .current_dir(&program_dir),
    )
    .unwrap();
}
