use crate::{
    db::Db,
    structs::{db_error::DbError, event_type::EventType},
    tables::events::events_index::table_struct::{EVENTS_KEYS, EVENTS_TABLE_NAME},
};
use chrono::{DateTime, Utc};
use sqlx::{query, Postgres, Row, Transaction};

impl Db {
    pub async fn create_new_event_entry(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        app_id: &String,
        event_type: &EventType,
        current_timestamp: &DateTime<Utc>,
    ) -> Result<i64, DbError> {
        let query_body = format!(
            "INSERT INTO {EVENTS_TABLE_NAME} ({EVENTS_KEYS}) VALUES (DEFAULT, $1, $2, $3) RETURNING event_id"
        );

        let query_result = query(&query_body)
            .bind(app_id)
            .bind(event_type)
            .bind(current_timestamp)
            .fetch_one(&mut **tx)
            .await;

        match query_result {
            Ok(row) => Ok(row.get("event_id")),
            Err(e) => Err(e).map_err(|e| e.into()),
        }
    }
}

#[cfg(feature = "cloud_integration_tests")]
#[cfg(test)]
mod tests {
    use crate::tables::utils::get_current_datetime;

    use super::*;

    #[tokio::test]
    async fn test_create_new_event_entry() {
        let db = super::Db::connect_to_the_pool().await;
        db.truncate_all_tables().await.unwrap();

        let app_id = "test_app".to_string();
        let event_type = EventType::AppConnect;

        // Create 1001 entries
        let mut tx = db
            .connection_pool
            .begin()
            .await
            .expect("Failed to start transaction");

        for _ in 0..2001 {
            let _ = db
                .create_new_event_entry(&mut tx, &app_id, &event_type, &get_current_datetime())
                .await
                .unwrap();
        }

        // Commit the transaction
        tx.commit().await.expect("Failed to commit transaction");

        // Get events
        let (events, cursor) = db
            .get_events_by_app_id(None, &app_id)
            .await
            .expect("Failed to get events");

        assert_eq!(events.len(), 1000);
        assert!(cursor.is_some());

        // Get the next page
        let (events, cursor) = db
            .get_events_by_app_id(cursor, &app_id)
            .await
            .expect("Failed to get events");

        assert_eq!(events.len(), 1000);
        assert!(cursor.is_some());

        // Get the next page
        let next_page_events = db
            .get_events_by_app_id(cursor, &app_id)
            .await
            .expect("Failed to get next page events");

        assert_eq!(next_page_events.0.len(), 1);
        assert!(next_page_events.1.is_none());
    }
}
