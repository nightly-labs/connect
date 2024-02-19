use super::table_struct::{Request, REQUESTS_KEYS, REQUESTS_TABLE_NAME};
use crate::{db::Db, structs::request_status::RequestStatus};
use sqlx::query;

impl Db {
    pub async fn save_request(&self, request: &Request) -> Result<(), sqlx::Error> {
        let query_body = format!(
            "INSERT INTO {REQUESTS_TABLE_NAME} ({}) VALUES ($1, $2, $3, $4, $5, $6)",
            REQUESTS_KEYS
        );

        let query_result = query(&query_body)
            .bind(&request.request_id)
            .bind(&request.request_type)
            .bind(&request.session_id)
            .bind(&request.request_status)
            .bind(&request.network)
            .bind(&(request.creation_timestamp as i64))
            .execute(&self.connection_pool)
            .await;

        match query_result {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }

    pub async fn update_request_status(
        &self,
        request_id: &String,
        new_status: &RequestStatus,
    ) -> Result<(), sqlx::Error> {
        let query_body =
            format!("UPDATE {REQUESTS_TABLE_NAME} SET request_status = $1 WHERE request_id = $2");
        let query_result = query(&query_body)
            .bind(new_status)
            .bind(request_id)
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

    use super::*;

    #[tokio::test]
    async fn test_requests() {
        let db = super::Db::connect_to_the_pool().await;
        db.truncate_table(REQUESTS_TABLE_NAME).await.unwrap();

        let request = Request {
            request_id: "test_request_id".to_string(),
            request_type: "test_request_type".to_string(),
            session_id: "test_session_id".to_string(),
            request_status: RequestStatus::Pending,
            network: "test_network".to_string(),
            creation_timestamp: 10,
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
            network: "test_network".to_string(),
            creation_timestamp: 12,
        };

        db.save_request(&second_request).await.unwrap();

        let requests = db
            .get_requests_by_session_id(&request.session_id)
            .await
            .unwrap();
        assert_eq!(requests.len(), 2);
        assert_eq!(second_request, requests[0]);
        assert_eq!(request, requests[1]);

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
