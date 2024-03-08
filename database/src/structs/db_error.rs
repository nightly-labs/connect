use std::fmt;

#[derive(Debug)]
pub enum DbError {
    DatabaseError(String),
    SqlxDbError(sqlx::Error),
}

impl fmt::Display for DbError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DbError::DatabaseError(ref err) => write!(f, "Database error: {}", err),
            DbError::SqlxDbError(ref err) => write!(f, "Sqlx database error: {}", err),
        }
    }
}

impl From<sqlx::Error> for DbError {
    fn from(error: sqlx::Error) -> DbError {
        DbError::SqlxDbError(error)
    }
}

impl From<String> for DbError {
    fn from(error: String) -> DbError {
        DbError::DatabaseError(error)
    }
}
