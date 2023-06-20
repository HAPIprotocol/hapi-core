use ethers::{types::H160, utils::to_checksum as ethers_to_checksum};
use regex::Regex;

pub fn is_tx_match(value: &serde_json::Value) -> bool {
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
    ($output:expr, $json_val:expr) => {{
        let output = $output.unwrap_or_else(|e| panic!("{}", e));

        if !output.success {
            panic!("Expected command success: {:?}", output);
        }

        let value =
            serde_json::from_str::<serde_json::Value>(&output.stdout).expect("json parse error");

        assert_eq!(value, $json_val, "correct json value expected");

        value
    }};
}

#[macro_export]
macro_rules! assert_tx_output {
    ($output:expr) => {{
        let output = $output.unwrap_or_else(|e| panic!("{}", e));

        if !output.success {
            panic!("Expected command success: {:?}", output);
        }

        let value =
            serde_json::from_str::<serde_json::Value>(&output.stdout).expect("json parse error");

        assert!(is_tx_match(&value), "transaction hash expected");

        value
            .get("tx")
            .and_then(serde_json::Value::as_str)
            .map(|s| s.to_owned())
            .expect("`tx` key not found or not a string")
    }};
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
