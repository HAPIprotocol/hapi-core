use std::{
    process::{Command, Stdio},
    thread::sleep,
    time::Duration,
};

use super::fixtures::*;
use crate::cmd_utils::*;

pub fn get_validator_pid() -> Option<u32> {
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
