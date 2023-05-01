use crate::schema::*;
use diesel::prelude::*;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use super::{account, crypto, user};

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

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Queryable, Identifiable)]
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
