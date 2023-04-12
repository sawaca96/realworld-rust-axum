use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub mod user {
    use super::*;

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct User {
        pub id: Uuid,
        pub name: String,
        pub email: String,
    }
}

pub mod account {
    use super::*;

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct Account {
        pub id: Uuid,
        pub user_id: Uuid,
        pub cash: Decimal,
    }

    impl Account {
        pub fn execute_trade(&mut self, trade: &trade::Trade) -> Result<(), &'static str> {
            // Check if the user has sufficient funds
            if self.cash < trade.total_cost() {
                return Err("Insufficient funds");
            }

            // Lock the account instance to prevent race conditions
            // ...

            // Execute the trade
            // ...

            // Update the account's cash_balance
            self.cash -= trade.total_cost();

            // Unlock the account instance
            // ...

            Ok(())
        }
    }
}

pub mod crypto {
    use super::*;

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct Crypto {
        pub id: u64,
        pub name: String,
        pub symbol: String,
    }
}

pub mod position {
    use super::*;

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct Position {
        pub account_id: Uuid,
        pub crypto_id: u64,
        pub quantity: Decimal,
        pub buy_price: Decimal,
        pub purchase: Decimal,
        pub created_at: u64,
    }
}

pub mod trade {
    use super::*;

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct Trade {
        pub id: u64,
        pub user: user::User,
        pub order_id: Uuid,
        pub crypto: crypto::Crypto,
        pub quantity: Decimal,
        pub price: Decimal,
        pub executed: bool,
        pub executed_at: u64,
    }

    impl Trade {
        pub fn total_cost(&self) -> Decimal {
            self.quantity * self.price
        }
    }
}

pub mod order {
    use super::*;

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub enum TradeAction {
        Buy,
        Sell,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub enum OrderType {
        Market,
        Limit,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct Order {
        pub id: u64,
        pub user: user::User,
        pub account_id: Uuid,
        pub order_type: OrderType,
        pub trade_action: TradeAction,
        pub crypto_id: Uuid,
        pub quantity: Decimal,
        pub unfilled: Decimal,
        pub price: Decimal,
        pub created_at: u64,
    }
}
