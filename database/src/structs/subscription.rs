use sqlx::Type;

#[derive(Clone, Debug, Eq, PartialEq, Type)]
#[sqlx(type_name = "subscription")]
pub struct Subscription {
    pub subscription_type: String,
    pub valid_from: i64,
    pub valid_till: i64,
}
