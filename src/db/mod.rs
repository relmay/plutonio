pub mod account;

use actix::prelude::{Actor, Context};
use diesel::{sqlite::SqliteConnection, Connection as _, ConnectionError};

pub struct DbExecutor(pub SqliteConnection);
impl Actor for DbExecutor {
    type Context = Context<Self>;
}

pub fn get_db_connection(database_url: &str) -> Result<SqliteConnection, ConnectionError> {
    SqliteConnection::establish(database_url)
}
