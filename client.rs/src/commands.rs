use anyhow::anyhow;
use clap::ArgMatches;
use serde_json::json;

use hapi_core::{
    client::{
        configuration::{RewardConfiguration, StakeConfiguration},
        entities::{
            address::{ConfirmAddressInput, CreateAddressInput, UpdateAddressInput},
            asset::{ConfirmAssetInput, CreateAssetInput, UpdateAssetInput},
            case::{CreateCaseInput, UpdateCaseInput},
            reporter::{CreateReporterInput, UpdateReporterInput},
        },
    },
    Amount,
};

mod context;
mod matcher;

pub(crate) use context::{CommandOutput, HapiCoreCommandContext, TokenCommandContext};
pub(crate) use matcher::matcher;

pub async fn get_authority(args: &ArgMatches) -> anyhow::Result<()> {
    let context = HapiCoreCommandContext::try_from(args)?;

    let authority = context.hapi_core.get_authority().await?;

    match context.output {
        CommandOutput::Json => println!("{}", json!({ "authority": authority })),
        CommandOutput::Plain => println!("{}", authority),
    }

    Ok(())
}

pub async fn set_authority(args: &ArgMatches) -> anyhow::Result<()> {
    let context = HapiCoreCommandContext::try_from(args)?;

    let authority = args
        .get_one::<String>("authority")
        .expect("`authority` is required");

    context
        .hapi_core
        .is_valid_address(authority)
        .map_err(|e| anyhow!("Invalid address in `authority`: {e}"))?;

    let tx = context.hapi_core.set_authority(authority).await?;

    match context.output {
        CommandOutput::Json => println!("{}", json!({ "tx": tx.hash })),
        CommandOutput::Plain => println!("{}", tx.hash),
    }

    Ok(())
}

pub async fn update_stake_configuration(args: &ArgMatches) -> anyhow::Result<()> {
    let context = HapiCoreCommandContext::try_from(args)?;

    let token = args
        .get_one::<String>("token")
        .ok_or(anyhow!("`token` is required"))?
        .to_string();

    context
        .hapi_core
        .is_valid_address(&token)
        .map_err(|e| anyhow!("Invalid address in `token`: {e}"))?;

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

    match context.output {
        CommandOutput::Json => println!("{}", json!({ "tx": tx.hash })),
        CommandOutput::Plain => println!("{}", tx.hash),
    }

    Ok(())
}

pub async fn get_stake_configuration(args: &ArgMatches) -> anyhow::Result<()> {
    let context = HapiCoreCommandContext::try_from(args)?;

    let configuration = context.hapi_core.get_stake_configuration().await?;

    match context.output {
        CommandOutput::Json => println!("{}", json!({ "configuration": configuration })),
        CommandOutput::Plain => {
            println!("{configuration:#?}")
        }
    }

    Ok(())
}

pub async fn update_reward_configuration(args: &ArgMatches) -> anyhow::Result<()> {
    let context = HapiCoreCommandContext::try_from(args)?;

    let token = args
        .get_one::<String>("token")
        .ok_or(anyhow!("`token` is required"))?
        .to_string();

    context
        .hapi_core
        .is_valid_address(&token)
        .map_err(|e| anyhow!("Invalid address in `token`: {e}"))?;

    let address_confirmation_reward = args
        .get_one::<String>("address-confirmation-reward")
        .ok_or(anyhow!("`address-confirmation-reward` is required"))?
        .parse()
        .map_err(|e| anyhow!("`address-confirmation-reward`: {e}"))?;

    let asset_tracer_reward = args
        .get_one::<String>("asset-tracer-reward")
        .ok_or(anyhow!("`asset-tracer-reward` is required"))?
        .parse()
        .map_err(|e| anyhow!("`asset-tracer-reward`: {e}"))?;

    let asset_confirmation_reward = args
        .get_one::<String>("asset-confirmation-reward")
        .ok_or(anyhow!("`asset-confirmation-reward` is required"))?
        .parse()
        .map_err(|e| anyhow!("`asset-confirmation-reward`: {e}"))?;

    let address_tracer_reward = args
        .get_one::<String>("address-tracer-reward")
        .ok_or(anyhow!("`address-tracer-reward` is required"))?
        .parse()
        .map_err(|e| anyhow!("`address-tracer-reward`: {e}"))?;

    let cfg = RewardConfiguration {
        token,
        address_confirmation_reward,
        address_tracer_reward,
        asset_confirmation_reward,
        asset_tracer_reward,
    };

    let tx = context.hapi_core.update_reward_configuration(cfg).await?;

    match context.output {
        CommandOutput::Json => println!("{}", json!({ "tx": tx.hash })),
        CommandOutput::Plain => println!("{}", tx.hash),
    }

    Ok(())
}

