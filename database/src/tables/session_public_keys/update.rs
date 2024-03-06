use crate::db::Db;
use crate::tables::session_public_keys::table_struct::{
    SESSION_PUBLIC_KEYS_KEYS, SESSION_PUBLIC_KEYS_TABLE_NAME,
};
use sqlx::Transaction;
use sqlx::{query, Postgres};

impl Db {
    pub async fn create_new_session_public_key(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        session_id: &String,
        public_key: String,
        client_profile_id: Option<i64>,
        main_session_key: bool,
    ) -> Result<(), sqlx::Error> {
        let query_body = format!(
            "INSERT INTO {SESSION_PUBLIC_KEYS_TABLE_NAME} ({SESSION_PUBLIC_KEYS_KEYS}) VALUES (DEFAULT, $1, $2, $3, $4, DEFAULT)"
        );

        let query_result = query(&query_body)
            .bind(&session_id)
            .bind(&public_key)
            .bind(&client_profile_id)
            .bind(&main_session_key)
            .execute(&mut **tx)
            .await;

        match query_result {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_session_public_key_entry() {
        let db = Db::connect_to_the_pool().await;
        db.truncate_all_tables().await.unwrap();

        // Define a public key to test with
        let public_key_str = "test_public_key".to_string();

        // Create Public key
        let mut tx = db.connection_pool.begin().await.unwrap();
        let (client_profile_id, _public_key) = db
            .handle_public_keys_entries(&mut tx, &vec![public_key_str.clone()])
            .await
            .unwrap();
        assert!(client_profile_id == 1);

        // Create session public key
        let session_id = "test_session_id".to_string();
        let mut tx = db.connection_pool.begin().await.unwrap();
        db.create_new_session_public_key(
            &mut tx,
            &session_id,
            public_key_str.clone(),
            Some(client_profile_id),
            true,
        )
        .await
        .unwrap();

        // Commit changes
        tx.commit().await.unwrap();

        // Retrieve the inserted public key to verify
        let session_keys = db.get_session_public_keys(&session_id).await.unwrap();

        assert_eq!(session_keys.len(), 1);
        assert_eq!(session_keys[0].public_key, public_key_str);
    }
}
