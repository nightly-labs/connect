use super::table_struct::{RegisteredApp, REGISTERED_APPS_KEYS, REGISTERED_APPS_TABLE_NAME};
use crate::{db::Db, structs::subscription::Subscription};
use sqlx::query;

impl Db {
    pub async fn register_new_app(&self, app: &RegisteredApp) -> Result<(), sqlx::Error> {
        let query_body = format!(
            "INSERT INTO {REGISTERED_APPS_TABLE_NAME} ({}) VALUES ($1, $2, $3, $4, $5, $6, $7, $8)",
            REGISTERED_APPS_KEYS
        );

        let query_result = query(&query_body)
            .bind(&app.app_id)
            .bind(&app.app_name)
            .bind(&app.whitelisted_domains)
            .bind(&app.subscription)
            .bind(&app.ack_public_keys)
            .bind(&app.email)
            .bind(&(app.registration_timestamp as i64))
            .bind(&app.pass_hash)
            .execute(&self.connection_pool)
            .await;

        match query_result {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }

    pub async fn update_subscription(
        &self,
        app_id: &String,
        subscription: &Subscription,
    ) -> Result<(), sqlx::Error> {
        let query_body = "UPDATE registered_apps SET subscription = $1 WHERE app_id = $2";
        let query_result = query(query_body)
            .bind(subscription)
            .bind(app_id)
            .execute(&self.connection_pool)
            .await;

        match query_result {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::tables::registered_app::table_struct::{RegisteredApp, REGISTERED_APPS_TABLE_NAME};

    #[tokio::test]
    async fn test_register_app() {
        let db = super::Db::connect_to_the_pool().await;
        db.truncate_table(REGISTERED_APPS_TABLE_NAME).await.unwrap();

        let app = RegisteredApp {
            app_id: "test_app_id".to_string(),
            app_name: "test_app_name".to_string(),
            whitelisted_domains: vec!["test_domain".to_string()],
            subscription: None,
            ack_public_keys: vec!["test_key".to_string()],
            email: None,
            registration_timestamp: 0,
            pass_hash: None,
        };

        db.register_new_app(&app).await.unwrap();

        let result = db.get_registered_app_by_app_id(&app.app_id).await.unwrap();
        assert_eq!(app, result);
    }
}