pub async fn get_reward_configuration(args: &ArgMatches) -> anyhow::Result<()> {
    let context = HapiCoreCommandContext::try_from(args)?;

    let configuration = context.hapi_core.get_reward_configuration().await?;

    match context.output {
        CommandOutput::Json => println!("{}", json!({ "configuration": configuration })),
        CommandOutput::Plain => {
            println!("{configuration:#?}")
        }
    }

    Ok(())
}

pub async fn get_reporters(args: &ArgMatches) -> anyhow::Result<()> {
    let context = HapiCoreCommandContext::try_from(args)?;

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

    match context.output {
        CommandOutput::Json => println!("{}", json!({ "reporters": reporters })),
        CommandOutput::Plain => {
            println!("{:#?}", reporters)
        }
    }

    Ok(())
}

pub async fn get_reporter(args: &ArgMatches) -> anyhow::Result<()> {
    let context = HapiCoreCommandContext::try_from(args)?;

    let reporter_id = args
        .get_one::<String>("id")
        .ok_or(anyhow!("`id` is required"))?;

    let reporter = context.hapi_core.get_reporter(reporter_id).await?;

    match context.output {
        CommandOutput::Json => println!("{}", json!({ "reporter": reporter })),
        CommandOutput::Plain => {
            println!("{:#?}", reporter)
        }
    }

    Ok(())
}

pub async fn get_reporter_count(args: &ArgMatches) -> anyhow::Result<()> {
    let context = HapiCoreCommandContext::try_from(args)?;

    let count = context.hapi_core.get_reporter_count().await?;

    match context.output {
        CommandOutput::Json => println!("{}", json!({ "count": count })),
        CommandOutput::Plain => {
            println!("{count}")
        }
    }

    Ok(())
}

pub async fn create_reporter(args: &ArgMatches) -> anyhow::Result<()> {
    let context = HapiCoreCommandContext::try_from(args)?;

    let id = args
        .get_one::<String>("id")
        .ok_or(anyhow!("`id` is required"))?
        .parse()
        .map_err(|e| anyhow!("`id`: {e}"))?;

    let account: String = args
        .get_one::<String>("account")
        .ok_or(anyhow!("`account` is required"))?
        .parse()
        .map_err(|e| anyhow!("`account`: {e}"))?;

    context
        .hapi_core
        .is_valid_address(&account.clone())
        .map_err(|e| anyhow!("Invalid address in `account`: {e}"))?;

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

    match context.output {
        CommandOutput::Json => println!("{}", json!({ "tx": tx.hash })),
        CommandOutput::Plain => println!("{}", tx.hash),
    }

    Ok(())
}

pub async fn update_reporter(args: &ArgMatches) -> anyhow::Result<()> {
    let context = HapiCoreCommandContext::try_from(args)?;

    let id = args
        .get_one::<String>("id")
        .ok_or(anyhow!("`id` is required"))?
        .parse()
        .map_err(|e| anyhow!("`id`: {e}"))?;

    let account: String = args
        .get_one::<String>("account")
        .ok_or(anyhow!("`account` is required"))?
        .parse()
        .map_err(|e| anyhow!("`account`: {e}"))?;

    context
        .hapi_core
        .is_valid_address(&account.clone())
        .map_err(|e| anyhow!("Invalid address in `account`: {e}"))?;

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

    match context.output {
        CommandOutput::Json => println!("{}", json!({ "tx": tx.hash })),
        CommandOutput::Plain => println!("{}", tx.hash),
    }
    Ok(())
}

