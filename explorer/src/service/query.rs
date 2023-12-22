use {
    async_graphql::{InputType, OutputType},
    sea_orm::{DbConn, DbErr, EntityTrait, PaginatorTrait, PrimaryKeyTrait, QueryOrder, Select},
};

use crate::entity::{
    pagination::{EntityInput, EntityPage, Ordering, Paginator},
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
        <M as EntityFilter>::Condition: InputType,
        M::Model: OutputType,
        M::Column: From<<M as EntityFilter>::Condition>,
    {
        let mut query = M::find();

        if let Some(filter) = input.filtering {
            query = M::filter(query, &filter);
        }

        query = Self::order_by(query, input.ordering, input.ordering_condition);

        Self::paginate(db, query, input.pagination).await
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

    // Method for paginating query ordering
    fn order_by<M, C>(query: Select<M>, ordering: Ordering, condition: C) -> Select<M>
    where
        M: EntityTrait,
        M::Column: From<C>,
    {
        let column = M::Column::from(condition);
        match ordering {
            Ordering::Asc => query.order_by_asc(column),
            Ordering::Desc => query.order_by_desc(column),
        }
    }
}
