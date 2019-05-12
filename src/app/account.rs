use actix::prelude::Handler;
use diesel::result::Error as DieselError;
use futures::Future;

use crate::db::account::CreateAccount;

use super::PlutonioApp;
use std::rc::Rc;

pub struct NewAccount {
    pub title: String,
    pub currency: String,
    pub balance: Option<i32>,
}
impl From<NewAccount> for CreateAccount {
    fn from(from: NewAccount) -> CreateAccount {
        CreateAccount {
            title: from.title,
            currency: from.currency,
            balance: from.balance,
        }
    }
}

pub fn create_account(app: &PlutonioApp, account: NewAccount) {
    app.db.do_send(CreateAccount::from(account));
}