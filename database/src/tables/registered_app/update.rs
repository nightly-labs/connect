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
    use crate::{
        structs::{
            consts::{LAST_24_HOURS, LAST_30_DAYS, LAST_7_DAYS},
            request_status::RequestStatus,
        },
        tables::{
            registered_app::table_struct::{RegisteredApp, REGISTERED_APPS_TABLE_NAME},
            requests::table_struct::{Request, REQUESTS_TABLE_NAME},
            sessions::table_struct::{DbNcSession, SESSIONS_TABLE_NAME},
            utils::get_timestamp_in_milliseconds,
        },
    };

    #[tokio::test]
    async fn test_register_app() {
        let db = super::Db::connect_to_the_pool().await;
        db.truncate_table(
            format!(
                "{},{},{} CASCADE",
                REGISTERED_APPS_TABLE_NAME, SESSIONS_TABLE_NAME, REQUESTS_TABLE_NAME
            )
            .as_str(),
        )
        .await
        .unwrap();

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

    #[tokio::test]
    async fn test_get_requests() {
        let db = super::Db::connect_to_the_pool().await;
        db.truncate_table(
            format!(
                "{},{},{} CASCADE",
                REGISTERED_APPS_TABLE_NAME, SESSIONS_TABLE_NAME, REQUESTS_TABLE_NAME
            )
            .as_str(),
        )
        .await
        .unwrap();

        // "Register" an app
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

        // Create 2 sessions
        let session = DbNcSession {
            session_id: "test_session_id".to_string(),
            app_id: "test_app_id".to_string(),
            app_metadata: "test_app_metadata".to_string(),
            app_connection_address: "test_app_connection_address".to_string(),
            persistent: false,
            network: "test_network".to_string(),
            client: None,
            session_open_timestamp: 10,
            session_close_timestamp: None,
        };

        let second_session = DbNcSession {
            session_id: "test_session_id_2".to_string(),
            app_id: "test_app_id".to_string(),
            app_metadata: "test_app_metadata".to_string(),
            app_connection_address: "test_app_connection_address".to_string(),
            persistent: false,
            network: "test_network".to_string(),
            client: None,
            session_open_timestamp: 12,
            session_close_timestamp: None,
        };

        db.save_new_session(&session).await.unwrap();
        db.save_new_session(&second_session).await.unwrap();

        let result = db.get_sessions_by_app_id(&app.app_id).await.unwrap();
        assert_eq!(result.len(), 2);
        assert_eq!(second_session, result[0]);
        assert_eq!(session, result[1]);

        // Create 2 requests per session
        // First session
        let request = Request {
            request_id: "test_request_id".to_string(),
            session_id: "test_session_id".to_string(),
            network: "test_network".to_string(),
            creation_timestamp: 10,
            request_status: RequestStatus::Pending,
            request_type: "test_request_type".to_string(),
        };

        let second_request = Request {
            request_id: "test_request_id_2".to_string(),
            session_id: "test_session_id".to_string(),
            network: "test_network".to_string(),
            creation_timestamp: 12,
            request_status: RequestStatus::Pending,
            request_type: "test_request_type".to_string(),
        };

        db.save_request(&request).await.unwrap();
        db.save_request(&second_request).await.unwrap();

        // Second session
        let third_request = Request {
            request_id: "test_request_id_3".to_string(),
            session_id: "test_session_id_2".to_string(),
            network: "test_network".to_string(),
            creation_timestamp: 14,
            request_status: RequestStatus::Pending,
            request_type: "test_request_type".to_string(),
        };

        let fourth_request = Request {
            request_id: "test_request_id_4".to_string(),
            session_id: "test_session_id_2".to_string(),
            network: "test_network".to_string(),
            creation_timestamp: 16,
            request_status: RequestStatus::Pending,
            request_type: "test_request_type".to_string(),
        };

        db.save_request(&third_request).await.unwrap();
        db.save_request(&fourth_request).await.unwrap();

        // Get all requests by app_id
        let result = db.get_requests_by_app_id(&app.app_id).await.unwrap();
        assert_eq!(result.len(), 4);

        assert_eq!(result[0], fourth_request);
        assert_eq!(result[1], third_request);
        assert_eq!(result[2], second_request);
        assert_eq!(result[3], request);
    }

    #[tokio::test]
    async fn test_data_ranges() {
        let db = super::Db::connect_to_the_pool().await;
        db.truncate_table(
            format!(
                "{},{},{} CASCADE",
                REGISTERED_APPS_TABLE_NAME, SESSIONS_TABLE_NAME, REQUESTS_TABLE_NAME
            )
            .as_str(),
        )
        .await
        .unwrap();

        // "Register" an app
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

        // Create session
        let session = DbNcSession {
            session_id: "test_session_id".to_string(),
            app_id: "test_app_id".to_string(),
            app_metadata: "test_app_metadata".to_string(),
            app_connection_address: "test_app_connection_address".to_string(),
            persistent: false,
            network: "test_network".to_string(),
            client: None,
            session_open_timestamp: 10,
            session_close_timestamp: None,
        };

        db.save_new_session(&session).await.unwrap();

        let result = db.get_sessions_by_app_id(&app.app_id).await.unwrap();
        assert_eq!(result.len(), 1);
        assert_eq!(session, result[0]);

        let now = get_timestamp_in_milliseconds();
        // Create requests across last 33 days, 3 requests per day
        for i in 0..33 {
            for j in 0..3 {
                let request = Request {
                    request_id: format!("test_request_id_{}_{}", i, j),
                    session_id: "test_session_id".to_string(),
                    network: "test_network".to_string(),
                    creation_timestamp: (now - (i * 24 * 60 * 60 * 1000)) as u64,
                    request_status: RequestStatus::Pending,
                    request_type: "test_request_type".to_string(),
                };

                db.save_request(&request).await.unwrap();
            }
        }

        // Query last 30 days
        let result = db
            .get_requests_by_app_id_with_filter(&app.app_id, LAST_30_DAYS)
            .await
            .unwrap();
        assert_eq!(result.len(), 30 * 3);

        // Query last 7 days
        let result = db
            .get_requests_by_app_id_with_filter(&app.app_id, LAST_7_DAYS)
            .await
            .unwrap();
        assert_eq!(result.len(), 7 * 3);

        // Query last 24 hours
        let result = db
            .get_requests_by_app_id_with_filter(&app.app_id, LAST_24_HOURS)
            .await
            .unwrap();
        assert_eq!(result.len(), 3);
    }
}
