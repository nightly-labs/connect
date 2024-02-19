use sqlx::Type;

#[derive(Clone, Debug, Eq, PartialEq, Type)]
#[sqlx(type_name = "subscription")]
pub struct Subscription {
    pub email: String,
    pub subscribed_at: i64,
}
