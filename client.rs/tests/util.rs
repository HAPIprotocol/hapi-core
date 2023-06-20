use ethers::{types::H160, utils::to_checksum as ethers_to_checksum};
use regex::Regex;
use serde_json::Value;

pub fn to_json(value: &str) -> Value {
    serde_json::from_str::<Value>(value).expect("json parse error")
}

pub fn is_tx_match(value: Value) -> bool {
    Regex::new(r"^0x[0-9a-fA-F]{64}$").unwrap().is_match(
        value
            .get("tx")
            .expect("`tx` key not found")
            .as_str()
            .expect("`tx` is not a string"),
    )
}

pub fn to_checksum(value: &str) -> String {
    ethers_to_checksum(&value.parse::<H160>().expect("invalid address"), None)
}

#[macro_export]
macro_rules! assert_json_output {
    ($output:expr, $json_val:expr) => {
        let output = $output.unwrap_or_else(|e| panic!("{}", e));

        if !output.success {
            panic!("Expected command success: {:?}", output);
        }

        assert_eq!(
            to_json(&output.stdout),
            $json_val,
            "correct json value expected"
        );
    };
}

#[macro_export]
macro_rules! assert_tx_output {
    ($output:expr) => {
        let output = $output.unwrap_or_else(|e| panic!("{}", e));

        if !output.success {
            panic!("Expected command success: {:?}", output);
        }

        assert!(
            is_tx_match(to_json(&output.stdout)),
            "transaction hash expected"
        );
    };
}

#[macro_export]
macro_rules! assert_error_output {
    ($output:expr, $err_val:expr) => {
        let output = $output.unwrap_or_else(|e| panic!("{}", e));

        if output.success {
            panic!("Expected command error: {:?}", output);
        }

        assert_eq!(output.stderr, $err_val, "correct error message expected");
    };
}
