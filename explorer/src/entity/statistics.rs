use {
    async_graphql::{Context, Object, Result, SimpleObject},
    chrono::{Datelike, Utc, Weekday},
    sea_orm::{DatabaseConnection, EntityTrait, PaginatorTrait, QuerySelect},
    tracing::instrument,
    web3::types::U256,
};

use super::{
    address::query_utils::AddressCondition, asset::query_utils::AssetCondition,
    case::query_utils::CaseCondition, pagination::DEFAULT_PAGE_SIZE,
};

use crate::{
    entity::{address, asset, case, reporter, EntityFilter},
    service::count_rows_per_week,
};

pub const CHART_LENGTH: usize = 10;

#[derive(Clone, Debug, PartialEq, Eq, SimpleObject)]
#[graphql(name = "Dashboard")]
pub struct Dashboard {
    pub staked_by_reporters: String,
    pub total_reporters_count: u64,
    pub total_addresses_count: u64,
    pub total_asset_count: u64,
    pub total_case_count: u64,
    pub new_weekly_case_count: u64,
    pub new_weekly_address_count: u64,
    pub new_weekly_asset_count: u64,
    pub last_added_addresses: Vec<address::Model>,
    pub last_added_assets: Vec<asset::Model>,
    pub top_cases_by_address: Vec<case::Model>,
    pub top_cases_by_asset: Vec<case::Model>,
}

#[derive(Clone, Debug, PartialEq, Eq, SimpleObject)]
pub struct Charts {
    pub labels: Vec<String>,
    pub addresses: Vec<u64>,
    pub assets: Vec<u64>,
    pub cases: Vec<u64>,
}

/// The GraphQl Query segment
#[derive(Default)]
pub struct StatisticsQuery {}

/// Queries for the statistics
#[Object]
impl StatisticsQuery {
    /// Get a dashboard statistics
    #[instrument(level = "debug", skip(self, ctx))]
    pub async fn get_dashboard(&self, ctx: &Context<'_>) -> Result<Dashboard> {
        let db = ctx.data_unchecked::<DatabaseConnection>();
        let (year, week) = get_current_week();

        let (staked_by_reporters, total_reporters_count) = get_reporter_dashboard(db).await?;

        let (total_addresses_count, new_weekly_address_count, last_added_addresses) =
            get_address_dashboard(db, year, week).await?;

        let (total_asset_count, new_weekly_asset_count, last_added_assets) =
            get_asset_dashboard(db, year, week).await?;

        let (total_case_count, new_weekly_case_count, top_cases_by_address, top_cases_by_asset) =
            get_case_dashboard(db, year, week).await?;

        let dashboard = Dashboard {
            staked_by_reporters,
            total_reporters_count,

            total_case_count,
            new_weekly_case_count,
            top_cases_by_address,
            top_cases_by_asset,

            total_addresses_count,
            new_weekly_address_count,
            last_added_addresses,

            total_asset_count,
            new_weekly_asset_count,
            last_added_assets,
        };

        Ok(dashboard)
    }

    /// Get a chart statistics
    #[instrument(level = "debug", skip(self, ctx))]
    pub async fn get_charts(&self, ctx: &Context<'_>) -> Result<Charts> {
        let db = ctx.data_unchecked::<DatabaseConnection>();
        let weeks = get_past_weeks();

        let mut labels = vec![];
        let mut addresses = vec![];
        let mut assets = vec![];
        let mut cases = vec![];

        for (year, week) in weeks {
            let label = format!("{}:{}", year, week);
            labels.push(label);

            let addresses_count = count_rows_per_week::<address::Entity>(db, year, week).await?;
            addresses.push(addresses_count);

            let assets_count = count_rows_per_week::<asset::Entity>(db, year, week).await?;
            assets.push(assets_count);

            let cases_count = count_rows_per_week::<case::Entity>(db, year, week).await?;
            cases.push(cases_count);
        }

        let charts = Charts {
            labels,
            addresses,
            assets,
            cases,
        };

        Ok(charts)
    }
}

fn get_current_week() -> (i32, u32) {
    let now = Utc::now();
    let iso_week = now.iso_week();
    (iso_week.year(), iso_week.week())
}

fn get_past_weeks() -> Vec<(i32, u32)> {
    let mut weeks = Vec::new();
    let mut current_date = Utc::now();

    // Adjust to the start of the current week (assuming weeks start on Monday)
    if current_date.weekday() != Weekday::Mon {
        current_date = current_date
            - chrono::Duration::days(current_date.weekday().num_days_from_monday() as i64);
    }

    for _ in 0..CHART_LENGTH {
        let iso_week = current_date.iso_week();
        weeks.push((iso_week.year(), iso_week.week()));

        current_date -= chrono::Duration::weeks(1);
    }

    weeks.reverse();

    weeks
}

async fn get_reporter_dashboard(db: &DatabaseConnection) -> Result<(String, u64)> {
    let stakes: Vec<String> = reporter::Entity::find()
        .select_only()
        .column(reporter::Column::Stake)
        .into_tuple()
        .all(db)
        .await?
        .into_iter()
        .collect();

    let total_reporters_count = stakes.len() as u64;
    let staked_by_reporters = stakes
        .iter()
        .map(|s: &String| U256::from_dec_str(s).unwrap())
        .fold(U256::zero(), |acc, x| acc + x)
        .to_string();

    Ok((staked_by_reporters, total_reporters_count))
}

async fn get_case_dashboard(
    db: &DatabaseConnection,
    year: i32,
    week: u32,
) -> Result<(u64, u64, Vec<case::Model>, Vec<case::Model>)> {
    let total_case_count = case::Entity::find().count(db).await?;
    let new_weekly_case_count = count_rows_per_week::<case::Entity>(db, year, week).await?;
    let top_cases_by_address = case::Entity::order(
        case::Entity::find(),
        None,
        Some(CaseCondition::AddressCount),
    )
    .paginate(db, DEFAULT_PAGE_SIZE)
    .fetch()
    .await?;

    let top_cases_by_asset =
        case::Entity::order(case::Entity::find(), None, Some(CaseCondition::AssetCount))
            .paginate(db, DEFAULT_PAGE_SIZE)
            .fetch()
            .await?;

    Ok((
        total_case_count,
        new_weekly_case_count,
        top_cases_by_address,
        top_cases_by_asset,
    ))
}

async fn get_address_dashboard(
    db: &DatabaseConnection,
    year: i32,
    week: u32,
) -> Result<(u64, u64, Vec<address::Model>)> {
    let total_addresses_count = address::Entity::find().count(db).await?;

    let new_weekly_address_count = count_rows_per_week::<address::Entity>(db, year, week).await?;

    let last_added_addresses = address::Entity::order(
        address::Entity::find(),
        None,
        Some(AddressCondition::CreatedAt),
    )
    .paginate(db, DEFAULT_PAGE_SIZE)
    .fetch()
    .await?;

    Ok((
        total_addresses_count,
        new_weekly_address_count,
        last_added_addresses,
    ))
}

async fn get_asset_dashboard(
    db: &DatabaseConnection,
    year: i32,
    week: u32,
) -> Result<(u64, u64, Vec<asset::Model>)> {
    let total_asset_count = asset::Entity::find().count(db).await?;

    let new_weekly_asset_count = count_rows_per_week::<asset::Entity>(db, year, week).await?;

    let last_added_assets =
        asset::Entity::order(asset::Entity::find(), None, Some(AssetCondition::CreatedAt))
            .paginate(db, DEFAULT_PAGE_SIZE)
            .fetch()
            .await?;

    Ok((total_asset_count, new_weekly_asset_count, last_added_assets))
}
