use super::table_struct::{Request, REQUESTS_KEYS, REQUESTS_TABLE_NAME};
use crate::{
    db::Db,
    structs::{db_error::DbError, request_status::RequestStatus},
};
use sqlx::query;

impl Db {
    pub async fn save_request(&self, request: &Request) -> Result<(), DbError> {
        let query_body = format!(
            "INSERT INTO {REQUESTS_TABLE_NAME} ({}) VALUES ($1, $2, $3, $4, $5, $6, $7)",
            REQUESTS_KEYS
        );

        let query_result = query(&query_body)
            .bind(&request.request_id)
            .bind(&request.session_id)
            .bind(&request.app_id)
            .bind(&request.request_type)
            .bind(&request.request_status)
            .bind(&request.network)
            .bind(&request.creation_timestamp)
            .execute(&self.connection_pool)
            .await;

        match query_result {
            Ok(_) => Ok(()),
            Err(e) => Err(e).map_err(|e| e.into()),
        }
    }

    pub async fn update_request_status(
        &self,
        request_id: &String,
        new_status: &RequestStatus,
    ) -> Result<(), DbError> {
        let query_body =
            format!("UPDATE {REQUESTS_TABLE_NAME} SET request_status = $1 WHERE request_id = $2");
        let query_result = query(&query_body)
            .bind(new_status)
            .bind(request_id)
            .execute(&self.connection_pool)
            .await;

        match query_result {
            Ok(_) => Ok(()),
            Err(e) => Err(e).map_err(|e| e.into()),
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::{
        structs::session_type::SessionType,
        tables::{sessions::table_struct::DbNcSession, utils::get_date_time},
    };
    use sqlx::types::chrono::Utc;

    #[tokio::test]
    async fn test_requests() {
        let db = super::Db::connect_to_the_pool().await;
        db.truncate_all_tables().await.unwrap();

        // Create test team instance
        let team_id = "test_team_id".to_string();
        let app_id = "test_app_id".to_string();

        db.setup_test_team(&team_id, &app_id, Utc::now())
            .await
            .unwrap();

        // Create basic session to satisfy foreign key constraint
        let session = DbNcSession {
            session_id: "test_session_id".to_string(),
            session_type: SessionType::Relay,
            app_id: "test_app_id".to_string(),
            app_metadata: "test_app_metadata".to_string(),
            persistent: false,
            network: "test_network".to_string(),
            client_data: None,
            session_open_timestamp: get_date_time(10).unwrap(),
            session_close_timestamp: None,
        };

        // Create a new session entry
        db.handle_new_session(&session, &"connection_id".to_string())
            .await
            .unwrap();

        let request = Request {
            request_id: "test_request_id".to_string(),
            request_type: "test_request_type".to_string(),
            session_id: "test_session_id".to_string(),
            request_status: RequestStatus::Pending,
            network: "test_network".to_string(),
            app_id: "test_app_id".to_string(),
            creation_timestamp: get_date_time(10).unwrap(),
        };

        db.save_request(&request).await.unwrap();

        let requests = db
            .get_requests_by_session_id(&request.session_id)
            .await
            .unwrap();
        assert_eq!(requests.len(), 1);
        assert_eq!(request, requests[0]);

        let second_request = Request {
            request_id: "test_request_id2".to_string(),
            request_type: "test_request_type".to_string(),
            session_id: "test_session_id".to_string(),
            request_status: RequestStatus::Pending,
            app_id: "test_app_id".to_string(),
            network: "test_network".to_string(),
            creation_timestamp: get_date_time(10).unwrap(),
        };

        db.save_request(&second_request).await.unwrap();

        let requests = db
            .get_requests_by_session_id(&request.session_id)
            .await
            .unwrap();
        assert_eq!(requests.len(), 2);
        assert_eq!(request, requests[0]);
        assert_eq!(second_request, requests[1]);

        db.update_request_status(&request.request_id, &RequestStatus::Completed)
            .await
            .unwrap();

        let request = db
            .get_request_by_request_id(&request.request_id)
            .await
            .unwrap()
            .unwrap();

        assert_eq!(request.request_status, RequestStatus::Completed);
    }
}
