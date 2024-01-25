use crate::entity::{
    indexer, network,
    {types::NetworkBackend, FromPayload},
};

use {
    chrono::{DateTime, NaiveDateTime, Utc},
    hapi_indexer::NetworkData,
    sea_orm::*,
    uuid::Uuid,
};

pub struct EntityMutation;

impl EntityMutation {
    /// Universal method for inserting entities to database
    pub async fn create_entity<M, T>(
        db: &DbConn,
        payload: T,
        network: NetworkData,
        timestamp: u64,
    ) -> Result<<M::Entity as EntityTrait>::Model, DbErr>
    where
        <M::Entity as EntityTrait>::Model: IntoActiveModel<M>,
        M: ActiveModelBehavior + FromPayload<T> + Send,
    {
        let network_id = get_network_id(db, network.network.into(), network.chain_id).await?;
        let created_at = Some(
            NaiveDateTime::from_timestamp_opt(timestamp as i64, 0)
                .ok_or(DbErr::Custom("Invalid block timestamp".to_string()))?,
        );

        M::from(network_id, created_at, created_at, payload)
            .insert(db)
            .await
    }

    /// Universal method for updating entities in database
    pub async fn update_entity<M, T>(
        db: &DbConn,
        payload: T,
        network: NetworkData,
        timestamp: u64,
    ) -> Result<<M::Entity as EntityTrait>::Model, DbErr>
    where
        <M::Entity as EntityTrait>::Model: IntoActiveModel<M>,
        M: ActiveModelBehavior + FromPayload<T> + Send,
    {
        let network_id = get_network_id(db, network.network.into(), network.chain_id).await?;
        let updated_at = Some(
            NaiveDateTime::from_timestamp_opt(timestamp as i64, 0)
                .ok_or(DbErr::Custom("Invalid block timestamp".to_string()))?,
        );

        M::from(network_id, None, updated_at, payload)
            .update(db)
            .await
    }

    /// Method for creating network in database
    pub async fn create_network(
        db: &DbConn,
        id: String,
        name: String,
        backend: NetworkBackend,
        chain_id: Option<String>,
        authority: String,
        stake_token: String,
    ) -> Result<network::Model, DbErr> {
        let model: network::ActiveModel = network::ActiveModel {
            id: Set(id),
            name: Set(name),
            backend: Set(backend),
            chain_id: Set(chain_id),
            authority: Set(authority),
            stake_token: Set(stake_token),
            created_at: Set(chrono::Utc::now().naive_utc()),
            updated_at: Set(chrono::Utc::now().naive_utc()),
        };

        model.insert(db).await
    }

    /// Method for updating network in database
    pub async fn update_network(
        db: &DbConn,
        id: String,
        name: Option<String>,
        authority: Option<String>,
        stake_token: Option<String>,
    ) -> Result<network::Model, DbErr> {
        let name = name.map_or(NotSet, Set);
        let stake_token = stake_token.map_or(NotSet, Set);
        let authority = authority.map_or(NotSet, Set);

        let model = network::ActiveModel {
            id: Set(id),
            name,
            backend: NotSet,
            chain_id: NotSet,
            authority,
            stake_token,
            created_at: Set(chrono::Utc::now().naive_utc()),
            updated_at: Set(chrono::Utc::now().naive_utc()),
        };

        model.update(db).await
    }

    pub async fn create_indexer(
        db: &DbConn,
        backend: NetworkBackend,
        chain_id: Option<String>,
        id: Uuid,
        timestamp: DateTime<Utc>,
    ) -> Result<indexer::Model, DbErr> {
        let network_id = get_network_id(db, backend, chain_id).await?;

        indexer::ActiveModel {
            id: Set(id),
            network_id: Set(network_id),
            created_at: Set(timestamp.naive_utc()),
            last_heartbeat: Set(NaiveDateTime::default()),
            cursor: Set("".to_string()),
        }
        .insert(db)
        .await
    }
}

async fn get_network_id(
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
