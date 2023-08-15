use std::collections::HashMap;

use anchor_client::solana_sdk::signature::{read_keypair_file, Keypair};

pub const NETWORK: &str = "solana";

pub const PROGRAM_DIR: &str = "../solana";
pub const KEYS_DIR: &str = "tests/solana/keys";

// pub const HAPI_CORE_PROGRAM_ID: &str = "FgE5ySSi6fbnfYGGRyaeW8y6p8A5KybXPyQ2DdxPCNRk";
pub const HAPI_CORE_KEYPAIR: &str = "tests/test_keypair.json";

// pub const MINT_PUBKEY: &str = "WN4cDdcxEEzCVyaFEuG4zzJB6QNqrahtfYpSeeecrmC";
pub const MINT_KEYPAIR: &str = "token.json";

// pub const ADMIN_PUBKEY: &str = "QDWdYo5JWQ96cCEgdBXpL6TVs5whScFSzVbZgobHLrQ";
pub const ADMIN_KEYPAIR: &str = "wallet_1.json";

// pub const AUTHORITY_PUBKEY: &str = "C7DNJUKfDVpL9ZZqLnVTG1adj4Yu46JgDB6hiTdMEktX";
pub const AUTHORITY_KEYPAIR: &str = "wallet_2.json";

// pub const PUBLISHER_PUBKEY: &str = "5L6h3A2TgUF7DuUky55cCkVdBY9Dvd7rjELVD23reoKk";
pub const PUBLISHER_KEYPAIR: &str = "wallet_3.json";

pub const CASE_UUID_1: &str = "34b9d809-7511-46c5-9117-c7f80f379fad";
pub const CASE_NAME_1: &str = "HAPI Case 1";
pub const CASE_URL_1: &str = "https://hapi.one/case/1";

pub const ADDRESS_ADDR_1: &str = "8aqiaHSdGHcwnJQPJo95JqB2hPv4vzfuwc2zgAYHTWXz";
pub const ADDRESS_RISK_1: &str = "5";
pub const ADDRESS_CATEGORY_1: &str = "ransomware";

pub const ASSET_ADDR_1: &str = "6t4vnZsH5X8zcGm5Z5ZzY6TqHrsPtJcjWcNzL892XP37";
pub const ASSET_ID_1: &str = "1";
pub const ASSET_RISK_1: &str = "7";
pub const ASSET_CATEGORY_1: &str = "counterfeit";

pub struct TestWallet {
    pub keypair: Keypair,
    pub path: String,
}

pub struct TestData {
    pub wallets: HashMap<&'static str, TestWallet>,
    pub program_dir: String,
    pub program_keypair_path: String,
}

impl Default for TestData {
    fn default() -> Self {
        let dir = std::env::current_dir()
            .expect("Unable to detect directory")
            .to_string_lossy()
            .to_string();

        let mut wallets = HashMap::new();

        let mint_path = format!("{}/{}/{}", dir, KEYS_DIR, MINT_KEYPAIR);
        wallets.insert(
            "mint",
            TestWallet {
                keypair: read_keypair_file(&mint_path).unwrap(),
                path: mint_path.clone(),
            },
        );

        let admin_path = format!("{}/{}/{}", dir, KEYS_DIR, ADMIN_KEYPAIR);
        wallets.insert(
            "admin",
            TestWallet {
                keypair: read_keypair_file(&admin_path).unwrap(),
                path: admin_path.clone(),
            },
        );

        let authority_path = format!("{}/{}/{}", dir, KEYS_DIR, AUTHORITY_KEYPAIR);
        wallets.insert(
            "authority",
            TestWallet {
                keypair: read_keypair_file(&authority_path).unwrap(),
                path: authority_path.clone(),
            },
        );

        let publisher_path = format!("{}/{}/{}", dir, KEYS_DIR, PUBLISHER_KEYPAIR);
        wallets.insert(
            "publisher",
            TestWallet {
                keypair: read_keypair_file(&publisher_path).unwrap(),
                path: publisher_path.clone(),
            },
        );

        let program_keypair_path = format!("{}/{}/{}", dir, PROGRAM_DIR, HAPI_CORE_KEYPAIR);
        let program_dir = format!("{}/{}", dir, PROGRAM_DIR);

        Self {
            wallets,
            program_keypair_path,
            program_dir,
        }
    }
}

impl TestData {
    pub fn get_wallet(&self, name: &str) -> &TestWallet {
        self.wallets.get(name).expect("Wallet not found")
    }
}
