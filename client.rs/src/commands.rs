use anyhow::anyhow;
use clap::ArgMatches;

use hapi_core::client::{
    address::{CreateAddressInput, UpdateAddressInput},
    asset::{CreateAssetInput, UpdateAssetInput},
    case::{CreateCaseInput, UpdateCaseInput},
    configuration::{RewardConfiguration, StakeConfiguration},
    reporter::{CreateReporterInput, UpdateReporterInput},
};

use crate::CommandContext;

pub async fn get_authority(args: &ArgMatches) -> anyhow::Result<()> {
    let context = CommandContext::try_from(args)?;

    let authority = context.hapi_core.get_authority().await?;

    println!("{}", authority);

    Ok(())
}

pub async fn set_authority(args: &ArgMatches) -> anyhow::Result<()> {
    let context = CommandContext::try_from(args)?;

    let authority = args
        .get_one::<String>("authority")
        .expect("`authority` is required");

    let tx = context.hapi_core.set_authority(authority).await?;

    println!("{}", tx.hash);

    Ok(())
}

pub async fn update_stake_configuration(args: &ArgMatches) -> anyhow::Result<()> {
    let context = CommandContext::try_from(args)?;

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
    let context = CommandContext::try_from(args)?;

    println!("{:#?}", context.hapi_core.get_stake_configuration().await?);

    Ok(())
}

pub async fn update_reward_configuration(args: &ArgMatches) -> anyhow::Result<()> {
    let context = CommandContext::try_from(args)?;

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
    let context = CommandContext::try_from(args)?;

    println!("{:#?}", context.hapi_core.get_reward_configuration().await?);

    Ok(())
}

pub async fn get_reporters(args: &ArgMatches) -> anyhow::Result<()> {
    let context = CommandContext::try_from(args)?;

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
    let context = CommandContext::try_from(args)?;

    let reporter_id = args
        .get_one::<String>("reporter-id")
        .ok_or(anyhow!("`reporter-id` is required"))?;

    let reporter = context.hapi_core.get_reporter(reporter_id).await?;

    println!("{:#?}", reporter);

    Ok(())
}

pub async fn get_reporter_count(args: &ArgMatches) -> anyhow::Result<()> {
    let context = CommandContext::try_from(args)?;

    let count = context.hapi_core.get_reporter_count().await?;

    println!("{}", count);

    Ok(())
}

pub async fn create_reporter(args: &ArgMatches) -> anyhow::Result<()> {
    let context = CommandContext::try_from(args)?;

    let id = args
        .get_one::<String>("id")
        .ok_or(anyhow!("`id` is required"))?
        .parse()
        .map_err(|e| anyhow!("`id`: {e}"))?;

    let account = args
        .get_one::<String>("account")
        .ok_or(anyhow!("`account` is required"))?
        .parse()
        .map_err(|e| anyhow!("`account`: {e}"))?;

    let role = args
        .get_one::<String>("role")
        .ok_or(anyhow!("`role` is required"))?
        .parse()
        .map_err(|e| anyhow!("`role`: {e}"))?;

    let name = args
        .get_one::<String>("name")
        .ok_or(anyhow!("`name` is required"))?
        .parse()
        .map_err(|e| anyhow!("`name`: {e}"))?;

    let url = args
        .get_one::<String>("url")
        .ok_or(anyhow!("`url` is required"))?
        .parse()
        .map_err(|e| anyhow!("`url`: {e}"))?;

    let tx = context
        .hapi_core
        .create_reporter(CreateReporterInput {
            id,
            account,
            role,
            name,
            url,
        })
        .await?;

    println!("{}", tx.hash);

    Ok(())
}

