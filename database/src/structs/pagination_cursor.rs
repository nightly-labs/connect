use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::{consts::PAGINATION_PAGE_SIZE, db_error::DbError};

#[derive(Serialize, Deserialize, Debug)]
pub struct PaginationCursor(pub String);

impl PaginationCursor {
    // Decodes the PaginationCursor back into PaginationData
    pub fn decode(&self) -> anyhow::Result<CursorData> {
        let bytes = self.0.as_bytes();
        let pagination_data = serde_json::from_slice(bytes)?;
        Ok(pagination_data)
    }
}

#[derive(PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct CursorData {
    pub last_date_point: DateTime<Utc>,
    pub last_id: i64,
    pub query_data_range: Option<(DateTime<Utc>, DateTime<Utc>)>,
}

impl CursorData {
    pub fn get(
        pagination_cursor: &Option<PaginationCursor>,
    ) -> Result<Option<CursorData>, DbError> {
        match pagination_cursor {
            Some(cursor) => {
                let data = cursor
                    .decode()
                    .map_err(|_| DbError::from("Failed to decode cursor data".to_string()))?;

                Ok(Some(data))
            }
            None => Ok(None),
        }
    }

    pub fn encode(&self) -> anyhow::Result<PaginationCursor> {
        let bytes = serde_json::to_vec(self)?;
        let cursor = String::from_utf8(bytes)?;
        Ok(PaginationCursor(cursor))
    }

    pub fn get_pagination_cursor<T: CursorParams>(
        items: &Vec<T>,
        data_range: Option<(DateTime<Utc>, DateTime<Utc>)>,
    ) -> Result<Option<PaginationCursor>, DbError> {
        if items.is_empty() || items.len() < PAGINATION_PAGE_SIZE as usize {
            return Ok(None);
        }

        if let Some(last_item) = items.last() {
            let data = CursorData {
                last_date_point: last_item.get_date(),
                last_id: last_item.get_id(),
                query_data_range: data_range,
            };

            let encoded_data = data
                .encode()
                .map_err(|_| DbError::from("Failed to encode cursor data".to_string()))?;

            return Ok(Some(encoded_data));
        }

        Ok(None)
    }
}

pub trait CursorParams {
    fn get_date(&self) -> DateTime<Utc>;
    fn get_id(&self) -> i64;
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::tables::utils::get_current_datetime;

    #[test]
    fn test_pagination_cursor() {
        let cursor_data = CursorData {
            last_date_point: get_current_datetime(),
            last_id: 1,
            query_data_range: None,
        };

        let cursor = cursor_data.encode().unwrap();
        let decoded_cursor_data = cursor.decode().unwrap();

        assert_eq!(cursor_data, decoded_cursor_data);
    }
}