pub async fn activate_reporter(args: &ArgMatches) -> anyhow::Result<()> {
    let context = HapiCoreCommandContext::try_from(args)?;

    let tx = context.hapi_core.activate_reporter().await?;

    match context.output {
        CommandOutput::Json => println!("{}", json!({ "tx": tx.hash })),
        CommandOutput::Plain => println!("{}", tx.hash),
    }
    Ok(())
}

pub async fn deactivate_reporter(args: &ArgMatches) -> anyhow::Result<()> {
    let context = HapiCoreCommandContext::try_from(args)?;

    let tx = context.hapi_core.deactivate_reporter().await?;

    match context.output {
        CommandOutput::Json => println!("{}", json!({ "tx": tx.hash })),
        CommandOutput::Plain => println!("{}", tx.hash),
    }

    Ok(())
}

pub async fn unstake_reporter(args: &ArgMatches) -> anyhow::Result<()> {
    let context = HapiCoreCommandContext::try_from(args)?;

    let tx = context.hapi_core.unstake_reporter().await?;

    match context.output {
        CommandOutput::Json => println!("{}", json!({ "tx": tx.hash })),
        CommandOutput::Plain => println!("{}", tx.hash),
    }

    Ok(())
}

pub async fn create_case(args: &ArgMatches) -> anyhow::Result<()> {
    let context = HapiCoreCommandContext::try_from(args)?;

    let id = args
        .get_one::<String>("id")
        .ok_or(anyhow!("`id` is required"))?
        .parse()
        .map_err(|e| anyhow!("`id`: {e}"))?;

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
        .create_case(CreateCaseInput { id, name, url })
        .await?;

    match context.output {
        CommandOutput::Json => println!("{}", json!({ "tx": tx.hash })),
        CommandOutput::Plain => println!("{}", tx.hash),
    }

    Ok(())
}

pub async fn update_case(args: &ArgMatches) -> anyhow::Result<()> {
    let context = HapiCoreCommandContext::try_from(args)?;

    let id = args
        .get_one::<String>("id")
        .ok_or(anyhow!("`id` is required"))?
        .parse()
        .map_err(|e| anyhow!("`id`: {e}"))?;

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

    let status = args
        .get_one::<String>("status")
        .ok_or(anyhow!("`status` is required"))?
        .parse()
        .map_err(|e| anyhow!("`status`: {e}"))?;

    let tx = context
        .hapi_core
        .update_case(UpdateCaseInput {
            id,
            name,
            url,
            status,
        })
        .await?;

    match context.output {
        CommandOutput::Json => println!("{}", json!({ "tx": tx.hash })),
        CommandOutput::Plain => println!("{}", tx.hash),
    }

    Ok(())
}

pub async fn get_case(args: &ArgMatches) -> anyhow::Result<()> {
    let context = HapiCoreCommandContext::try_from(args)?;

    let case_id = args
        .get_one::<String>("id")
        .ok_or(anyhow!("`id` is required"))?;

    let case = context.hapi_core.get_case(case_id).await?;

    match context.output {
        CommandOutput::Json => println!("{}", json!({ "case": case })),
        CommandOutput::Plain => {
            println!("{:#?}", case)
        }
    }

    Ok(())
}

pub async fn get_case_count(args: &ArgMatches) -> anyhow::Result<()> {
    let context = HapiCoreCommandContext::try_from(args)?;

    let count = context.hapi_core.get_case_count().await?;

    match context.output {
        CommandOutput::Json => println!("{}", json!({ "count": count })),
        CommandOutput::Plain => {
            println!("{count}")
        }
    }

    Ok(())
}

