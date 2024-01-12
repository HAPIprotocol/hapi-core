use {
    async_graphql::SimpleObject,
    hapi_core::client::entities::address::Address as AddressPayload,
    sea_orm::{entity::prelude::*, NotSet, Set},
};

use super::query_utils::{AddressCondition, AddressFilter};
use crate::entity::{
    case, reporter,
    types::{Category, NetworkBackend},
    EntityFilter, FromPayload,
};

// Risk and confirmations do not correspond to the types of contracts (due to Postgresql restrictions)
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, SimpleObject)]
#[graphql(name = "Address")]
#[sea_orm(table_name = "address")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub network: NetworkBackend,
    #[sea_orm(primary_key, auto_increment = false)]
    pub address: String,
    pub case_id: Uuid,
    pub reporter_id: Uuid,
    pub risk: i16,
    pub category: Category,
    pub confirmations: String,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

impl EntityFilter for Entity {
    type Filter = AddressFilter;
    type Condition = AddressCondition;

    // Fitlering query
    fn filter(selected: Select<Entity>, filter_options: &AddressFilter) -> Select<Entity> {
        let mut query = selected;

        if let Some(network) = filter_options.network {
            query = query.filter(Column::Network.eq(network));
        }

        if let Some(case_id) = filter_options.case_id {
            query = query.filter(Column::CaseId.eq(case_id));
        }

        if let Some(reporter_id) = filter_options.reporter_id {
            query = query.filter(Column::ReporterId.eq(reporter_id));
        }

        if let Some(category) = filter_options.category {
            query = query.filter(Column::Category.eq(category));
        }

        if let Some(risk) = filter_options.risk {
            query = query.filter(Column::Risk.eq(risk));
        }

        if let Some(confirmations) = &filter_options.confirmations {
            query = query.filter(Column::Confirmations.eq(confirmations));
        }

        query
    }
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "reporter::Entity",
        from = "Column::ReporterId",
        to = "reporter::model::Column::ReporterId"
    )]
    Reporter,
    #[sea_orm(
        belongs_to = "case::Entity",
        from = "Column::CaseId",
        to = "case::model::Column::CaseId"
    )]
    Case,
}

impl Related<reporter::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Reporter.def()
    }
}

impl Related<case::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Case.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

impl FromPayload<AddressPayload> for ActiveModel {
    fn from(
        network: NetworkBackend,
        created_at: Option<DateTime>,
        updated_at: Option<DateTime>,
        payload: AddressPayload,
    ) -> Self {
        let created_at = created_at.map_or(NotSet, Set);
        let updated_at = updated_at.map_or(NotSet, Set);

        Self {
            network: Set(network),
            address: Set(payload.address.to_owned()),
            case_id: Set(payload.case_id.to_owned()),
            reporter_id: Set(payload.reporter_id.to_owned()),
            risk: Set(payload.risk.into()),
            category: Set(payload.category.into()),
            confirmations: Set(payload.confirmations.to_string()),
            created_at,
            updated_at,
        }
    }
}
