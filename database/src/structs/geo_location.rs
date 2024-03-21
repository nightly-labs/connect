use sqlx::Type;

#[derive(Clone, Debug, PartialEq, Type)]
#[sqlx(type_name = "geo_location")]
pub struct Geolocation {
    pub country: Option<String>,
    pub city: Option<String>,
    pub lat: Option<f64>,
    pub lon: Option<f64>,
}
