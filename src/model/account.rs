use crate::schema::accounts;

#[derive(Debug, Queryable, Identifiable)]
pub struct Account {
    pub id: i32,
    pub title: String,
    pub currency: String,
    pub balance: i32,
}

#[derive(Debug, Insertable)]
#[table_name = "accounts"]
pub struct NewAccount {
    pub title: String,
    pub currency: String,
    pub balance: Option<i32>,
}
