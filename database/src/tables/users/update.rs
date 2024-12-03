use super::table_struct::{USERS_KEYS, USERS_TABLE_NAME};
use crate::db::Db;
use crate::structs::db_error::DbError;
use crate::tables::utils::get_current_datetime;
use sqlx::{query, Transaction};
use webauthn_rs::prelude::Passkey;

impl Db {
    pub async fn add_new_user(
        &self,
        user_id: &String,
        email: &String,
        password_hash: Option<&String>,
        passkey: Option<&Passkey>,
    ) -> Result<(), DbError> {
        let query_body = format!(
            "INSERT INTO {USERS_TABLE_NAME} ({USERS_KEYS}) VALUES ($1, $2, $3, $4, $5, NULL)"
        );

        let passkey = match passkey {
            Some(passkey) => {
                let serialized_passkey =
                    serde_json::to_string(&vec![passkey.clone()]).map_err(|e| {
                        DbError::DatabaseError(format!(
                            "Failed to serialize passkey: {}",
                            e.to_string()
                        ))
                    })?;

                Some(serialized_passkey)
            }
            None => None,
        };
        let query_result = query(&query_body)
            .bind(&user_id)
            .bind(&email)
            .bind(&password_hash)
            .bind(&passkey)
            .bind(&get_current_datetime())
            .execute(&self.connection_pool)
            .await;

        match query_result {
            Ok(_) => Ok(()),
            Err(e) => Err(e).map_err(|e| e.into()),
        }
    }

    pub async fn set_new_password(
        &self,
        user_email: &String,
        new_password: &String,
    ) -> Result<(), DbError> {
        let query_body =
            format!("UPDATE {USERS_TABLE_NAME} SET password_hash = $1 WHERE email = $2 AND deactivated_at IS NULL");

        let query_result = query(&query_body)
            .bind(new_password)
            .bind(user_email)
            .execute(&self.connection_pool)
            .await;

        match query_result {
            Ok(_) => Ok(()),
            Err(e) => Err(e).map_err(|e| e.into()),
        }
    }

    pub async fn update_passkeys(
        &self,
        user_email: &String,
        passkeys: &Vec<Passkey>,
    ) -> Result<(), DbError> {
        let serialized_passkey = serde_json::to_string(passkeys).map_err(|e| {
            DbError::DatabaseError(format!("Failed to serialize passkey: {}", e.to_string()))
        })?;

        let query_body = format!("UPDATE {USERS_TABLE_NAME} SET passkeys = $1 WHERE email = $2 AND deactivated_at IS NULL");

        let query_result = query(&query_body)
            .bind(&serialized_passkey)
            .bind(user_email)
            .execute(&self.connection_pool)
            .await;

        match query_result {
            Ok(_) => Ok(()),
            Err(e) => Err(e).map_err(|e| e.into()),
        }
    }

    pub async fn deactivate_user(
        &self,
        user_id: &String,
        tx: &mut Transaction<'_, sqlx::Postgres>,
    ) -> Result<(), DbError> {
        let query_body = format!(
            "UPDATE {USERS_TABLE_NAME} SET deactivated_at = $1 WHERE user_id = $2 AND deactivated_at IS NULL"
        );

        let query_result = query(&query_body)
            .bind(&get_current_datetime())
            .bind(user_id)
            .execute(&mut **tx)
            .await;

        match query_result {
            Ok(_) => Ok(()),
            Err(e) => Err(e).map_err(|e| e.into()),
        }
    }
}

#[cfg(feature = "cloud_integration_tests")]
#[cfg(test)]
mod tests {
    use crate::tables::{
        test_utils::test_utils::to_microsecond_precision, users::table_struct::User,
    };
    use sqlx::types::chrono::Utc;

    #[tokio::test]
    async fn test_create_user() {
        let db = super::Db::connect_to_the_pool().await;
        db.truncate_all_tables().await.unwrap();

        // Create test team instance
        let team_id = "test_team_id".to_string();
        let app_id = "test_app_id".to_string();

        db.setup_test_team(&team_id, &app_id, Utc::now())
            .await
            .unwrap();

        let password = "test_password_hash".to_string();
        let user = User {
            email: "test_user_email".to_string(),
            password_hash: Some(password.clone()),
            user_id: "test_user_id".to_string(),
            passkeys: None,
            creation_timestamp: to_microsecond_precision(&Utc::now()),
            deactivated_at: None,
        };

        db.add_new_user(&user.user_id, &user.email, Some(&password), None)
            .await
            .unwrap();

        let user_result = db
            .get_user_by_user_id(&user.user_id)
            .await
            .unwrap()
            .unwrap();

        assert_eq!(user.email, user_result.email);
        assert_eq!(user.password_hash, user_result.password_hash);
        assert_eq!(user.user_id, user_result.user_id);
        assert_eq!(user.passkeys, user_result.passkeys);
    }
}
