use super::table_struct::{RegisteredApp, REGISTERED_APPS_TABLE_NAME};
use crate::structs::filter_requests::AggregatedRequestCount;
use crate::structs::time_filters::TimeFilter;
use crate::tables::requests::table_struct::REQUESTS_TABLE_NAME;
use crate::{db::Db, tables::requests::table_struct::Request};
use sqlx::{query_as, Error};

impl Db {
    pub async fn get_registered_app_by_app_id(
        &self,
        app_id: &String,
    ) -> Result<RegisteredApp, sqlx::Error> {
        let query = format!("SELECT * FROM {REGISTERED_APPS_TABLE_NAME} WHERE app_id = $1");
        let typed_query = query_as::<_, RegisteredApp>(&query);

        return typed_query
            .bind(&app_id)
            .fetch_one(&self.connection_pool)
            .await;
    }

    pub async fn get_requests_by_app_id(
        &self,
        app_id: &String,
    ) -> Result<Vec<Request>, sqlx::Error> {
        let query = format!(
            "SELECT r.* FROM {REQUESTS_TABLE_NAME} r 
            INNER JOIN sessions s ON r.session_id = s.session_id 
            WHERE s.app_id = $1
            ORDER BY r.creation_timestamp DESC"
        );
        let typed_query = query_as::<_, Request>(&query);

        return typed_query
            .bind(&app_id)
            .fetch_all(&self.connection_pool)
            .await;
    }

    pub async fn get_aggregated_requests_by_app_id(
        &self,
        app_id: &str,
        filter: TimeFilter,
    ) -> Result<Vec<AggregatedRequestCount>, Error> {
        let start_date = filter.to_date();
        let bucket_size = filter.bucket_size();

        // Correctly selecting the view based on the bucket_size
        let (view_name, bucket, request_count) = match bucket_size {
            "1 hour" => (
                "hourly_requests_per_app",
                "hourly_bucket",
                "hourly_request_count",
            ),
            "1 day" => (
                "daily_requests_per_app",
                "daily_bucket",
                "daily_request_count",
            ),
            // for now return WorkerCrashed but later create custom error
            _ => return Err(Error::WorkerCrashed),
        };

        let query = format!(
            "SELECT app_id, {} as bucket, {} as request_count
            FROM {}
            WHERE app_id = $1 AND {} >= $2
            ORDER BY {} DESC",
            bucket, request_count, view_name, bucket, bucket
        );

        sqlx::query_as::<_, AggregatedRequestCount>(&query)
            .bind(app_id)
            .bind(start_date)
            .fetch_all(&self.connection_pool)
            .await
    }
}
