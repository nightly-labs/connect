use crate::db::Db;
use crate::structs::db_error::DbError;
use crate::tables::client_profiles::table_struct::ClientProfile;
use crate::tables::public_keys::table_struct::{
    PublicKey, PUBLIC_KEYS_KEYS, PUBLIC_KEYS_TABLE_NAME,
};
use sqlx::{query, Postgres};
use sqlx::{query_as, Transaction};

impl Db {
    pub async fn handle_public_keys_entries(
        &self,
        mut tx: &mut Transaction<'_, Postgres>,
        public_keys: &Vec<String>,
    ) -> Result<(i64, String), DbError> {
        // Always take the first key as the reference key.
        let public_key = &public_keys[0];

        // 1. Check if the public key already exists.
        if let Ok(key) = self.update_public_key_last_seen(&mut tx, &public_key).await {
            return Ok((key.client_profile_id, public_key.clone()));
        }

        // 2. Key was not used before, create a new profile and public key entry.
        // This should not fail, but just in case
        let client_profile = match self.create_new_profile(Some(&mut tx)).await {
            Ok(profile) => profile,
            Err(e) => {
                return Err(e);
            }
        };

        // 3. Create a new public key entry
        if let Err(err) = self
            .create_new_public_key(&mut tx, &public_key, &client_profile)
            .await
        {
            return Err(err);
        }

        return Ok((client_profile.client_profile_id, public_key.clone()));
    }

    pub async fn create_new_public_key(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        public_key: &String,
        client_profile: &ClientProfile,
    ) -> Result<(), DbError> {
        let query_body = format!(
            "INSERT INTO {PUBLIC_KEYS_TABLE_NAME} ({PUBLIC_KEYS_KEYS}) VALUES ($1, $2, $3, $4)"
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
            Err(e) => Err(e).map_err(|e| e.into()),
        }
    }

    pub async fn update_public_key_last_seen(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        public_key: &String,
    ) -> Result<PublicKey, DbError> {
        let query = format!(
            "UPDATE {PUBLIC_KEYS_TABLE_NAME} SET last_seen = NOW() WHERE public_key = $1 RETURNING {PUBLIC_KEYS_KEYS}"
        );
        let typed_query = query_as::<_, PublicKey>(&query);

        return typed_query
            .bind(public_key)
            .fetch_one(&mut **tx)
            .await
            .map_err(|e| e.into());
    }
}

#[cfg(feature = "cloud_integration_tests")]
#[cfg(test)]
mod tests {

    use super::*;
    use std::time::Duration;

    #[tokio::test]
    async fn test_handle_public_key_entry() {
        let db = Db::connect_to_the_pool().await;
        db.truncate_all_tables().await.unwrap();

        // Define a public key to test with
        let first_public_key = "test_public_key".to_string();
        let second_public_key = "test_public_key_1".to_string();
        let third_public_key = "test_public_key_2".to_string();

        // Define a client profile id to test with
        let first_client_profile_id = 1;
        let second_client_profile_id = 2;

        // Scenario 1: Insert new client profile and public key
        let mut tx = db.connection_pool.begin().await.unwrap();
        let (client_profile_id, public_key) = db
            .handle_public_keys_entries(&mut tx, &vec![first_public_key.clone()])
            .await
            .unwrap();
        tx.commit().await.unwrap();
        assert!(client_profile_id == first_client_profile_id);

        // Retrieve the inserted public key to verify
        let inserted_public_key = db.get_public_key(&first_public_key).await.unwrap();
        assert_eq!(inserted_public_key.public_key, first_public_key);
        assert_eq!(first_public_key, public_key);
        assert_eq!(inserted_public_key.client_profile_id, client_profile_id);

        // Scenario 2: Update the existing public key's last_seen
        tokio::time::sleep(Duration::from_millis(1000)).await;

        let mut tx = db.connection_pool.begin().await.unwrap();
        let (updated_client_profile_id, public_key) = db
            .handle_public_keys_entries(&mut tx, &vec![first_public_key.clone()])
            .await
            .unwrap();
        tx.commit().await.unwrap();

        assert_eq!(updated_client_profile_id, client_profile_id);
        assert_eq!(first_public_key, public_key);

        // Retrieve the updated public key to verify last_seen has been updated
        let first_pub_updated = db.get_public_key(&first_public_key).await.unwrap();
        assert_eq!(first_pub_updated.public_key, first_public_key);
        assert_eq!(first_pub_updated.public_key, public_key);
        assert!(first_pub_updated.last_seen > inserted_public_key.last_seen);

        // Scenario 3: Use multiple public keys
        let mut tx = db.connection_pool.begin().await.unwrap();
        let (client_profile_id, public_key) = db
            .handle_public_keys_entries(
                &mut tx,
                &vec![second_public_key.clone(), first_public_key.clone()],
            )
            .await
            .unwrap();
        tx.commit().await.unwrap();

        assert!(client_profile_id == second_client_profile_id);
        assert_eq!(public_key, second_public_key);

        // Retrieve the inserted public key to verify
        let second_pub_updated = db.get_public_key(&public_key).await.unwrap();
        assert_eq!(second_pub_updated.public_key, public_key);
        assert_eq!(second_pub_updated.client_profile_id, client_profile_id);

        // Retrieve the public key entry to verify if it was updated, it should not be updated
        let not_updated_first_public_key = db.get_public_key(&first_public_key).await.unwrap();
        assert_eq!(
            not_updated_first_public_key.last_seen,
            first_pub_updated.last_seen
        );

        // Scenario 4: use multiple keys but in reversed order
        let mut tx = db.connection_pool.begin().await.unwrap();
        let (client_profile_id, public_key) = db
            .handle_public_keys_entries(
                &mut tx,
                &vec![
                    first_public_key.clone(),
                    second_public_key.clone(),
                    // Throw in a new key
                    third_public_key.clone(),
                ],
            )
            .await
            .unwrap();

        tx.commit().await.unwrap();

        assert!(client_profile_id == first_client_profile_id);
        assert_eq!(public_key, first_public_key);

        // Retrieve the data for all of the keys to verify
        let first_public_key_data = db.get_public_key(&first_public_key).await.unwrap();
        // Last seen should have been updated since the scenario 3
        assert!(first_public_key_data.last_seen > first_pub_updated.last_seen);
        assert!(first_public_key_data.client_profile_id == first_client_profile_id);

        let second_public_key_data = db.get_public_key(&second_public_key).await.unwrap();
        // Last seen should be the same as the time from scenario 3
        assert_eq!(
            second_public_key_data.last_seen,
            second_pub_updated.last_seen
        );
        assert!(second_public_key_data.client_profile_id == second_client_profile_id);

        // This should fail, while this key was used in the previous scenario, it was not used as the main key
        db.get_public_key(&third_public_key).await.unwrap_err();
    }
}
