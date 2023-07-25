mod commands;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    match commands::matcher().subcommand() {
        Some(("authority", matches)) => match matches.subcommand() {
            Some(("get", matches)) => commands::get_authority(matches).await?,
            Some(("set", matches)) => commands::set_authority(matches).await?,
            _ => unreachable!(),
        },
        Some(("configuration", matches)) => match matches.subcommand() {
            Some(("get-stake", matches)) => commands::get_stake_configuration(matches).await?,
            Some(("update-stake", matches)) => {
                commands::update_stake_configuration(matches).await?
            }
            Some(("get-reward", matches)) => commands::get_reward_configuration(matches).await?,
            Some(("update-reward", matches)) => {
                commands::update_reward_configuration(matches).await?
            }
            _ => unreachable!(),
        },
        Some(("reporter", matches)) => match matches.subcommand() {
            Some(("create", matches)) => commands::create_reporter(matches).await?,
            Some(("update", matches)) => commands::update_reporter(matches).await?,
            Some(("get", matches)) => commands::get_reporter(matches).await?,
            Some(("count", matches)) => commands::get_reporter_count(matches).await?,
            Some(("list", matches)) => commands::get_reporters(matches).await?,
            Some(("activate", matches)) => commands::activate_reporter(matches).await?,
            Some(("deactivate", matches)) => commands::deactivate_reporter(matches).await?,
            Some(("unstake", matches)) => commands::unstake_reporter(matches).await?,
            _ => unreachable!(),
        },
        Some(("case", matches)) => match matches.subcommand() {
            Some(("create", matches)) => commands::create_case(matches).await?,
            Some(("update", matches)) => commands::update_case(matches).await?,
            Some(("get", matches)) => commands::get_case(matches).await?,
            Some(("count", matches)) => commands::get_case_count(matches).await?,
            Some(("list", matches)) => commands::get_cases(matches).await?,
            _ => unreachable!(),
        },
        Some(("address", matches)) => match matches.subcommand() {
            Some(("create", matches)) => commands::create_address(matches).await?,
            Some(("update", matches)) => commands::update_address(matches).await?,
            Some(("get", matches)) => commands::get_address(matches).await?,
            Some(("count", matches)) => commands::get_address_count(matches).await?,
            Some(("list", matches)) => commands::get_addresses(matches).await?,
            _ => unreachable!(),
        },
        Some(("asset", matches)) => match matches.subcommand() {
            Some(("create", matches)) => commands::create_asset(matches).await?,
            Some(("update", matches)) => commands::update_asset(matches).await?,
            Some(("get", matches)) => commands::get_asset(matches).await?,
            Some(("count", matches)) => commands::get_asset_count(matches).await?,
            Some(("list", matches)) => commands::get_assets(matches).await?,
            _ => unreachable!(),
        },
        Some(("token", matches)) => match matches.subcommand() {
            Some(("transfer", matches)) => commands::transfer_token(matches).await?,
            Some(("approve", matches)) => commands::approve_token(matches).await?,
            Some(("balance", matches)) => commands::balance_token(matches).await?,
            _ => unreachable!(),
        },
        _ => unreachable!(),
    };

    Ok(())
}