pub async fn update_reporter(args: &ArgMatches) -> anyhow::Result<()> {
    let context = CommandContext::try_from(args)?;

    let id = args
        .get_one::<String>("id")
        .ok_or(anyhow!("`id` is required"))?
        .parse()
        .map_err(|e| anyhow!("`id`: {e}"))?;

    let account = args
        .get_one::<String>("account")
        .ok_or(anyhow!("`account` is required"))?
        .parse()
        .map_err(|e| anyhow!("`account`: {e}"))?;

    let role = args
        .get_one::<String>("role")
        .ok_or(anyhow!("`role` is required"))?
        .parse()
        .map_err(|e| anyhow!("`role`: {e}"))?;

    let name = args
        .get_one::<String>("name")
        .ok_or(anyhow!("`name` is required"))?
        .parse()
        .map_err(|e| anyhow!("`name`: {e}"))?;

    let url = args
        .get_one::<String>("url")
        .ok_or(anyhow!("`url` is required"))?
        .parse()
        .map_err(|e| anyhow!("`url`: {e}"))?;

    let tx = context
        .hapi_core
        .update_reporter(UpdateReporterInput {
            id,
            account,
            role,
            name,
            url,
        })
        .await?;

    println!("{}", tx.hash);

    Ok(())
}

pub async fn activate_reporter(args: &ArgMatches) -> anyhow::Result<()> {
    let context = CommandContext::try_from(args)?;

    let tx = context.hapi_core.activate_reporter().await?;

    println!("{}", tx.hash);

    Ok(())
}

pub async fn deactivate_reporter(args: &ArgMatches) -> anyhow::Result<()> {
    let context = CommandContext::try_from(args)?;

    let tx = context.hapi_core.deactivate_reporter().await?;

    println!("{}", tx.hash);

    Ok(())
}

pub async fn unstake_reporter(args: &ArgMatches) -> anyhow::Result<()> {
    let context = CommandContext::try_from(args)?;

    let tx = context.hapi_core.unstake_reporter().await?;

    println!("{}", tx.hash);

    Ok(())
}

pub async fn create_case(args: &ArgMatches) -> anyhow::Result<()> {
    let context = CommandContext::try_from(args)?;

    let tx = context.hapi_core.create_case(CreateCaseInput {}).await?;

    println!("{}", tx.hash);

    Ok(())
}

pub async fn update_case(args: &ArgMatches) -> anyhow::Result<()> {
    let context = CommandContext::try_from(args)?;

    let tx = context.hapi_core.update_case(UpdateCaseInput {}).await?;

    println!("{}", tx.hash);

    Ok(())
}

pub async fn get_case(args: &ArgMatches) -> anyhow::Result<()> {
    let context = CommandContext::try_from(args)?;

    let case_id = args
        .get_one::<String>("case-id")
        .ok_or(anyhow!("`case-id` is required"))?;

    let case = context.hapi_core.get_case(case_id).await?;

    println!("{:#?}", case);

    Ok(())
}

pub async fn get_case_count(args: &ArgMatches) -> anyhow::Result<()> {
    let context = CommandContext::try_from(args)?;

    let count = context.hapi_core.get_case_count().await?;

    println!("{}", count);

    Ok(())
}

pub async fn get_cases(args: &ArgMatches) -> anyhow::Result<()> {
    let context = CommandContext::try_from(args)?;

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

    let cases = context.hapi_core.get_cases(skip, take).await?;

    println!("{:#?}", cases);

    Ok(())
}

pub async fn create_address(args: &ArgMatches) -> anyhow::Result<()> {
    let context = CommandContext::try_from(args)?;

    let address = args
        .get_one::<String>("address")
        .ok_or(anyhow!("`address` is required"))?
        .to_owned();

    let case_id = args
        .get_one::<String>("case-id")
        .ok_or(anyhow!("`case-id` is required"))?
        .parse()
        .map_err(|e| anyhow!("`case-id`: {e}"))?;

    let risk = args
        .get_one::<String>("risk")
        .ok_or(anyhow!("`risk` is required"))?
        .parse()
        .map_err(|e| anyhow!("`risk`: {e}"))?;

    let category = args
        .get_one::<String>("category")
        .ok_or(anyhow!("`category` is required"))?
        .parse()
        .map_err(|e| anyhow!("`category`: {e}"))?;

    let tx = context
        .hapi_core
        .create_address(CreateAddressInput {
            address,
            case_id,
            risk,
            category,
        })
        .await?;

    println!("{}", tx.hash);

    Ok(())
}

