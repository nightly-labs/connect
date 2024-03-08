use sqlx::Type;

#[derive(Clone, Debug, Eq, PartialEq, Type)]
#[sqlx(type_name = "session_type_enum")]
pub enum SessionType {
    Extension,
    Relay,
}
