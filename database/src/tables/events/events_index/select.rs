use super::table_struct::Event;
use crate::structs::consts::PAGINATION_PAGE_SIZE;
use crate::structs::pagination_cursor::CursorData;
use crate::{
    db::Db,
    structs::{db_error::DbError, pagination_cursor::PaginationCursor},
    tables::events::events_index::table_struct::EVENTS_TABLE_NAME,
};
use sqlx::query_as;

impl Db {
    pub async fn get_events_by_app_id(
        &self,
        cursor: Option<PaginationCursor>,
        app_id: &String,
    ) -> Result<(Vec<Event>, Option<PaginationCursor>), DbError> {
        let cursor_data = CursorData::get(&cursor)?;

        // Determine the query body based on whether a cursor is provided
        let query_body: String = match cursor_data {
            Some(_) => {
                let query = format!(
                    "SELECT * FROM {EVENTS_TABLE_NAME} 
                        WHERE app_id = $1 AND (creation_timestamp, event_id) < ($2, $3) 
                        ORDER BY creation_timestamp DESC, event_id DESC LIMIT $4",
                );

                query
            }
            None => {
                let query = format!(
                    "SELECT * FROM {EVENTS_TABLE_NAME} 
                        WHERE app_id = $1 
                        ORDER BY creation_timestamp DESC, event_id DESC LIMIT $2",
                );

                query
            }
        };

        // Build the query and bind the parameters based on whether a cursor is provided
        let mut query = query_as::<_, Event>(&query_body);

        query = if let Some(cursor_data) = cursor_data {
            query
                .bind(app_id)
                .bind(cursor_data.last_date_point)
                .bind(cursor_data.last_id)
                .bind(PAGINATION_PAGE_SIZE)
        } else {
            query.bind(app_id).bind(PAGINATION_PAGE_SIZE)
        };

        // Execute the query
        let events: Vec<Event> = query
            .fetch_all(&self.connection_pool)
            .await
            .map_err(|_| DbError::from("Failed to get events by app id"))?;

        // Get the pagination cursor
        let cursor = CursorData::get_pagination_cursor(&events, None)?;

        Ok((events, cursor))
    }
}