pub async fn update_address(args: &ArgMatches) -> anyhow::Result<()> {
    let context = CommandContext::try_from(args)?;

    let address = args
        .get_one::<String>("address")
        .ok_or(anyhow!("`address` is required"))?
        .to_owned();

    let case_id = args
        .get_one::<String>("case-id")
        .ok_or(anyhow!("`case-id` is required"))?
        .parse()
        .map_err(|e| anyhow!("`case-id`: {e}"))?;

    let risk = args
        .get_one::<String>("risk")
        .ok_or(anyhow!("`risk` is required"))?
        .parse()
        .map_err(|e| anyhow!("`risk`: {e}"))?;

    let category = args
        .get_one::<String>("category")
        .ok_or(anyhow!("`category` is required"))?
        .parse()
        .map_err(|e| anyhow!("`category`: {e}"))?;

    let tx = context
        .hapi_core
        .update_address(UpdateAddressInput {
            address,
            case_id,
            risk,
            category,
        })
        .await?;

    println!("{}", tx.hash);

    Ok(())
}

pub async fn get_address(args: &ArgMatches) -> anyhow::Result<()> {
    let context = CommandContext::try_from(args)?;

    let addr = args
        .get_one::<String>("address")
        .ok_or(anyhow!("`address` is required"))?;

    let address = context.hapi_core.get_address(addr).await?;

    println!("{:#?}", address);

    Ok(())
}

pub async fn get_address_count(args: &ArgMatches) -> anyhow::Result<()> {
    let context = CommandContext::try_from(args)?;

    let count = context.hapi_core.get_address_count().await?;

    println!("{}", count);

    Ok(())
}

pub async fn get_addresses(args: &ArgMatches) -> anyhow::Result<()> {
    let context = CommandContext::try_from(args)?;

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

    let addresses = context.hapi_core.get_addresses(skip, take).await?;

    println!("{:#?}", addresses);

    Ok(())
}

pub async fn create_asset(args: &ArgMatches) -> anyhow::Result<()> {
    let context = CommandContext::try_from(args)?;

    let tx = context.hapi_core.create_asset(CreateAssetInput {}).await?;

    println!("{}", tx.hash);

    Ok(())
}

pub async fn update_asset(args: &ArgMatches) -> anyhow::Result<()> {
    let context = CommandContext::try_from(args)?;

    let tx = context.hapi_core.update_asset(UpdateAssetInput {}).await?;

    println!("{}", tx.hash);

    Ok(())
}

pub async fn get_asset(args: &ArgMatches) -> anyhow::Result<()> {
    let context = CommandContext::try_from(args)?;

    let addr = args
        .get_one::<String>("address")
        .ok_or(anyhow!("`address` is required"))?;

    let asset_id = args
        .get_one::<String>("asset-id")
        .ok_or(anyhow!("`asset-id` is required"))?
        .parse()
        .map_err(|e| anyhow!("`asset-id`: {}", e))?;

    let asset = context.hapi_core.get_asset(addr, &asset_id).await?;

    println!("{:#?}", asset);

    Ok(())
}

pub async fn get_asset_count(args: &ArgMatches) -> anyhow::Result<()> {
    let context = CommandContext::try_from(args)?;

    let count = context.hapi_core.get_asset_count().await?;

    println!("{}", count);

    Ok(())
}

pub async fn get_assets(args: &ArgMatches) -> anyhow::Result<()> {
    let context = CommandContext::try_from(args)?;

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

    let assets = context.hapi_core.get_assets(skip, take).await?;

    println!("{:#?}", assets);

    Ok(())
}
