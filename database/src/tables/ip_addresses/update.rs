use crate::{
    db::Db,
    structs::db_error::DbError,
    tables::ip_addresses::table_struct::{
        IpAddressEntry, IP_ADDRESSES_KEYS, IP_ADDRESSES_TABLE_NAME,
    },
};
use sqlx::query;

impl Db {
    pub async fn upsert_ip_address(&self, ip_address: &IpAddressEntry) -> Result<(), DbError> {
        let query_body = format!(
            "INSERT INTO {IP_ADDRESSES_TABLE_NAME} ({IP_ADDRESSES_KEYS}) VALUES ($1, $2, $3, $4, $5, $6)
             ON CONFLICT (ip_addr) DO UPDATE SET 
             last_updated_at = EXCLUDED.last_updated_at, 
             country = EXCLUDED.country, 
             city = EXCLUDED.city, 
             lat = EXCLUDED.lat, 
             lon = EXCLUDED.lon"
        );

        let query_result = query(&query_body)
            .bind(&ip_address.ip_addr)
            .bind(&ip_address.last_updated_at)
            .bind(&ip_address.country)
            .bind(&ip_address.city)
            .bind(&ip_address.lat)
            .bind(&ip_address.lon)
            .execute(&self.connection_pool)
            .await;

        match query_result {
            Ok(_) => Ok(()),
            Err(e) => Err(e).map_err(|e| e.into()),
        }
    }
}

#[cfg(feature = "cloud_db_tests")]
#[cfg(test)]
mod tests {

    use crate::tables::utils::get_current_datetime;

    use super::*;
    use std::time::Duration;

    #[tokio::test]
    async fn test_handle_ip_address_entry() {
        let db = Db::connect_to_the_pool().await;
        db.truncate_all_tables().await.unwrap();

        // Define IP addresses to test with
        let first_ip_address = "24.48.0.1".to_string();
        let second_ip_address = "98.137.27.103".to_string();

        // Define country and city for testing
        let country = "TestCountry".to_string();
        let city = "TestCity".to_string();
        let lat = 1.234;
        let lon = 5.678;

        // Scenario 1: Insert new IP address
        let test_ip_address = IpAddressEntry {
            ip_addr: first_ip_address.clone(),
            last_updated_at: get_current_datetime(),
            country: Some(country.clone()),
            city: Some(city.clone()),
            lat: Some(lat),
            lon: Some(lon),
        };
        db.upsert_ip_address(&test_ip_address).await.unwrap();

        // Retrieve the inserted IP address to verify
        let inserted_ip_address = db.get_ip_address(&first_ip_address).await.unwrap().unwrap();
        assert_eq!(inserted_ip_address.ip_addr, first_ip_address);
        assert_eq!(inserted_ip_address.country, Some(country.clone()));
        assert_eq!(inserted_ip_address.city, Some(city.clone()));

        // Scenario 2: Update the existing IP address
        tokio::time::sleep(Duration::from_millis(1000)).await;

        let updated_ip_address = IpAddressEntry {
            ip_addr: first_ip_address.clone(),
            last_updated_at: get_current_datetime(), // Updated timestamp
            country: Some(country.clone()),
            city: Some(city.clone()),
            lat: Some(lat),
            lon: Some(lon),
        };
        db.upsert_ip_address(&updated_ip_address).await.unwrap();

        let updated_ip_data = db.get_ip_address(&first_ip_address).await.unwrap().unwrap();
        assert_eq!(updated_ip_data.ip_addr, first_ip_address);
        assert!(updated_ip_data.last_updated_at > inserted_ip_address.last_updated_at);

        // Scenario 3: Insert a new distinct IP address
        let new_ip_address = IpAddressEntry {
            ip_addr: second_ip_address.clone(),
            last_updated_at: get_current_datetime(),
            country: Some("AnotherCountry".to_string()),
            city: Some("AnotherCity".to_string()),
            lat: Some(9.876),
            lon: Some(5.432),
        };
        db.upsert_ip_address(&new_ip_address).await.unwrap();

        let second_ip_data = db
            .get_ip_address(&second_ip_address)
            .await
            .unwrap()
            .unwrap();
        assert_eq!(second_ip_data.ip_addr, second_ip_address);
        assert_ne!(second_ip_data.country, inserted_ip_address.country);
    }
}
