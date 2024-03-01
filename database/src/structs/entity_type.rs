use sqlx::Type;

#[derive(Clone, Debug, Eq, PartialEq, Type)]
#[sqlx(type_name = "entity_type_enum")]
pub enum EntityType {
    Client,
    App,
}
