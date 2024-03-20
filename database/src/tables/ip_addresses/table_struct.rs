use sqlx::{
    postgres::PgRow,
    types::chrono::{DateTime, Utc},
    FromRow, Row,
};

pub const IP_ADDRESSES_TABLE_NAME: &str = "ip_addresses";
pub const IP_ADDRESSES_KEYS: &str = "ip_addr, last_updated_at, country, city, lat, lon";

#[derive(Clone, Debug)]
pub struct IpAddressEntry {
    pub ip_addr: String,
    pub last_updated_at: DateTime<Utc>,
    pub country: Option<String>,
    pub city: Option<String>,
    pub lat: Option<f64>,
    pub lon: Option<f64>,
}

impl FromRow<'_, PgRow> for IpAddressEntry {
    fn from_row(row: &sqlx::postgres::PgRow) -> std::result::Result<Self, sqlx::Error> {
        Ok(IpAddressEntry {
            ip_addr: row.get("ip_addr"),
            last_updated_at: row.get("last_updated_at"),
            city: row.get("city"),
            country: row.get("country"),
            lat: row.get("lat"),
            lon: row.get("lon"),
        })
    }
}
