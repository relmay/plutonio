#[macro_use]
extern crate diesel;
#[macro_use]
extern crate serde;
#[macro_use]
extern crate smart_default;
#[macro_use]
extern crate actix;
#[macro_use]
extern crate diesel_migrations;

mod app;
mod conf;
mod db;
mod model;
mod schema;

use clap::{App, Arg, SubCommand};

use crate::{
    app::{
        PlutonioApp,
        account,
    },

};

fn main() {
    let matches = App::new("Plutonio")
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .subcommand(
            SubCommand::with_name("add")
                .alias("a")
                .about("Add new transaction"),
        )
        .subcommand(SubCommand::with_name("balance").about("Get balance info."))
        .subcommand(SubCommand::with_name("stats").about("Get accounts stats."))
        .subcommand(
            SubCommand::with_name("budget")
                .alias("b")
                .about("Control your budget."),
        )
        .subcommand(SubCommand::with_name("account").about("Control your accounts."))
        .get_matches();

    let sys = actix::System::new("plutonio");
    let app = PlutonioApp::new();
    account::create_account(&app, account::NewAccount {
        title: "TEst3".to_string(),
        currency: "RUB".to_string(),
    });

    let _ = sys.run();
}
