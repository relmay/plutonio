use diesel::{
    sqlite::SqliteConnection,
    ConnectionError,
    Connection as _,
};

pub fn get_db_connection(database_url: &str) -> Result<SqliteConnection, ConnectionError> {
    SqliteConnection::establish(database_url)
}