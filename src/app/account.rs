use actix::prelude::Handler;
use diesel::result::Error as DieselError;
use futures::Future;

use crate::db::account::CreateAccount;

use super::PlutonioApp;

#[derive(Message)]
#[rtype(result = "Result<(), DieselError>")]
pub struct NewAccount {
    pub title: String,
    pub currency: String,
}
impl From<NewAccount> for CreateAccount {
    fn from(from: NewAccount) -> CreateAccount {
        CreateAccount {
            title: from.title,
            currency: from.currency,
        }
    }
}
impl Handler<NewAccount> for PlutonioApp {
    type Result = Result<(), DieselError>;

    fn handle(&mut self, msg: NewAccount, ctx: &mut Self::Context) -> Self::Result {
        self.db.do_send(CreateAccount::from(msg));

        Ok(())
    }
}
