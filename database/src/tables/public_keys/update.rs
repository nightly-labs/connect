use crate::db::Db;
use crate::tables::client_profiles::table_struct::ClientProfile;
use crate::tables::public_keys::table_struct::{
    PublicKey, PUBLIC_KEYS_KEYS, PUBLIC_KEYS_TABLE_NAME,
};
use sqlx::{query, Postgres};
use sqlx::{query_as, Transaction};
use std::collections::HashMap;

impl Db {
    pub async fn handle_public_keys_entries(
        &self,
        mut tx: &mut Transaction<'_, Postgres>,
        public_keys: &Vec<String>,
    ) -> Result<i64, sqlx::Error> {
        // Single key: straightforward client_profile_id resolution.
        if public_keys.len() == 1 {
            let public_key = &public_keys[0];
            if let Ok(key) = self.update_public_key_last_seen(&mut tx, &public_key).await {
                {
                    match key.target_client_profile_id {
                        Some(profile_id) => return Ok(profile_id),
                        None => return Ok(key.origin_client_profile_id),
                    }
                };
            }

            // This should not fail, but just in case
            let client_profile = match self.create_new_profile(Some(&mut tx)).await {
                Ok(profile) => profile,
                Err(e) => {
                    return Err(e);
                }
            };

            // Create a new public key entry
            if let Err(e) = self
                .create_new_public_key(&mut tx, &public_key, &client_profile)
                .await
            {
                return Err(e);
            }
        }

        // Multiple keys: resolving potential conflicts in client_profile_id.
        let mut profile_ids = HashMap::new();
        for key in public_keys {
            if let Ok(public_key) = self.get_public_key(&key).await {
                profile_ids.insert(
                    key.clone(),
                    (
                        public_key.origin_client_profile_id,
                        public_key.target_client_profile_id,
                    ),
                );
            }
        }

        match profile_ids.len() {
            0 => {
                // No existing profiles were found for any keys; create a new profile.
                let new_profile = self.create_new_profile(Some(tx)).await?;
                for key in public_keys {
                    self.create_new_public_key(tx, key, &new_profile).await?;
                }
                Ok(new_profile.client_profile_id)
            }
            1 => {
                // All keys map to the same existing profile.

                // Update the last_seen timestamp for each key.
                for key in public_keys {
                    self.update_public_key_last_seen(tx, key).await?;
                }

                // Return the profile ID. We know there's only one.
                let profile_entry = profile_ids.values().next().unwrap();
                match profile_entry.1 {
                    Some(profile_id) => Ok(profile_id),
                    None => Ok(profile_entry.0),
                }
            }
            _ => {
                // Keys map to multiple profiles; resolve which one to use. Take the one with the lowest profile ID.
                let target_profile_id = profile_ids
                    .iter()
                    .flat_map(|(_key, &(value, opt_value))| {
                        std::iter::once(value).chain(opt_value.into_iter())
                    })
                    .min()
                    .unwrap();

                // Perform few actions on each key
                for (key, (origin_client_profile_id, current_target_client_profile_id)) in
                    profile_ids
                {
                    // Skip the target profile or if key target is already set to the target profile.
                    if origin_client_profile_id == target_profile_id
                        || current_target_client_profile_id == Some(target_profile_id)
                    {
                        // Still update last_seen for the target profile.
                        self.update_public_key_last_seen(tx, &key).await?;

                        continue;
                    }

                    // 1. Update the last_seen timestamp.
                    self.update_public_key_last_seen(tx, &key).await?;

                    // 2. Update the pointer to the target profile.
                    self.update_client_profile_pointer(tx, &key, &target_profile_id)
                        .await?;

                    // 3. Update public key's target_client_profile_id.
                    self.update_client_profile_pointer(tx, &key, &target_profile_id)
                        .await?;

                    // 4. Create log entry for the merge.
                    self.log_client_profile_merge_event(
                        tx,
                        origin_client_profile_id,
                        current_target_client_profile_id,
                        target_profile_id,
                    )
                    .await?;
                }

                Ok(target_profile_id)
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

    pub async fn update_client_profile_pointer(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        pub_key: &String,
        new_client_profile_id: &i64,
    ) -> Result<(), sqlx::Error> {
        let query_body = format!(
            "UPDATE {PUBLIC_KEYS_TABLE_NAME} SET target_client_profile_id = $1 WHERE pub_key = $2"
        );

        let query_result = query(&query_body)
            .bind(new_client_profile_id)
            .bind(pub_key)
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
    use std::time::Duration;

    #[tokio::test]
    async fn test_handle_public_key_entry() {
        let db = Db::connect_to_the_pool().await;
        db.truncate_all_tables().await.unwrap();

        // Define a public key to test with
        let public_key_str = "test_public_key".to_string();

        // Scenario 1: Insert new client profile and public key
        let mut tx = db.connection_pool.begin().await.unwrap();
        let client_profile_id = db
            .handle_public_keys_entries(&mut tx, &vec![public_key_str.clone()])
            .await
            .unwrap();
        tx.commit().await.unwrap();
        assert!(client_profile_id == 1);

        // Retrieve the inserted public key to verify
        let inserted_public_key = db.get_public_key(&public_key_str).await.unwrap();
        assert_eq!(inserted_public_key.public_key, public_key_str);
        assert_eq!(
            inserted_public_key.origin_client_profile_id,
            client_profile_id
        );
        assert_eq!(inserted_public_key.target_client_profile_id, None);

        // Scenario 2: Update the existing public key's last_seen
        tokio::time::sleep(Duration::from_millis(1000)).await;

        let mut tx = db.connection_pool.begin().await.unwrap();
        let updated_client_profile_id = db
            .handle_public_keys_entries(&mut tx, &vec![public_key_str.clone()])
            .await
            .unwrap();
        tx.commit().await.unwrap();
        assert_eq!(updated_client_profile_id, client_profile_id);

        // Retrieve the updated public key to verify last_seen has been updated
        let updated_public_key = db.get_public_key(&public_key_str).await.unwrap();
        assert_eq!(updated_public_key.public_key, public_key_str);
        assert!(updated_public_key.last_seen > inserted_public_key.last_seen);
    }
}
