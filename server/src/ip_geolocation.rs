use anyhow::{bail, Result};
use database::{
    structs::geo_location::Geolocation, tables::ip_addresses::table_struct::IpAddressEntry,
};
use reqwest::Client;
use serde::{Deserialize, Serialize};

pub struct GeolocationRequester {
    pub reqwest_client: Client,
}

impl GeolocationRequester {
    pub async fn new() -> GeolocationRequester {
        GeolocationRequester {
            reqwest_client: Client::new(),
        }
    }
    pub async fn get_geolocation(&self, ip: &str) -> Result<IpGeolocation> {
        let url = format!("http://ip-api.com/json/{ip}?fields=message,country,city,lat,lon,query");

        let response = self.reqwest_client.get(&url).send().await?;

        if response.status().is_success() {
            let ip_geolocation = response.json::<IpGeolocation>().await?;

            // Request might return message in case of error
            if let Some(message) = ip_geolocation.message {
                bail!("Failed to get geolocation, message: {}", message);
            }

            Ok(ip_geolocation)
        } else {
            bail!("Failed to get geolocation, status: {}", response.status())
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IpGeolocation {
    pub query: String,
    pub message: Option<String>,
    pub country: Option<String>,
    pub city: Option<String>,
    pub lat: Option<f64>,
    pub lon: Option<f64>,
}

impl Into<Geolocation> for IpGeolocation {
    fn into(self) -> Geolocation {
        Geolocation {
            country: self.country,
            city: self.city,
            lat: self.lat,
            lon: self.lon,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[ignore = "for manual testing only"]
    #[tokio::test]
    async fn test_get_geolocation() {
        let geolocation_requester = GeolocationRequester::new().await;
        let ip = "24.48.0.1";

        match geolocation_requester.get_geolocation(ip).await {
            Ok(geolocation) => {
                println!("Geolocation: {:?}", geolocation);
            }
            Err(err) => {
                println!("Failed to get geolocation, err: {}", err);
            }
        }
    }
}
