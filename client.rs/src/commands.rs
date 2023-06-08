use anyhow::anyhow;
use clap::ArgMatches;

use hapi_core::client::configuration::{RewardConfiguration, StakeConfiguration};

use crate::Context;

pub async fn get_authority(args: &ArgMatches) -> anyhow::Result<()> {
    let context = Context::try_from(args)?;

    let authority = context.hapi_core.get_authority().await?;

    println!("{}", authority);

    Ok(())
}

pub async fn set_authority(args: &ArgMatches) -> anyhow::Result<()> {
    let context = Context::try_from(args)?;

    let authority = args
        .get_one::<String>("authority")
        .expect("`authority` is required");

    let tx = context.hapi_core.set_authority(authority).await?;

    println!("{}", tx.hash);

    Ok(())
}

pub async fn update_stake_configuration(args: &ArgMatches) -> anyhow::Result<()> {
    let context = Context::try_from(args)?;

    // TODO: validate address depending on network
    let token = args
        .get_one::<String>("token")
        .ok_or(anyhow!("`token` is required"))?
        .to_string();

    let unlock_duration = args
        .get_one::<String>("unlock-duration")
        .ok_or(anyhow!("`unlock-duration` is required"))?
        .parse()
        .map_err(|e| anyhow!("`unlock-duration`: {e}"))?;

    let validator_stake = args
        .get_one::<String>("validator-stake")
        .ok_or(anyhow!("`validator-stake` is required"))?
        .parse()
        .map_err(|e| anyhow!("`validator-stake`: {e}"))?;

    let tracer_stake = args
        .get_one::<String>("tracer-stake")
        .ok_or(anyhow!("`tracer-stake` is required"))?
        .parse()
        .map_err(|e| anyhow!("`tracer-stake`: {e}"))?;

    let publisher_stake = args
        .get_one::<String>("publisher-stake")
        .ok_or(anyhow!("`publisher-stake` is required"))?
        .parse()
        .map_err(|e| anyhow!("`publisher-stake`: {e}"))?;

    let authority_stake = args
        .get_one::<String>("authority-stake")
        .ok_or(anyhow!("`authority-stake` is required"))?
        .parse()
        .map_err(|e| anyhow!("`authority-stake`: {e}"))?;

    let cfg = StakeConfiguration {
        token,
        unlock_duration,
        validator_stake,
        tracer_stake,
        publisher_stake,
        authority_stake,
    };

    let tx = context.hapi_core.update_stake_configuration(cfg).await?;

    println!("{}", tx.hash);

    Ok(())
}

pub async fn get_stake_configuration(args: &ArgMatches) -> anyhow::Result<()> {
    let context = Context::try_from(args)?;

    println!("{:#?}", context.hapi_core.get_stake_configuration().await?);

    Ok(())
}

pub async fn update_reward_configuration(args: &ArgMatches) -> anyhow::Result<()> {
    let context = Context::try_from(args)?;

    // TODO: validate address depending on network
    let token = args
        .get_one::<String>("token")
        .ok_or(anyhow!("`token` is required"))?
        .to_string();

    let address_confirmation_reward = args
        .get_one::<String>("address-confirmation-reward")
        .ok_or(anyhow!("`address-confirmation-reward` is required"))?
        .parse()
        .map_err(|e| anyhow!("`address-confirmation-reward`: {e}"))?;

    let tracer_reward = args
        .get_one::<String>("tracer-reward")
        .ok_or(anyhow!("`tracer-reward` is required"))?
        .parse()
        .map_err(|e| anyhow!("`tracer-reward`: {e}"))?;

    let cfg = RewardConfiguration {
        token,
        address_confirmation_reward,
        tracer_reward,
    };

    let tx = context.hapi_core.update_reward_configuration(cfg).await?;

    println!("{}", tx.hash);

    Ok(())
}

pub async fn get_reward_configuration(args: &ArgMatches) -> anyhow::Result<()> {
    let context = Context::try_from(args)?;

    println!("{:#?}", context.hapi_core.get_reward_configuration().await?);

    Ok(())
}