pub async fn get_cases(args: &ArgMatches) -> anyhow::Result<()> {
    let context = HapiCoreCommandContext::try_from(args)?;

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

    match context.output {
        CommandOutput::Json => println!("{}", json!({ "cases": cases })),
        CommandOutput::Plain => {
            println!("{:#?}", cases)
        }
    }

    Ok(())
}

pub async fn create_address(args: &ArgMatches) -> anyhow::Result<()> {
    let context = HapiCoreCommandContext::try_from(args)?;

    let address = args
        .get_one::<String>("address")
        .ok_or(anyhow!("`address` is required"))?
        .to_owned();

    context
        .hapi_core
        .is_valid_address(&address.clone())
        .map_err(|e| anyhow!("Invalid address in `address`: {e}"))?;

    let case_id = args
        .get_one::<String>("case-id")
        .ok_or(anyhow!("`case-id` is required"))?
        .parse()
        .map_err(|e| anyhow!("`case-id`: {e}"))?;

    let risk = *args
        .get_one::<u8>("risk")
        .ok_or(anyhow!("`risk` is required"))?;

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

    match context.output {
        CommandOutput::Json => println!("{}", json!({ "tx": tx.hash })),
        CommandOutput::Plain => println!("{}", tx.hash),
    }

    Ok(())
}

pub async fn update_address(args: &ArgMatches) -> anyhow::Result<()> {
    let context = HapiCoreCommandContext::try_from(args)?;

    let address = args
        .get_one::<String>("address")
        .ok_or(anyhow!("`address` is required"))?
        .to_owned();

    context
        .hapi_core
        .is_valid_address(&address.clone())
        .map_err(|e| anyhow!("Invalid address in `address`: {e}"))?;

    let case_id = args
        .get_one::<String>("case-id")
        .ok_or(anyhow!("`case-id` is required"))?
        .parse()
        .map_err(|e| anyhow!("`case-id`: {e}"))?;

    let risk = *args
        .get_one::<u8>("risk")
        .ok_or(anyhow!("`risk` is required"))?;

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

    match context.output {
        CommandOutput::Json => println!("{}", json!({ "tx": tx.hash })),
        CommandOutput::Plain => println!("{}", tx.hash),
    }

    Ok(())
}

pub async fn confirm_address(args: &ArgMatches) -> anyhow::Result<()> {
    let context = HapiCoreCommandContext::try_from(args)?;

    let address = args
        .get_one::<String>("address")
        .ok_or(anyhow!("`address` is required"))?
        .to_owned();

    context
        .hapi_core
        .is_valid_address(&address.clone())
        .map_err(|e| anyhow!("Invalid address in `address`: {e}"))?;

    let tx = context
        .hapi_core
        .confirm_address(ConfirmAddressInput { address })
        .await?;

    match context.output {
        CommandOutput::Json => println!("{}", json!({ "tx": tx.hash })),
        CommandOutput::Plain => println!("{}", tx.hash),
    }

    Ok(())
}

pub async fn get_address(args: &ArgMatches) -> anyhow::Result<()> {
    let context = HapiCoreCommandContext::try_from(args)?;

    let addr = args
        .get_one::<String>("address")
        .ok_or(anyhow!("`address` is required"))?;

    context
        .hapi_core
        .is_valid_address(addr)
        .map_err(|e| anyhow!("Invalid address in `addr`: {e}"))?;

    let address = context.hapi_core.get_address(addr).await?;

    match context.output {
        CommandOutput::Json => println!("{}", json!({ "address": address })),
        CommandOutput::Plain => {
            println!("{:#?}", address)
        }
    }

    Ok(())
}

pub async fn get_address_count(args: &ArgMatches) -> anyhow::Result<()> {
    let context = HapiCoreCommandContext::try_from(args)?;

    let count = context.hapi_core.get_address_count().await?;

    match context.output {
        CommandOutput::Json => println!("{}", json!({ "count": count })),
        CommandOutput::Plain => {
            println!("{count}")
        }
    }

    Ok(())
}

