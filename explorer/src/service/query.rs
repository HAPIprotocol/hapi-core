use {
    async_graphql::{InputType, OutputType},
    chrono::{Duration, NaiveDate},
    sea_orm::{prelude::*, sea_query::Cond, PaginatorTrait},
};

use crate::entity::{
    network,
    pagination::{EntityInput, EntityPage, Paginator},
    types::NetworkBackend,
    EntityFilter,
};

pub struct EntityQuery;

impl EntityQuery {
    /// Universal method for fetching entity from database
    pub async fn find_entity_by_id<M, T>(db: &DbConn, id: T) -> Result<Option<M::Model>, DbErr>
    where
        M: EntityTrait,
        T: Into<<M::PrimaryKey as PrimaryKeyTrait>::ValueType>,
    {
        M::find_by_id(id).one(db).await
    }

    /// Universal method for fetching entities from database
    pub async fn find_many<M>(
        db: &DbConn,
        input: EntityInput<<M as EntityFilter>::Filter, <M as EntityFilter>::Condition>,
    ) -> Result<EntityPage<M::Model>, DbErr>
    where
        M: EntityTrait + EntityFilter,
        <M as EntityFilter>::Filter: InputType,
        <M as EntityFilter>::Condition: InputType + Default,
        M::Model: OutputType,
        M::Column: From<<M as EntityFilter>::Condition>,
    {
        let mut query = M::find();

        if let Some(filter) = input.filtering {
            query = M::filter(query, &filter);
        }

        if let Some(search) = input.search {
            query = Self::search(query, &search);
        }

        query = M::order(query, input.ordering, input.ordering_condition);

        Self::paginate(db, query, input.pagination).await
    }

    fn search<M>(query: Select<M>, value: &str) -> Select<M>
    where
        M: EntityTrait + EntityFilter,
    {
        let value = &format!("%{}%", value);
        let columns = M::columns_for_search();
        let mut cond = Cond::any();

        for column in columns {
            cond = cond.add(Expr::cust(column).like(value));
        }

        query.filter(cond)
    }

    /// Method for query pagination
    async fn paginate<M>(
        db: &DbConn,
        query: Select<M>,
        pagination: Option<Paginator>,
    ) -> Result<EntityPage<M::Model>, DbErr>
    where
        M: EntityTrait,
        M::Model: OutputType,
    {
        let page = if let Some(pagination) = pagination {
            let paginator = query.paginate(db, pagination.page_size);
            let total = paginator.num_items().await?;
            let data = paginator.fetch_page(pagination.page_num - 1).await?;

            let page_count =
                total / pagination.page_size + u64::from(total % pagination.page_size != 0);

            EntityPage {
                data,
                total,
                page_count,
            }
        } else {
            let data = query.all(db).await?;
            let total = data.len() as u64;

            EntityPage {
                data,
                total,
                page_count: 1,
            }
        };

        Ok(page)
    }
}

pub async fn get_network_id(
    db: &DbConn,
    backend: NetworkBackend,
    chain_id: Option<String>,
) -> Result<String, DbErr> {
    let mut filtered = network::Entity::find().filter(network::Column::Backend.eq(backend));

    filtered = if let Some(chain_id) = chain_id {
        filtered.filter(network::Column::ChainId.eq(chain_id))
    } else {
        filtered.filter(network::Column::ChainId.is_null())
    };

    Ok(filtered
        .one(db)
        .await?
        .ok_or(DbErr::RecordNotFound(
            "This network does not exist".to_string(),
        ))?
        .id)
}

pub async fn count_rows_per_week<M>(
    db: &DbConn,
    query: Select<M>,
    year: i32,
    week: u32,
) -> Result<u64, DbErr>
where
    M: EntityTrait,
    M::Model: OutputType,
{
    let start_of_week = NaiveDate::from_isoywd_opt(year, week, chrono::Weekday::Mon).ok_or(
        DbErr::Custom(format!("Invalid week: {} for year: {}", week, year)),
    )?;
    let end_of_week = start_of_week + Duration::days(7);

    let count = query
        .filter(Expr::cust(format!(
            "DATE(created_at) BETWEEN '{start_of_week}' AND '{end_of_week}'"
        )))
        .count(db)
        .await?;

    Ok(count)
}
