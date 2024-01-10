use clap::{Arg, ArgGroup, ArgMatches, Command};
use std::process::exit;

pub(crate) fn matcher() -> ArgMatches {
    Command::new(env!("CARGO_CRATE_NAME"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .version(env!("CARGO_PKG_VERSION"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .arg(
            Arg::new("network")
                .global(true)
                .long("network")
                .short('n')
                .value_name("NETWORK")
                .env("NETWORK")
                .value_parser(["ethereum", "bsc", "solana", "bitcoin", "near"])
                .help("Network to use"),
        )
        .arg(
            Arg::new("provider-url")
                .global(true)
                .long("provider-url")
                .short('p')
                .value_name("PROVIDER_URL")
                .env("PROVIDER_URL")
                .help("Network-specific provider URL (e.g. RPC node URL)"),
        )
        .arg(
            Arg::new("contract-address")
                .global(true)
                .long("contract-address")
                .short('c')
                .value_name("CONTRACT_ADDRESS")
                .env("CONTRACT_ADDRESS")
                .help("Network-specific HAPI Core contract address"),
        )
        .arg(
            Arg::new("private-key")
                .global(true)
                .long("private-key")
                .short('k')
                .value_name("PRIVATE_KEY")
                .env("PRIVATE_KEY")
                .hide_env(true)
                .help("Private key to sign transactions"),
        )
        .arg(
            Arg::new("chain-id")
                .global(true)
                .long("chain-id")
                .value_name("CHAIN_ID")
                .env("CHAIN_ID")
                .required(false)
                .help("[OPTIONAL] Chain ID for EVM-based networks"),
        )
        .arg(
            Arg::new("account-id")
                .global(true)
                .long("account-id")
                .value_name("ACCOUNT_ID")
                .env("ACCOUNT_ID")
                .required(false)
                .help("[OPTIONAL] Account ID for NEAR network"),
        )
        .arg(
            Arg::new("output")
                .global(true)
                .short('o')
                .long("output")
                .value_name("OUTPUT")
                .env("OUTPUT")
                .value_parser(["json", "text"])
                .help("[OPTIONAL] Command output format"),
        )
        .subcommand_required(true)
        .subcommand(
            Command::new("authority")
                .about("Authority commands")
                .subcommand_required(true)
                .subcommand(Command::new("get").about("Get authority address"))
                .subcommand(
                    Command::new("set")
                        .arg(
                            Arg::new("authority")
                                .value_name("AUTHORITY")
                                .index(1)
                                .required(true)
                                .help("New authority address"),
                        )
                        .about("Set new authority address"),
                ),
        )
        .subcommand(
            Command::new("configuration")
                .alias("cfg")
                .about("Configuration commands")
                .subcommand_required(true)
                .subcommand(Command::new("get-stake").about("Get stake configuration"))
                .subcommand(
                    Command::new("update-stake")
                        .about("Update stake configuration")
                        .arg(
                            Arg::new("token")
                                .value_name("TOKEN")
                                .index(1)
                                .required(true)
                                .help("Token address"),
                        )
                        .arg(
                            Arg::new("unlock-duration")
                                .value_name("UNLOCK_DURATION")
                                .index(2)
                                .required(true)
                                .help("Unlock duration"),
                        )
                        .arg(
                            Arg::new("validator-stake")
                                .value_name("VALIDATOR_STAKE")
                                .index(3)
                                .required(true)
                                .help("Validator stake"),
                        )
                        .arg(
                            Arg::new("tracer-stake")
                                .value_name("TRACER_STAKE")
                                .index(4)
                                .required(true)
                                .help("Tracer stake"),
                        )
                        .arg(
                            Arg::new("publisher-stake")
                                .value_name("PUBLISHER_STAKE")
                                .index(5)
                                .required(true)
                                .help("Publisher stake"),
                        )
                        .arg(
                            Arg::new("authority-stake")
                                .value_name("AUTHORITY_STAKE")
                                .index(6)
                                .required(true)
                                .help("Authority stake"),
                        ),
                )
                .subcommand(Command::new("get-reward").about("Get reward configuration"))
                .subcommand(
                    Command::new("update-reward")
                        .about("Update reward configuration")
                        .arg(
                            Arg::new("token")
                                .value_name("TOKEN")
                                .index(1)
                                .required(true)
                                .help("Token address"),
                        )
                        .arg(
                            Arg::new("address-confirmation-reward")
                                .value_name("ADDRESS_CONFIRMATION_REWARD")
                                .index(2)
                                .required(true)
                                .help("Address confirmation reward"),
                        )
                        .arg(
                            Arg::new("address-tracer-reward")
                                .value_name("ADDRESS_TRACER_REWARD")
                                .index(3)
                                .required(true)
                                .help("Address tracer reward"),
                        )
                        .arg(
                            Arg::new("asset-confirmation-reward")
                                .value_name("ASSET_CONFIRMATION_REWARD")
                                .index(4)
                                .required(true)
                                .help("Asset confirmation reward"),
                        )
                        .arg(
                            Arg::new("asset-tracer-reward")
                                .value_name("ASSET_TRACER_REWARD")
                                .index(5)
                                .required(true)
                                .help("Asset tracer reward"),
                        ),
                ),
        )
        .subcommand(
            Command::new("reporter")
                .about("Reporter commands")
                .subcommand_required(true)
                .subcommand(
                    Command::new("create")
                        .about("Create reporter")
                        .arg(
                            Arg::new("id")
                                .value_name("ID")
                                .index(1)
                                .required(true)
                                .help("Reporter UUID"),
                        )
                        .arg(
                            Arg::new("account")
                                .value_name("ACCOUNT")
                                .index(2)
                                .required(true)
                                .help("Reporter account address"),
                        )
                        .arg(
                            Arg::new("role")
                                .value_name("ROLE")
                                .index(3)
                                .required(true)
                                .help("Reporter role")
                                .value_parser([
                                    "Validator",
                                    "Tracer",
                                    "Publisher",
                                    "Authority",
                                    "validator",
                                    "tracer",
                                    "publisher",
                                    "authority",
                                ]),
                        )
                        .arg(
                            Arg::new("name")
                                .value_name("NAME")
                                .index(4)
                                .required(true)
                                .help("Reporter display name"),
                        )
                        .arg(
                            Arg::new("url")
                                .value_name("URL")
                                .index(5)
                                .required(true)
                                .help("Reporter URL"),
                        ),
                )
                .subcommand(
                    Command::new("update")
                        .about("Update reporter")
                        .arg(
                            Arg::new("id")
                                .value_name("ID")
                                .index(1)
                                .required(true)
                                .help("Reporter UUID"),
                        )
                        .arg(
                            Arg::new("account")
                                .value_name("ACCOUNT")
                                .index(2)
                                .required(true)
                                .help("Reporter account address"),
                        )
                        .arg(
                            Arg::new("role")
                                .value_name("ROLE")
                                .index(3)
                                .required(true)
                                .help("Reporter role")
                                .value_parser([
                                    "Validator",
                                    "Tracer",
                                    "Publisher",
                                    "Authority",
                                    "validator",
                                    "tracer",
                                    "publisher",
                                    "authority",
                                ]),
                        )
                        .arg(
                            Arg::new("name")
                                .value_name("NAME")
                                .index(4)
                                .required(true)
                                .help("Reporter display name"),
                        )
                        .arg(
                            Arg::new("url")
                                .value_name("URL")
                                .index(5)
                                .required(true)
                                .help("Reporter URL"),
                        ),
                )
                .subcommand(
                    Command::new("get").about("Get reporter").arg(
                        Arg::new("id")
                            .value_name("ID")
                            .index(1)
                            .required(true)
                            .help("Reporter UUID"),
                    ),
                )
                .subcommand(Command::new("count").about("Get reporter count"))
                .subcommand(
                    Command::new("list")
                        .about("Get reporter list")
                        .arg(
                            Arg::new("skip")
                                .group("pagination")
                                .long("skip")
                                .short('s')
                                .value_name("SKIP")
                                .default_value("0")
                                .help("Skip first N items"),
                        )
                        .arg(
                            Arg::new("take")
                                .group("pagination")
                                .long("take")
                                .short('t')
                                .value_name("TAKE")
                                .default_value("10")
                                .help("Return N items"),
                        ),
                )
                .subcommand(Command::new("activate").about("Activate reporter"))
                .subcommand(Command::new("deactivate").about("Deactivate reporter"))
                .subcommand(Command::new("unstake").about("Unstake reporter")),
        )
        .subcommand(
            Command::new("case")
                .about("Case commands")
                .subcommand_required(true)
                .subcommand(
                    Command::new("create")
                        .about("Create case")
                        .arg(
                            Arg::new("id")
                                .value_name("ID")
                                .index(1)
                                .required(true)
                                .help("Case UUID"),
                        )
                        .arg(
                            Arg::new("name")
                                .value_name("NAME")
                                .index(2)
                                .required(true)
                                .help("Case display name"),
                        )
                        .arg(
                            Arg::new("url")
                                .value_name("URL")
                                .index(3)
                                .required(true)
                                .help("Case URL"),
                        ),
                )
                .subcommand(
                    Command::new("update")
                        .about("Update case")
                        .arg(
                            Arg::new("id")
                                .value_name("ID")
                                .index(1)
                                .required(true)
                                .help("Case UUID"),
                        )
                        .arg(
                            Arg::new("name")
                                .value_name("NAME")
                                .index(2)
                                .required(true)
                                .help("Case display name"),
                        )
                        .arg(
                            Arg::new("url")
                                .value_name("URL")
                                .index(3)
                                .required(true)
                                .help("Case URL"),
                        )
                        .arg(
                            Arg::new("status")
                                .value_name("STATUS")
                                .index(4)
                                .required(true)
                                .help("Case status")
                                .value_parser(["Closed", "Open", "closed", "open"]),
                        ),
                )
                .subcommand(
                    Command::new("get").about("Get case").arg(
                        Arg::new("id")
                            .value_name("ID")
                            .index(1)
                            .required(true)
                            .help("Case UUID"),
                    ),
                )
                .subcommand(Command::new("count").about("Get case count"))
                .subcommand(
                    Command::new("list")
                        .about("Get case list")
                        .arg(
                            Arg::new("skip")
                                .group("pagination")
                                .long("skip")
                                .short('s')
                                .value_name("SKIP")
                                .default_value("0")
                                .help("Skip first N items"),
                        )
                        .arg(
                            Arg::new("take")
                                .group("pagination")
                                .long("take")
                                .short('t')
                                .value_name("TAKE")
                                .default_value("10")
                                .help("Return N items"),
                        ),
                ),
        )
        .subcommand(
            Command::new("address")
                .about("Address commands")
                .subcommand_required(true)
                .subcommand(
                    Command::new("create")
                        .about("Create address")
                        .arg(
                            Arg::new("address")
                                .value_name("ADDRESS")
                                .index(1)
                                .required(true)
                                .help("Address"),
                        )
                        .arg(
                            Arg::new("case-id")
                                .value_name("CASE_ID")
                                .index(2)
                                .required(true)
                                .help("Case UUID"),
                        )
                        .arg(
                            Arg::new("category")
                                .value_name("CATEGORY")
                                .index(3)
                                .required(true)
                                .help("Category")
                                .value_parser([
                                    "None",
                                    "WalletService",
                                    "MerchantService",
                                    "MiningPool",
                                    "Exchange",
                                    "DeFi",
                                    "OTCBroker",
                                    "ATM",
                                    "Gambling",
                                    "IllicitOrganization",
                                    "Mixer",
                                    "DarknetService",
                                    "Scam",
                                    "Ransomware",
                                    "Theft",
                                    "Counterfeit",
                                    "TerroristFinancing",
                                    "Sanctions",
                                    "ChildAbuse",
                                    "Hacker",
                                    "HighRiskJurisdiction",
                                    "none",
                                    "wallet_service",
                                    "merchant_service",
                                    "mining_pool",
                                    "exchange",
                                    "defi",
                                    "otc_broker",
                                    "atm",
                                    "gambling",
                                    "illicit_organization",
                                    "mixer",
                                    "darknet_service",
                                    "scam",
                                    "ransomware",
                                    "theft",
                                    "counterfeit",
                                    "terrorist_financing",
                                    "sanctions",
                                    "child_abuse",
                                    "hacker",
                                    "high_risk_jurisdiction",
                                ]),
                        )
                        .arg(
                            Arg::new("risk")
                                .value_name("RISK")
                                .index(4)
                                .required(true)
                                .help("Risk score (0..10)")
                                .value_parser(risk_parser),
                        ),
                )
                .subcommand(
                    Command::new("update")
                        .about("Update address")
                        .arg(
                            Arg::new("address")
                                .value_name("ADDRESS")
                                .index(1)
                                .required(true)
                                .help("Address"),
                        )
                        .arg(
                            Arg::new("case-id")
                                .value_name("CASE_ID")
                                .index(2)
                                .required(true)
                                .help("Case UUID"),
                        )
                        .arg(
                            Arg::new("category")
                                .value_name("CATEGORY")
                                .index(3)
                                .required(true)
                                .help("Category")
                                .value_parser([
                                    "None",
                                    "WalletService",
                                    "MerchantService",
                                    "MiningPool",
                                    "Exchange",
                                    "DeFi",
                                    "OTCBroker",
                                    "ATM",
                                    "Gambling",
                                    "IllicitOrganization",
                                    "Mixer",
                                    "DarknetService",
                                    "Scam",
                                    "Ransomware",
                                    "Theft",
                                    "Counterfeit",
                                    "TerroristFinancing",
                                    "Sanctions",
                                    "ChildAbuse",
                                    "Hacker",
                                    "HighRiskJurisdiction",
                                    "none",
                                    "wallet_service",
                                    "merchant_service",
                                    "mining_pool",
                                    "exchange",
                                    "defi",
                                    "otc_broker",
                                    "atm",
                                    "gambling",
                                    "illicit_organization",
                                    "mixer",
                                    "darknet_service",
                                    "scam",
                                    "ransomware",
                                    "theft",
                                    "counterfeit",
                                    "terrorist_financing",
                                    "sanctions",
                                    "child_abuse",
                                    "hacker",
                                    "high_risk_jurisdiction",
                                ]),
                        )
                        .arg(
                            Arg::new("risk")
                                .value_name("RISK")
                                .index(4)
                                .required(true)
                                .help("Risk score (0..10)")
                                .value_parser(risk_parser),
                        ),
                )
                .subcommand(
                    Command::new("confirm").about("Confirm address").arg(
                        Arg::new("address")
                            .value_name("ADDRESS")
                            .index(1)
                            .required(true)
                            .help("Address"),
                    ),
                )
                .subcommand(
                    Command::new("get").about("Get address").arg(
                        Arg::new("address")
                            .value_name("ADDRESS")
                            .index(1)
                            .required(true)
                            .help("Address"),
                    ),
                )
                .subcommand(Command::new("count").about("Get address count"))
                .subcommand(
                    Command::new("list")
                        .about("Get address list")
                        .group(ArgGroup::new("pagination").args(["skip", "take"]))
                        .arg(
                            Arg::new("skip")
                                .group("pagination")
                                .long("skip")
                                .short('s')
                                .value_name("SKIP")
                                .default_value("0")
                                .help("Skip first N items"),
                        )
                        .arg(
                            Arg::new("take")
                                .group("pagination")
                                .long("take")
                                .short('t')
                                .value_name("TAKE")
                                .default_value("10")
                                .help("Return N items"),
                        ),
                ),
        )
        .subcommand(
            Command::new("asset")
                .about("Asset commands")
                .subcommand_required(true)
                .subcommand(
                    Command::new("create")
                        .about("Create asset")
                        .arg(
                            Arg::new("address")
                                .value_name("ADDRESS")
                                .index(1)
                                .required(true)
                                .help("Asset contract address"),
                        )
                        .arg(
                            Arg::new("asset-id")
                                .value_name("ASSET_ID")
                                .index(2)
                                .required(true)
                                .help("Asset ID"),
                        )
                        .arg(
                            Arg::new("case-id")
                                .value_name("CASE_ID")
                                .index(3)
                                .required(true)
                                .help("Case UUID"),
                        )
                        .arg(
                            Arg::new("category")
                                .value_name("CATEGORY")
                                .index(4)
                                .required(true)
                                .help("Category")
                                .value_parser([
                                    "None",
                                    "WalletService",
                                    "MerchantService",
                                    "MiningPool",
                                    "Exchange",
                                    "DeFi",
                                    "OTCBroker",
                                    "ATM",
                                    "Gambling",
                                    "IllicitOrganization",
                                    "Mixer",
                                    "DarknetService",
                                    "Scam",
                                    "Ransomware",
                                    "Theft",
                                    "Counterfeit",
                                    "TerroristFinancing",
                                    "Sanctions",
                                    "ChildAbuse",
                                    "Hacker",
                                    "HighRiskJurisdiction",
                                    "none",
                                    "wallet_service",
                                    "merchant_service",
                                    "mining_pool",
                                    "exchange",
                                    "defi",
                                    "otc_broker",
                                    "atm",
                                    "gambling",
                                    "illicit_organization",
                                    "mixer",
                                    "darknet_service",
                                    "scam",
                                    "ransomware",
                                    "theft",
                                    "counterfeit",
                                    "terrorist_financing",
                                    "sanctions",
                                    "child_abuse",
                                    "hacker",
                                    "high_risk_jurisdiction",
                                ]),
                        )
                        .arg(
                            Arg::new("risk")
                                .value_name("RISK")
                                .index(5)
                                .required(true)
                                .help("Risk score (0..10)")
                                .value_parser(risk_parser),
                        ),
                )
                .subcommand(
                    Command::new("update")
                        .about("Update asset")
                        .arg(
                            Arg::new("address")
                                .value_name("ADDRESS")
                                .index(1)
                                .required(true)
                                .help("Asset contract address"),
                        )
                        .arg(
                            Arg::new("asset-id")
                                .value_name("ASSET_ID")
                                .index(2)
                                .required(true)
                                .help("Asset ID"),
                        )
                        .arg(
                            Arg::new("case-id")
                                .value_name("CASE_ID")
                                .index(3)
                                .required(true)
                                .help("Case UUID"),
                        )
                        .arg(
                            Arg::new("category")
                                .value_name("CATEGORY")
                                .index(4)
                                .required(true)
                                .help("Category")
                                .value_parser([
                                    "None",
                                    "WalletService",
                                    "MerchantService",
                                    "MiningPool",
                                    "Exchange",
                                    "DeFi",
                                    "OTCBroker",
                                    "ATM",
                                    "Gambling",
                                    "IllicitOrganization",
                                    "Mixer",
                                    "DarknetService",
                                    "Scam",
                                    "Ransomware",
                                    "Theft",
                                    "Counterfeit",
                                    "TerroristFinancing",
                                    "Sanctions",
                                    "ChildAbuse",
                                    "Hacker",
                                    "HighRiskJurisdiction",
                                    "none",
                                    "wallet_service",
                                    "merchant_service",
                                    "mining_pool",
                                    "exchange",
                                    "defi",
                                    "otc_broker",
                                    "atm",
                                    "gambling",
                                    "illicit_organization",
                                    "mixer",
                                    "darknet_service",
                                    "scam",
                                    "ransomware",
                                    "theft",
                                    "counterfeit",
                                    "terrorist_financing",
                                    "sanctions",
                                    "child_abuse",
                                    "hacker",
                                    "high_risk_jurisdiction",
                                ]),
                        )
                        .arg(
                            Arg::new("risk")
                                .value_name("RISK")
                                .index(5)
                                .required(true)
                                .help("Risk score (0..10)")
                                .value_parser(risk_parser),
                        ),
                )
                .subcommand(
                    Command::new("confirm")
                        .about("Confirm asset")
                        .arg(
                            Arg::new("address")
                                .value_name("ADDRESS")
                                .index(1)
                                .required(true)
                                .help("Asset contract address"),
                        )
                        .arg(
                            Arg::new("asset-id")
                                .value_name("ASSET_ID")
                                .index(2)
                                .required(true)
                                .help("Asset ID"),
                        ),
                )
                .subcommand(
                    Command::new("get")
                        .about("Get asset")
                        .arg(
                            Arg::new("address")
                                .value_name("ADDRESS")
                                .index(1)
                                .required(true)
                                .help("Asset contract address"),
                        )
                        .arg(
                            Arg::new("asset-id")
                                .value_name("ASSET_ID")
                                .index(2)
                                .required(true)
                                .help("Asset ID"),
                        ),
                )
                .subcommand(Command::new("count").about("Get asset count"))
                .subcommand(
                    Command::new("list")
                        .about("Get asset list")
                        .arg(
                            Arg::new("skip")
                                .group("pagination")
                                .long("skip")
                                .short('s')
                                .value_name("SKIP")
                                .default_value("0")
                                .help("Skip first N items"),
                        )
                        .arg(
                            Arg::new("take")
                                .group("pagination")
                                .long("take")
                                .short('t')
                                .value_name("TAKE")
                                .default_value("10")
                                .help("Return N items"),
                        ),
                ),
        )
        .subcommand(
            Command::new("token")
                .about("Token operations")
                .subcommand_required(true)
                .subcommand(
                    Command::new("transfer")
                        .about("Transfer token")
                        .arg(
                            Arg::new("token-contract")
                                .value_name("TOKEN_CONTRACT")
                                .index(1)
                                .required(true)
                                .help("Token contract address"),
                        )
                        .arg(
                            Arg::new("to")
                                .value_name("TO")
                                .index(2)
                                .required(true)
                                .help("Receiver address"),
                        )
                        .arg(
                            Arg::new("amount")
                                .value_name("AMOUNT")
                                .index(3)
                                .required(true)
                                .help("Amount to transfer"),
                        ),
                )
                .subcommand(
                    Command::new("approve")
                        .about("Approve token allowance")
                        .arg(
                            Arg::new("token-contract")
                                .value_name("TOKEN_CONTRACT")
                                .index(1)
                                .required(true)
                                .help("Token contract address"),
                        )
                        .arg(
                            Arg::new("spender")
                                .value_name("SPENDER")
                                .index(2)
                                .required(true)
                                .help("Address that receives allowance"),
                        )
                        .arg(
                            Arg::new("amount")
                                .value_name("AMOUNT")
                                .index(3)
                                .required(true)
                                .help("Amount to approve"),
                        ),
                )
                .subcommand(
                    Command::new("balance")
                        .about("Get token balance")
                        .arg(
                            Arg::new("token-contract")
                                .value_name("TOKEN_CONTRACT")
                                .index(1)
                                .required(true)
                                .help("Token contract address"),
                        )
                        .arg(
                            Arg::new("address")
                                .value_name("ADDRESS")
                                .index(2)
                                .required(true)
                                .help("Address to get balance for"),
                        ),
                ),
        )
        .try_get_matches()
        .map_err(|e| {
            eprintln!("{}", e);
            exit(1);
        })
        .expect("Failed to parse CLI arguments")
}

fn risk_parser(val: &str) -> Result<u8, String> {
    match val.parse::<u8>() {
        Ok(val) if val > 10 => Err(format!("Risk must be an integer between 0 and 10: {}", val)),
        Ok(val) => Ok(val),
        Err(err) => Err(format!("Risk must be an integer between 0 and 10: {}", err)),
    }
}
