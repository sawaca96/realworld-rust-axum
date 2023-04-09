// src/tests/models.rs

use crate::domain::models::account::Account;
use crate::domain::models::crypto::Crypto;
use crate::domain::models::portfolio::{Holding, Portfolio};
use crate::domain::models::trade::{Trade, TradeType};
use crate::domain::models::user::User;

#[test]
fn test_execute_trade_sufficient_funds() {
    let mut account = Account {
        id: 1,
        user_id: 1,
        cash_balance: 5000.0,
    };

    let trade = Trade {
        id: 1,
        user: User {
            id: 1,
            username: "testuser".to_string(),
            email: "test@example.com".to_string(),
            password: "password".to_string(),
            created_at: "2023-04-09".to_string(),
        },
        trade_type: TradeType::Buy,
        crypto: Crypto {
            id: 1,
            name: "Bitcoin".to_string(),
            symbol: "BTC".to_string(),
            price_usd: 1000.0,
        },
        quantity: 2.0,
        price: 1000.0,
        executed: false,
        execution_date: None,
    };

    assert!(account.execute_trade(&trade).is_ok());
    assert_eq!(account.cash_balance, 3000.0);
}

#[test]
fn test_execute_trade_insufficient_funds() {
    let mut account = Account {
        id: 1,
        user_id: 1,
        cash_balance: 1000.0,
    };

    let trade = Trade {
        id: 1,
        user: User {
            id: 1,
            username: "testuser".to_string(),
            email: "test@example.com".to_string(),
            password: "password".to_string(),
            created_at: "2023-04-09".to_string(),
        },
        trade_type: TradeType::Buy,
        crypto: Crypto {
            id: 1,
            name: "Bitcoin".to_string(),
            symbol: "BTC".to_string(),
            price_usd: 1000.0,
        },
        quantity: 2.0,
        price: 1000.0,
        executed: false,
        execution_date: None,
    };

    assert_eq!(
        account.execute_trade(&trade).unwrap_err(),
        "Insufficient funds"
    );
}
#[test]
fn test_add_holding() {
    let mut portfolio = Portfolio {
        id: 1,
        user_id: 1,
        holdings: vec![],
    };

    portfolio.add_holding(1, 2.0);

    assert_eq!(
        portfolio.holdings,
        vec![Holding {
            crypto_id: 1,
            quantity: 2.0,
        }]
    );
}

#[test]
fn test_update_holding() {
    let mut portfolio = Portfolio {
        id: 1,
        user_id: 1,
        holdings: vec![Holding {
            crypto_id: 1,
            quantity: 2.0,
        }],
    };

    assert!(portfolio.update_holding(1, 3.0).is_ok());
    assert_eq!(
        portfolio.holdings,
        vec![Holding {
            crypto_id: 1,
            quantity: 3.0,
        }]
    );
}

#[test]
fn test_remove_holding() {
    let mut portfolio = Portfolio {
        id: 1,
        user_id: 1,
        holdings: vec![Holding {
            crypto_id: 1,
            quantity: 2.0,
        }],
    };

    assert!(portfolio.remove_holding(1).is_ok());
    assert!(portfolio.holdings.is_empty());
}
