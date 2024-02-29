use sqlx::Type;

#[derive(Clone, Debug, Eq, PartialEq, Type)]
#[sqlx(type_name = "privilege_level_enum")]
pub enum PrivilegeLevel {
    Read,
    Edit,
    Admin,
}