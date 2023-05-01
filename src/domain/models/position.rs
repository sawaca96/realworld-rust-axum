use crate::schema::*;
use diesel::prelude::*;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::{crypto, trade};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Queryable, Identifiable)]
pub struct Position {
    pub id: Uuid,
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
