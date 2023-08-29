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

        let tx = value
            .get("tx")
            .and_then(serde_json::Value::as_str)
            .map(|s| s.to_owned())
            .expect("`tx` key not found or not a string");

        println!("tx: {}", tx);

        tx
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

#[macro_export]
macro_rules! assert_error_output_contains {
    ($output:expr, $err_val:expr) => {
        let output = $output.unwrap_or_else(|e| panic!("{}", e));

        if output.success {
            panic!("Expected command error: {:?}", output);
        }

        assert!(
            output.stderr.contains($err_val),
            "correct error message expected. Got: {}", output.stderr
        );
    };
}
