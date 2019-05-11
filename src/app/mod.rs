use diesel::{
    sqlite::SqliteConnection
};

use crate::db::get_db_connection;
use crate::conf::Conf;

struct PlutonioApp {
    db: SqliteConnection,
    config: Conf,
}
impl PlutonioApp {
    // TODO: take config argument
    pub fn new() -> Self {
        let conf = Conf::parse().unwrap();

        Self {
            db: get_db_connection(&conf.database_path).unwrap(),
            config: conf,
        }
    }
}