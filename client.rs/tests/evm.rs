use regex::Regex;

mod setup;
use setup::{CmdOutput, Setup, PRIVATE_KEY_2, PUBLIC_KEY_1, PUBLIC_KEY_2};

#[tokio::test]
async fn it_works() {
    let t = Setup::new();

    // Check that initial authority matches the key of contract deployer
    let CmdOutput { stdout, .. } = t
        .exec(["authority", "get"])
        .unwrap_or_else(|e| panic!("{}", e));
    assert_eq!(stdout.trim(), PUBLIC_KEY_1);

    // Assign authority to a new address
    let CmdOutput { stdout, .. } = t
        .exec(["authority", "set", PUBLIC_KEY_2])
        .unwrap_or_else(|e| panic!("{}", e));
    assert!(
        Regex::new(r"^0x[0-9a-fA-F]{64}$")
            .unwrap()
            .is_match(stdout.trim()),
        "expected a transaction hash"
    );

    // Make sure that authority has changed
    let CmdOutput { stdout, .. } = t
        .exec(["authority", "get"])
        .unwrap_or_else(|e| panic!("{}", e));
    assert_eq!(stdout.trim(), PUBLIC_KEY_2);

    // Use the private key of the new authority to change the authority back
    let CmdOutput { stdout, .. } = t
        .exec([
            "authority",
            "set",
            PUBLIC_KEY_1,
            "--private-key",
            PRIVATE_KEY_2,
        ])
        .unwrap_or_else(|e| panic!("{}", e));
    assert!(
        Regex::new(r"^0x[0-9a-fA-F]{64}$")
            .unwrap()
            .is_match(stdout.trim()),
        "expected a transaction hash"
    );

    // Make sure that authority has changed back
    let CmdOutput { stdout, .. } = t
        .exec(["authority", "get"])
        .unwrap_or_else(|e| panic!("{e}"));
    assert_eq!(stdout.trim(), PUBLIC_KEY_1);

    // Check that initial configuration is empty
    let CmdOutput {
        success, stderr, ..
    } = t
        .exec(["configuration", "get-stake"])
        .unwrap_or_else(|e| panic!("{e}"));

    assert!(!success);
    assert_eq!(stderr, "Error: Ethers error: `get_stake_configuration` reverted with: Stake configuration is not set");

    // Update stake configuration
    let CmdOutput { stdout, .. } = t
        .exec([
            "configuration",
            "update-stake",
            &t.token,
            "600",
            "10",
            "11",
            "12",
            "13",
        ])
        .unwrap_or_else(|e| panic!("{e}"));
    assert!(
        Regex::new(r"^0x[0-9a-fA-F]{64}$")
            .unwrap()
            .is_match(stdout.trim()),
        "expected a transaction hash"
    );

    // Make sure that the new stake configuration is applied
    let CmdOutput { stdout, .. } = t
        .exec(["configuration", "get-stake"])
        .unwrap_or_else(|e| panic!("{e}"));
    assert_eq!(stdout, "StakeConfiguration {\n    token: \"0x5fbd…0aa3\",\n    unlock_duration: 600,\n    validator_stake: Amount(\n        10,\n    ),\n    tracer_stake: Amount(\n        11,\n    ),\n    publisher_stake: Amount(\n        12,\n    ),\n    authority_stake: Amount(\n        13,\n    ),\n}");
}
