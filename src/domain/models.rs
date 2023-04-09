pub mod user {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct User {
        pub id: u64,
        pub username: String,
        pub email: String,
        pub password: String,
        pub created_at: String,
    }
}

pub mod account {
    use serde::{Deserialize, Serialize};

    use super::trade::Trade;

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct Account {
        pub id: u64,
        pub user_id: u64,
        pub cash_balance: f64,
    }

    impl Account {
        pub fn execute_trade(&mut self, trade: &Trade) -> Result<(), &'static str> {
            // Check if the user has sufficient funds
            if self.cash_balance < trade.total_cost() {
                return Err("Insufficient funds");
            }

            // Lock the account instance to prevent race conditions
            // ...

            // Execute the trade
            // ...

            // Update the account's cash_balance
            self.cash_balance -= trade.total_cost();

            // Unlock the account instance
            // ...

            Ok(())
        }
    }
}
pub mod crypto {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct Crypto {
        pub id: u64,
        pub name: String,
        pub symbol: String,
        pub price_usd: f64,
    }
}

pub mod portfolio {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct Portfolio {
        pub id: u64,
        pub user_id: u64,
        pub holdings: Vec<Holding>,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct Holding {
        pub crypto_id: u64,
        pub quantity: f64,
    }

    impl Portfolio {
        pub fn add_holding(&mut self, crypto_id: u64, quantity: f64) {
            self.holdings.push(Holding {
                crypto_id,
                quantity,
            });
        }

        pub fn update_holding(
            &mut self,
            crypto_id: u64,
            new_quantity: f64,
        ) -> Result<(), &'static str> {
            let holding = self.holdings.iter_mut().find(|h| h.crypto_id == crypto_id);

            match holding {
                Some(h) => {
                    h.quantity = new_quantity;
                    Ok(())
                }
                None => Err("Holding not found"),
            }
        }

        pub fn remove_holding(&mut self, crypto_id: u64) -> Result<(), &'static str> {
            let index = self.holdings.iter().position(|h| h.crypto_id == crypto_id);

            match index {
                Some(i) => {
                    self.holdings.remove(i);
                    Ok(())
                }
                None => Err("Holding not found"),
            }
        }
    }
}

pub mod trade {
    use serde::{Deserialize, Serialize};

    use crate::domain::models::{crypto::Crypto, user::User};

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub enum TradeType {
        Buy,
        Sell,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct Trade {
        pub id: u64,
        pub user: User,
        pub trade_type: TradeType,
        pub crypto: Crypto,
        pub quantity: f64,
        pub price: f64,
        pub executed: bool,
        pub execution_date: Option<String>,
    }

    impl Trade {
        pub fn total_cost(&self) -> f64 {
            self.quantity * self.price
        }
    }
}

pub mod order {
    use serde::{Deserialize, Serialize};

    use crate::domain::models::{crypto::Crypto, user::User};

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub enum OrderType {
        Buy,
        Sell,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub enum OrderExecutionType {
        Market,
        Limit,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct Order {
        pub id: u64,
        pub user: User,
        pub order_type: OrderType,
        pub execution_type: OrderExecutionType,
        pub crypto: Crypto,
        pub quantity: f64,
        pub price: f64,
        pub order_date: String,
    }
}
