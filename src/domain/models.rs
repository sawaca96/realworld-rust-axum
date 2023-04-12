use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub mod user {
    use super::*;

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct User {
        pub id: Uuid,
        pub nickname: String,
        pub email: String,
    }
}

pub mod account {
    use super::*;

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct Account {
        pub id: Uuid,
        pub user: user::User,
        pub cash: Decimal,
    }

    impl Account {
        pub fn has_sufficient_cash(&self, required_cash: Decimal) -> bool {
            self.cash >= required_cash
        }

        pub fn reserve_cash(&mut self, amount: Decimal) {
            self.cash -= amount;
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
        pub crypto: crypto::Crypto,
        pub quantity: Decimal,
        pub sellable_quantity: Decimal,
        pub buy_price: Decimal,
        pub purchase: Decimal,
        pub created_at: u64,
    }

    impl Position {
        pub fn can_sell(&self, quantity: Decimal) -> bool {
            self.sellable_quantity >= quantity
        }

        pub fn reserve_for_sell(&mut self, amount: Decimal) {
            self.sellable_quantity -= amount;
        }

        pub fn update_position(&mut self, trade: &trade::Trade) {
            let new_quantity = self.quantity + trade.quantity;
            let new_purchase = self.purchase + trade.total_cost();
            self.buy_price = new_purchase / new_quantity;
            self.purchase = new_purchase;
            self.quantity = new_quantity;
            self.sellable_quantity += trade.quantity;
        }
    }
}

pub mod trade {
    use super::*;

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub enum TradeType {
        Buy,
        Sell,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct Trade {
        pub id: u64,
        pub user: user::User,
        pub trade_type: TradeType,
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
        pub account: account::Account,
        pub order_type: OrderType,
        pub trade_action: TradeAction,
        pub crypto: crypto::Crypto,
        pub quantity: Decimal,
        pub unfilled: Decimal,
        pub price: Decimal,
        pub created_at: u64,
    }

    impl Order {
        pub fn update_unfilled(&mut self, executed_quantity: Decimal) {
            self.unfilled -= executed_quantity;
        }
    }
}