pub async fn get_addresses(args: &ArgMatches) -> anyhow::Result<()> {
    let context = HapiCoreCommandContext::try_from(args)?;

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

    match context.output {
        CommandOutput::Json => println!("{}", json!({ "addresses": addresses })),
        CommandOutput::Plain => {
            println!("{:#?}", addresses)
        }
    }

    Ok(())
}

pub async fn create_asset(args: &ArgMatches) -> anyhow::Result<()> {
    let context = HapiCoreCommandContext::try_from(args)?;

    let address = args
        .get_one::<String>("address")
        .ok_or(anyhow!("`address` is required"))?
        .to_owned();

    context
        .hapi_core
        .is_valid_address(&address.clone())
        .map_err(|e| anyhow!("Invalid address in `address`: {e}"))?;

    let asset_id = args
        .get_one::<String>("asset-id")
        .ok_or(anyhow!("`asset-id` is required"))?
        .parse()
        .map_err(|e| anyhow!("`asset-id`: {e}"))?;

    let case_id = args
        .get_one::<String>("case-id")
        .ok_or(anyhow!("`case-id` is required"))?
        .parse()
        .map_err(|e| anyhow!("`case-id`: {e}"))?;

    let risk = *args
        .get_one::<u8>("risk")
        .ok_or(anyhow!("`risk` is required"))?;

    let category = args
        .get_one::<String>("category")
        .ok_or(anyhow!("`category` is required"))?
        .parse()
        .map_err(|e| anyhow!("`category`: {e}"))?;

    let tx = context
        .hapi_core
        .create_asset(CreateAssetInput {
            address,
            asset_id,
            case_id,
            risk,
            category,
        })
        .await?;

    match context.output {
        CommandOutput::Json => println!("{}", json!({ "tx": tx.hash })),
        CommandOutput::Plain => println!("{}", tx.hash),
    }

    Ok(())
}

pub async fn update_asset(args: &ArgMatches) -> anyhow::Result<()> {
    let context = HapiCoreCommandContext::try_from(args)?;

    let address = args
        .get_one::<String>("address")
        .ok_or(anyhow!("`address` is required"))?
        .to_owned();

    context
        .hapi_core
        .is_valid_address(&address.clone())
        .map_err(|e| anyhow!("Invalid address in `address`: {e}"))?;

    let asset_id = args
        .get_one::<String>("asset-id")
        .ok_or(anyhow!("`asset-id` is required"))?
        .parse()
        .map_err(|e| anyhow!("`asset-id`: {e}"))?;

    let case_id = args
        .get_one::<String>("case-id")
        .ok_or(anyhow!("`case-id` is required"))?
        .parse()
        .map_err(|e| anyhow!("`case-id`: {e}"))?;

    let risk = *args
        .get_one::<u8>("risk")
        .ok_or(anyhow!("`risk` is required"))?;

    let category = args
        .get_one::<String>("category")
        .ok_or(anyhow!("`category` is required"))?
        .parse()
        .map_err(|e| anyhow!("`category`: {e}"))?;

    let tx = context
        .hapi_core
        .update_asset(UpdateAssetInput {
            address,
            asset_id,
            case_id,
            risk,
            category,
        })
        .await?;

    match context.output {
        CommandOutput::Json => println!("{}", json!({ "tx": tx.hash })),
        CommandOutput::Plain => println!("{}", tx.hash),
    }

    Ok(())
}

pub async fn confirm_asset(args: &ArgMatches) -> anyhow::Result<()> {
    let context = HapiCoreCommandContext::try_from(args)?;

    let address = args
        .get_one::<String>("address")
        .ok_or(anyhow!("`address` is required"))?
        .to_owned();

    context
        .hapi_core
        .is_valid_address(&address.clone())
        .map_err(|e| anyhow!("Invalid address in `address`: {e}"))?;

    let asset_id = args
        .get_one::<String>("asset-id")
        .ok_or(anyhow!("`asset-id` is required"))?
        .parse()
        .map_err(|e| anyhow!("`asset-id`: {e}"))?;

    let tx = context
        .hapi_core
        .confirm_asset(ConfirmAssetInput { address, asset_id })
        .await?;

    match context.output {
        CommandOutput::Json => println!("{}", json!({ "tx": tx.hash })),
        CommandOutput::Plain => println!("{}", tx.hash),
    }

    Ok(())
}

