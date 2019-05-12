//! Model for `accounts` table

use crate::schema::accounts;

/// This is bank account entity.
/// This entity reflects a certain money account (for example in a bank).
/// It can also be a cash, bank deposit or any other depository of money.
#[derive(Debug, Queryable, Identifiable)]
pub struct Account {
    pub id: i32,
    pub title: String,
    /// Currency code in __ISO4217__ format
    pub currency: String,
    /// Balance in penny
    pub balance: i32,
}

/// Entity for creating new [`Account`].
#[derive(Debug, Insertable)]
#[table_name = "accounts"]
pub struct NewAccount {
    pub title: String,
    pub currency: String,
    /// Start balance of account in penny
    pub balance: Option<i32>,
}
