use crate::schema::transactions;

use super::account::Account;

#[derive(Identifiable, Queryable, Associations, PartialEq, Debug)]
#[belongs_to(Account)]
#[table_name = "transactions"]
pub struct Transaction {
    pub id: i32,
    pub account_id: i32,
    pub title: String,
    pub t_type: i32,
    pub amount: i32,
}

#[derive(Debug, Insertable)]
#[table_name = "transactions"]
pub struct NewTransaction {
    pub account_id: i32,
    pub title: String,
    pub t_type: i32,
    pub amount: i32,
}
