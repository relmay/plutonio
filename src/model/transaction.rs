//! Models for `transactions` table.

use crate::schema::transactions;

use super::account::Account;

/// Entity for some transaction.
/// A transaction can be both outgoing and incoming.
/// Based on this, you can determine how it affected the balance.
/// To determine the type of transaction used field `t_type`.
#[derive(Identifiable, Queryable, Associations, PartialEq, Debug)]
#[belongs_to(Account)]
#[table_name = "transactions"]
pub struct Transaction {
    pub id: i32,
    /// ID of [`Account`] that this transaction affects.
    pub account_id: i32,
    /// Some description of transaction.
    pub title: String,
    /// This field represents type of transaction.
    pub t_type: i32,
    /// Amount of money in penny.
    pub amount: i32,
}

#[derive(Debug, Insertable)]
#[table_name = "transactions"]
pub struct NewTransaction {
    /// ID of [`Account`] that this transaction affects.
    pub account_id: i32,
    /// Some description of transaction.
    pub title: String,
    /// This field represents type of transaction.
    pub t_type: i32,
    /// Amount of money in penny.
    pub amount: i32,
}
