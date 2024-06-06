use crate::{
    db::Db,
    structs::{db_error::DbError, device_metadata::DeviceMetadata, medium_type::DeviceMediumType},
    tables::{
        events::app_connect::table_struct::{EVENT_APP_CONNECT_KEYS, EVENT_APP_CONNECT_TABLE_NAME},
        metadata::{
            mobile_metadata::table_struct::DbMobileMetadata,
            web_metadata::table_struct::DbWebMetadata,
        },
    },
};
use chrono::{DateTime, Utc};
use sqlx::Transaction;
use sqlx::{query, Postgres};

impl Db {
    pub async fn create_new_event_app_connect(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        event_id: i64,
        app_id: &String,
        network: &String,
        session_id: &String,
        device_metadata: &DeviceMetadata,
        lang: &String,
        timezone: &String,
        new_session: bool,
        creation_timestamp: &DateTime<Utc>,
    ) -> Result<(), DbError> {
        // Save the device metadata to the corresponding table
        let (device_metadata_type, device_metadata_uuid) = match device_metadata {
            DeviceMetadata::Web(metadata) => {
                let device_metadata_uuid = uuid7::uuid7().to_string();
                let web_metadata = DbWebMetadata {
                    uuid: device_metadata_uuid,
                    browser: metadata.browser.clone(),
                    browser_version: metadata.browser_version.clone(),
                    os: metadata.os.clone(),
                    os_version: metadata.os_version.clone(),
                };

                self.create_new_device_web_metadata_within_tx(tx, &web_metadata)
                    .await?;
                (DeviceMediumType::Browser, Some(web_metadata.uuid.clone()))
            }
            DeviceMetadata::Mobile(metadata) => {
                let device_metadata_uuid = uuid7::uuid7().to_string();
                let mobile_metadata = DbMobileMetadata {
                    uuid: device_metadata_uuid,
                    system_type: metadata.system.to_string().clone(),
                    system_version: metadata.version.clone(),
                };

                self.create_new_device_mobile_metadata_within_tx(tx, &mobile_metadata)
                    .await?;
                (DeviceMediumType::Mobile, Some(mobile_metadata.uuid.clone()))
            }
            DeviceMetadata::Unknown => (DeviceMediumType::Unknown, None),
        };

        // Save the app connect event to the database
        let query_body = format!(
            "INSERT INTO {EVENT_APP_CONNECT_TABLE_NAME} ({EVENT_APP_CONNECT_KEYS}) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)"
        );

        let query_result = query(&query_body)
            .bind(event_id)
            .bind(app_id)
            .bind(network)
            .bind(session_id)
            .bind(device_metadata_type)
            .bind(device_metadata_uuid)
            .bind(lang)
            .bind(timezone)
            .bind(new_session)
            .bind(creation_timestamp)
            .execute(&mut **tx)
            .await;

        match query_result {
            Ok(_) => Ok(()),
            Err(e) => Err(e).map_err(|e| e.into()),
        }
    }
}
