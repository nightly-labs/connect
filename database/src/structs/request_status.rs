use sqlx::Type;

#[derive(Clone, Debug, Eq, PartialEq, Type)]
#[sqlx(type_name = "request_status_enum")]
pub enum RequestStatus {
    Pending,
    Completed,
    Rejected,
    TimedOut,
}
