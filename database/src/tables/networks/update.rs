use super::table_struct::{NETWORKS_KEYS, NETWORKS_TABLE_NAME};
use crate::{db::Db, structs::db_error::DbError};
use sqlx::{query, Transaction};

impl Db {
    pub async fn add_new_network(
        &self,
        tx: &mut Transaction<'_, sqlx::Postgres>,
        network: &String,
    ) -> Result<(), DbError> {
        let query_body = format!(
            "INSERT INTO {NETWORKS_TABLE_NAME} ({NETWORKS_KEYS}) VALUES ($1) ON CONFLICT DO NOTHING"
        );
        let query_result = query(&query_body).bind(network).execute(&mut **tx).await;

        match query_result {
            Ok(_) => Ok(()),
            Err(e) => Err(e).map_err(|e| e.into()),
        }
    }
}

#[cfg(feature = "cloud_integration_tests")]
#[cfg(test)]
mod tests {

    #[tokio::test]
    async fn test_add_new_network() {
        let db = super::Db::connect_to_the_pool().await;
        db.truncate_all_tables().await.unwrap();

        let network = "test_network".to_string();

        let mut tx = db.connection_pool.begin().await.unwrap();
        db.add_new_network(&mut tx, &network).await.unwrap();
        tx.commit().await.unwrap();

        let networks = db.get_all_networks().await.unwrap();
        assert_eq!(networks.len(), 1);
        assert_eq!(networks[0].network, network);

        let mut tx = db.connection_pool.begin().await.unwrap();
        db.add_new_network(&mut tx, &network).await.unwrap();
        tx.commit().await.unwrap();

        let networks = db.get_all_networks().await.unwrap();
        assert_eq!(networks.len(), 1);
        assert_eq!(networks[0].network, network);
    }
}