pub async fn get_reporters(args: &ArgMatches) -> anyhow::Result<()> {
    let context = Context::try_from(args)?;

    let skip = args
        .get_one::<String>("skip")
        .ok_or(anyhow!("`skip` is required"))?
        .parse()
        .map_err(|e| anyhow!("`skip`: {e}"))?;

    let take = args
        .get_one::<String>("take")
        .ok_or(anyhow!("`take` is required"))?
        .parse()
        .map_err(|e| anyhow!("`take`: {e}"))?;

    let reporters = context.hapi_core.get_reporters(skip, take).await?;

    println!("{:#?}", reporters);

    Ok(())
}

pub async fn get_reporter(args: &ArgMatches) -> anyhow::Result<()> {
    let context = Context::try_from(args)?;

    let reporter_id = args
        .get_one::<String>("reporter-id")
        .ok_or(anyhow!("`reporter-id` is required"))?;

    let reporter = context.hapi_core.get_reporter(reporter_id).await?;

    println!("{:#?}", reporter);

    Ok(())
}

pub async fn get_reporter_count(args: &ArgMatches) -> anyhow::Result<()> {
    let context = Context::try_from(args)?;

    let count = context.hapi_core.get_reporter_count().await?;

    println!("{}", count);

    Ok(())
}

pub async fn create_reporter(args: &ArgMatches) -> anyhow::Result<()> {
    let _context = Context::try_from(args)?;

    unimplemented!()
}

pub async fn update_reporter(args: &ArgMatches) -> anyhow::Result<()> {
    let _context = Context::try_from(args)?;

    unimplemented!()
}

pub async fn activate_reporter(args: &ArgMatches) -> anyhow::Result<()> {
    let _context = Context::try_from(args)?;

    unimplemented!()
}

pub async fn deactivate_reporter(args: &ArgMatches) -> anyhow::Result<()> {
    let _context = Context::try_from(args)?;

    unimplemented!()
}

pub async fn unstake_reporter(args: &ArgMatches) -> anyhow::Result<()> {
    let _context = Context::try_from(args)?;

    unimplemented!()
}

pub async fn create_case(args: &ArgMatches) -> anyhow::Result<()> {
    let _context = Context::try_from(args)?;

    unimplemented!()
}

pub async fn update_case(args: &ArgMatches) -> anyhow::Result<()> {
    let _context = Context::try_from(args)?;

    unimplemented!()
}

pub async fn get_case(args: &ArgMatches) -> anyhow::Result<()> {
    let _context = Context::try_from(args)?;

    unimplemented!()
}

pub async fn get_case_count(args: &ArgMatches) -> anyhow::Result<()> {
    let _context = Context::try_from(args)?;

    unimplemented!()
}

pub async fn get_cases(args: &ArgMatches) -> anyhow::Result<()> {
    let _context = Context::try_from(args)?;

    unimplemented!()
}

pub async fn create_address(args: &ArgMatches) -> anyhow::Result<()> {
    let _context = Context::try_from(args)?;

    unimplemented!()
}

pub async fn update_address(args: &ArgMatches) -> anyhow::Result<()> {
    let _context = Context::try_from(args)?;

    unimplemented!()
}

pub async fn get_address(args: &ArgMatches) -> anyhow::Result<()> {
    let _context = Context::try_from(args)?;

    unimplemented!()
}

pub async fn get_address_count(args: &ArgMatches) -> anyhow::Result<()> {
    let _context = Context::try_from(args)?;

    unimplemented!()
}

pub async fn get_addresses(args: &ArgMatches) -> anyhow::Result<()> {
    let _context = Context::try_from(args)?;

    unimplemented!()
}

pub async fn create_asset(args: &ArgMatches) -> anyhow::Result<()> {
    let _context = Context::try_from(args)?;

    unimplemented!()
}

pub async fn update_asset(args: &ArgMatches) -> anyhow::Result<()> {
    let _context = Context::try_from(args)?;

    unimplemented!()
}

pub async fn get_asset(args: &ArgMatches) -> anyhow::Result<()> {
    let _context = Context::try_from(args)?;

    unimplemented!()
}

pub async fn get_asset_count(args: &ArgMatches) -> anyhow::Result<()> {
    let _context = Context::try_from(args)?;

    unimplemented!()
}

pub async fn get_assets(args: &ArgMatches) -> anyhow::Result<()> {
    let _context = Context::try_from(args)?;

    unimplemented!()
}
