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
macro_rules! sort_assert_json_output {
    ($output:expr, $json_val:expr, $key: literal) => {{
        let output = $output.unwrap_or_else(|e| panic!("{}", e));

        if !output.success {
            panic!("Expected command success: {:?}", output);
        }

        let result =
            serde_json::from_str::<serde_json::Value>(&output.stdout).expect("json parse error");

        let mut items = result
            .get($key)
            .expect("Invalid token")
            .as_array()
            .expect("Empty json")
            .iter()
            .map(|v| v.to_string())
            .collect::<Vec<String>>();

        let mut val_items = $json_val
            .get($key)
            .expect("Invalid token")
            .as_array()
            .expect("Empty json")
            .iter()
            .map(|v| v.to_string())
            .collect::<Vec<String>>();

        items.sort();
        val_items.sort();

        assert_eq!(items, val_items, "correct json value expected");

        result
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

        assert!(Setup::is_tx_match(&value), "transaction hash expected");

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