pub async fn get_asset(args: &ArgMatches) -> anyhow::Result<()> {
    let context = HapiCoreCommandContext::try_from(args)?;

    let addr = args
        .get_one::<String>("address")
        .ok_or(anyhow!("`address` is required"))?;

    context
        .hapi_core
        .is_valid_address(addr)
        .map_err(|e| anyhow!("Invalid address in `address`: {e}"))?;

    let asset_id = args
        .get_one::<String>("asset-id")
        .ok_or(anyhow!("`asset-id` is required"))?
        .parse()
        .map_err(|e| anyhow!("`asset-id`: {}", e))?;

    let asset = context.hapi_core.get_asset(addr, &asset_id).await?;

    match context.output {
        CommandOutput::Json => println!("{}", json!({ "asset": asset })),
        CommandOutput::Plain => {
            println!("{:#?}", asset)
        }
    }

    Ok(())
}

pub async fn get_asset_count(args: &ArgMatches) -> anyhow::Result<()> {
    let context = HapiCoreCommandContext::try_from(args)?;

    let count = context.hapi_core.get_asset_count().await?;

    match context.output {
        CommandOutput::Json => println!("{}", json!({ "count": count })),
        CommandOutput::Plain => {
            println!("{count}")
        }
    }

    Ok(())
}

pub async fn get_assets(args: &ArgMatches) -> anyhow::Result<()> {
    let context = HapiCoreCommandContext::try_from(args)?;

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

    match context.output {
        CommandOutput::Json => println!("{}", json!({ "assets": assets })),
        CommandOutput::Plain => {
            println!("{:#?}", assets)
        }
    }

    Ok(())
}

pub async fn transfer_token(args: &ArgMatches) -> anyhow::Result<()> {
    let context = TokenCommandContext::try_from(args)?;

    let to = args
        .get_one::<String>("to")
        .ok_or(anyhow!("`to` is required"))?;

    let amount: Amount = args
        .get_one::<String>("amount")
        .ok_or(anyhow!("`amount` is required"))?
        .parse()
        .map_err(|e| anyhow!("`amount`: {}", e))?;

    let tx = context.token.transfer(to, amount).await?;

    match context.output {
        CommandOutput::Json => println!("{}", json!({ "tx": tx.hash })),
        CommandOutput::Plain => println!("{}", tx.hash),
    }

    Ok(())
}

pub async fn approve_token(args: &ArgMatches) -> anyhow::Result<()> {
    let context = TokenCommandContext::try_from(args)?;

    let spender = args
        .get_one::<String>("spender")
        .ok_or(anyhow!("`spender` is required"))?;

    let amount: Amount = args
        .get_one::<String>("amount")
        .ok_or(anyhow!("`amount` is required"))?
        .parse()
        .map_err(|e| anyhow!("`amount`: {}", e))?;

    let tx = context.token.approve(spender, amount).await?;

    match context.output {
        CommandOutput::Json => println!("{}", json!({ "tx": tx.hash })),
        CommandOutput::Plain => println!("{}", tx.hash),
    }

    Ok(())
}

pub async fn balance_token(args: &ArgMatches) -> anyhow::Result<()> {
    let context = TokenCommandContext::try_from(args)?;

    let address = args
        .get_one::<String>("address")
        .ok_or(anyhow!("`address` is required"))?;

    let balance = context.token.balance(address).await?;

    match context.output {
        CommandOutput::Json => println!("{}", json!({ "balance": balance })),
        CommandOutput::Plain => println!("{}", balance),
    }

    Ok(())
}
