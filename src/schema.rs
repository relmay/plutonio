table! {
    accounts (id) {
        id -> Nullable<Integer>,
        title -> Text,
        currency -> Text,
    }
}

table! {
    transactions (id) {
        id -> Nullable<Integer>,
        account_id -> Integer,
        title -> Nullable<Text>,
        t_type -> Integer,
        amount -> Integer,
    }
}

joinable!(transactions -> accounts (account_id));

allow_tables_to_appear_in_same_query!(
    accounts,
    transactions,
);
