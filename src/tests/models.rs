use crate::domain::models::account;
use crate::domain::models::crypto;
use crate::domain::models::order;
use crate::domain::models::position;
use crate::domain::models::trade;
use crate::domain::models::user;
use rust_decimal_macros::dec;
use uuid::Uuid;

fn get_test_user() -> user::User {
    user::User {
        id: Uuid::new_v4(),
        nickname: "John Doe".to_string(),
        email: "john.doe@example.com".to_string(),
    }
}

fn get_test_crypto() -> crypto::Crypto {
    crypto::Crypto {
        id: 1,
        name: "Bitcoin".to_string(),
        symbol: "BTC".to_string(),
    }
}

#[test]
fn account_has_sufficient_cash() {
    let account = account::Account {
        id: Uuid::new_v4(),
        user: get_test_user(),
        cash: dec!(100.00),
    };

    assert!(account.has_sufficient_cash(dec!(50.00)));
    assert!(!account.has_sufficient_cash(dec!(200.00)));
}

#[test]
fn account_reserve_cash() {
    let mut account = account::Account {
        id: Uuid::new_v4(),
        user: get_test_user(),
        cash: dec!(100.00),
    };

    account.reserve_cash(dec!(25.00));
    assert_eq!(account.cash, dec!(75.00));
}

#[test]
fn position_can_sell() {
    let position = position::Position {
        account_id: Uuid::new_v4(),
        crypto: get_test_crypto(),
        quantity: dec!(10.00),
        sellable_quantity: dec!(8.00),
        buy_price: dec!(5000.00),
        purchase: dec!(50000.00),
        created_at: 0,
    };

    assert!(position.can_sell(dec!(5.00)));
    assert!(!position.can_sell(dec!(10.00)));
}

#[test]
fn position_reserve_for_sell() {
    let mut position = position::Position {
        account_id: Uuid::new_v4(),
        crypto: get_test_crypto(),
        quantity: dec!(10.00),
        sellable_quantity: dec!(8.00),
        buy_price: dec!(5000.00),
        purchase: dec!(50000.00),
        created_at: 0,
    };

    position.reserve_for_sell(dec!(2.00));
    assert_eq!(position.sellable_quantity, dec!(6.00));
}

#[test]
fn position_update_position() {
    let mut position = position::Position {
        account_id: Uuid::new_v4(),
        crypto: get_test_crypto(),
        quantity: dec!(10.00),
        sellable_quantity: dec!(8.00),
        buy_price: dec!(5000.00),
        purchase: dec!(50000.00),
        created_at: 0,
    };

    let trade = trade::Trade {
        id: 1,
        user: get_test_user(),
        trade_type: trade::TradeType::Buy,
        crypto: get_test_crypto(),
        quantity: dec!(5.00),
        price: dec!(6000.00),
        executed: true,
        executed_at: 100,
    };

    position.update_position(&trade);
    assert_eq!(position.quantity, dec!(15.00));
    assert_eq!(position.sellable_quantity, dec!(13.00));
    assert_eq!(position.buy_price, dec!(5333.3333333333333333333333333));
}
#[test]
fn trade_total_cost() {
    let trade = trade::Trade {
        id: 1,
        user: get_test_user(),
        trade_type: trade::TradeType::Buy,
        crypto: get_test_crypto(),
        quantity: dec!(5.00),
        price: dec!(6000.00),
        executed: true,
        executed_at: 100,
    };

    assert_eq!(trade.total_cost(), dec!(30000.00));
}

#[test]
fn order_update_unfilled() {
    let mut order = order::Order {
        id: 1,
        user: get_test_user(),
        account: account::Account {
            id: Uuid::new_v4(),
            user: get_test_user(),
            cash: dec!(100.00),
        },
        order_type: order::OrderType::Market,
        trade_action: order::TradeAction::Buy,
        crypto: get_test_crypto(),
        quantity: dec!(10.00),
        unfilled: dec!(10.00),
        price: dec!(6000.00),
        created_at: 0,
    };

    order.update_unfilled(dec!(4.00));
    assert_eq!(order.unfilled, dec!(6.00));
}
