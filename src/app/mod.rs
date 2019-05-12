pub mod account;

use actix::prelude::{Actor, Addr, Arbiter, Context};
use diesel::sqlite::SqliteConnection;

use crate::{
    conf::Conf,
    db::{get_db_connection, DbExecutor},
};

pub struct PlutonioApp {
    db: Addr<DbExecutor>,
    config: Conf,
}
impl Actor for PlutonioApp {
    type Context = Context<Self>;
}
impl PlutonioApp {
    // TODO: take config argument
    pub fn new() -> Addr<Self> {
        Arbiter::start(|_| {
            let conf = Conf::parse().unwrap();
            println!("Current config: {:?}", conf);
            let addr = conf.database_path.clone();
            let db_executor =
                Arbiter::start(move |_| {
                    let conn = get_db_connection(&addr).unwrap();

                    embed_migrations!("migrations");
                    embedded_migrations::run(&conn);

                    DbExecutor(conn)
                });

            Self {
                db: db_executor,
                config: conf,
            }
        })
    }
}
