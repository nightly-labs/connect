use sqlx::Type;

#[derive(Clone, Debug, Eq, PartialEq, Type)]
#[sqlx(type_name = "device_medium_type_enum")]
pub enum DeviceMediumType {
    Browser,
    Mobile,
    Unknown,
}
