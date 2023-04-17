// @generated automatically by Diesel CLI.

diesel::table! {
    accounts (id) {
        id -> Uuid,
        user_id -> Uuid,
        cash -> Numeric,
    }
}

diesel::table! {
    cryptos (id) {
        id -> Int8,
        name -> Varchar,
        symbol -> Varchar,
    }
}

diesel::table! {
    orders (id) {
        id -> Int8,
        user_id -> Uuid,
        account_id -> Uuid,
        order_type -> Varchar,
        trade_action -> Varchar,
        crypto_id -> Int8,
        quantity -> Numeric,
        unfilled -> Numeric,
        price -> Numeric,
        created_at -> Int8,
    }
}

diesel::table! {
    positions (account_id, crypto_id) {
        account_id -> Uuid,
        crypto_id -> Int8,
        quantity -> Numeric,
        sellable_quantity -> Numeric,
        buy_price -> Numeric,
        purchase -> Numeric,
        created_at -> Int8,
    }
}

diesel::table! {
    trades (id) {
        id -> Int8,
        user_id -> Uuid,
        trade_type -> Varchar,
        crypto_id -> Int8,
        quantity -> Numeric,
        price -> Numeric,
        executed -> Bool,
        executed_at -> Int8,
    }
}

diesel::table! {
    users (id) {
        id -> Uuid,
        nickname -> Varchar,
        email -> Varchar,
    }
}

diesel::joinable!(accounts -> users (user_id));
diesel::joinable!(orders -> accounts (account_id));
diesel::joinable!(orders -> cryptos (crypto_id));
diesel::joinable!(orders -> users (user_id));
diesel::joinable!(positions -> accounts (account_id));
diesel::joinable!(positions -> cryptos (crypto_id));
diesel::joinable!(trades -> cryptos (crypto_id));
diesel::joinable!(trades -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    accounts,
    cryptos,
    orders,
    positions,
    trades,
    users,
);
