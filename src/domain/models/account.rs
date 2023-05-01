use super::*;
use crate::schema::*;
use diesel::prelude::*;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Queryable, Identifiable)]
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
