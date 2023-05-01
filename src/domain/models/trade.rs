use crate::schema::*;
use diesel::prelude::*;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use super::{crypto, user};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TradeType {
    Buy,
    Sell,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Queryable, Identifiable)]
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
