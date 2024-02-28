use crate::db::Db;
use crate::tables::client_profiles::table_struct::ClientProfile;
use crate::tables::public_keys::table_struct::{
    PublicKey, PUBLIC_KEYS_KEYS, PUBLIC_KEYS_TABLE_NAME,
};
use sqlx::{query, Postgres};
use sqlx::{query_as, Transaction};

impl Db {
    // We can create a new entry or update an existing one, returns the client_profile_id
    pub async fn handle_public_key_entry(&self, public_key: &String) -> Result<i64, sqlx::Error> {
        // Start a transaction
        let mut tx: Transaction<'_, Postgres> = self.connection_pool.begin().await?;

        match self.update_public_key_last_seen(&mut tx, public_key).await {
            Ok(key) => {
                tx.commit().await?;
                return Ok(key.client_profile_id);
            }
            Err(_) => {
                // This should not fail, but just in case
                let client_profile = match self.create_new_profile(Some(&mut tx)).await {
                    Ok(profile) => profile,
                    Err(e) => {
                        tx.rollback().await?;
                        return Err(e);
                    }
                };

                if let Err(e) = self
                    .create_new_public_key(&mut tx, public_key, &client_profile)
                    .await
                {
                    tx.rollback().await?;
                    return Err(e);
                }

                tx.commit().await?;
                return Ok(client_profile.client_profile_id);
            }
        }
    }

    pub async fn create_new_public_key(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        public_key: &String,
        client_profile: &ClientProfile,
    ) -> Result<(), sqlx::Error> {
        let query_body = format!(
            "INSERT INTO {PUBLIC_KEYS_TABLE_NAME} ({PUBLIC_KEYS_KEYS}) VALUES (DEFAULT, $1, $2, $3, $4)"
        );

        let query_result = query(&query_body)
            .bind(&public_key)
            .bind(&client_profile.client_profile_id)
            .bind(&client_profile.created_at)
            .bind(&client_profile.created_at)
            .execute(&mut **tx)
            .await;

        match query_result {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }

    pub async fn update_public_key_last_seen(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        public_key: &String,
    ) -> Result<PublicKey, sqlx::Error> {
        let query = format!(
            "UPDATE {PUBLIC_KEYS_TABLE_NAME} SET last_seen = NOW() WHERE public_key = $1 RETURNING {PUBLIC_KEYS_KEYS}"
        );
        let typed_query = query_as::<_, PublicKey>(&query);

        return typed_query.bind(public_key).fetch_one(&mut **tx).await;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[tokio::test]
    async fn test_handle_public_key_entry() {
        let db = Db::connect_to_the_pool().await;
        db.truncate_all_tables().await.unwrap();

        // Define a public key to test with
        let public_key_str = "test_public_key".to_string();

        // Scenario 1: Insert new client profile and public key
        let client_profile_id = db.handle_public_key_entry(&public_key_str).await.unwrap();
        assert!(client_profile_id == 1);

        // Retrieve the inserted public key to verify
        let inserted_public_key = db.get_public_key(&public_key_str).await.unwrap();
        assert_eq!(inserted_public_key.public_key, public_key_str);
        assert_eq!(inserted_public_key.client_profile_id, client_profile_id);

        // Scenario 2: Update the existing public key's last_seen
        tokio::time::sleep(Duration::from_millis(1000)).await;

        let updated_client_profile_id = db.handle_public_key_entry(&public_key_str).await.unwrap();
        assert_eq!(updated_client_profile_id, client_profile_id);

        // Retrieve the updated public key to verify last_seen has been updated
        let updated_public_key = db.get_public_key(&public_key_str).await.unwrap();
        assert_eq!(updated_public_key.public_key, public_key_str);
        assert!(updated_public_key.last_seen > inserted_public_key.last_seen);
    }
}
