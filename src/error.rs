/// An error encountered while interacting with the database.
#[derive(Debug)]
pub(crate) enum DbError {
    Sql(sqlx::Error),
    Parse(hnefatafl::error::ParseError),
    Serde(serde_json::Error),
}

impl From<sqlx::Error> for DbError {
    fn from(value: sqlx::Error) -> Self {
        Self::Sql(value)
    }
}

impl From<hnefatafl::error::ParseError> for DbError {
    fn from(value: hnefatafl::error::ParseError) -> Self {
        Self::Parse(value)
    }
}

impl From<serde_json::Error> for DbError {
    fn from(value: serde_json::Error) -> Self {
        Self::Serde(value)
    }
}