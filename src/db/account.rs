use actix::prelude::{Handler, Message};
use diesel::{prelude::*, result::Error as DieselError};

use crate::model::account::{Account, NewAccount};

use super::{DbExecutor, SqliteConnection};

#[derive(Message)]
#[rtype(result = "Result<(), DieselError>")]
pub struct CreateAccount {
    pub title: String,
    pub currency: String,
}
impl From<CreateAccount> for NewAccount {
    fn from(from: CreateAccount) -> NewAccount {
        NewAccount {
            title: from.title,
            currency: from.currency,
        }
    }
}
impl Handler<CreateAccount> for DbExecutor {
    type Result = Result<(), DieselError>;

    fn handle(&mut self, msg: CreateAccount, _ctx: &mut Self::Context) -> Self::Result {
        use crate::schema::accounts;

        let conn = &self.0;

        diesel::insert_into(accounts::table)
            .values(NewAccount::from(msg))
            .execute(conn)?;

        Ok(())
    }
}
