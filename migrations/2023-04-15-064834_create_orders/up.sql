CREATE TABLE orders (
    id BIGSERIAL PRIMARY KEY,
    user_id UUID NOT NULL REFERENCES users(id),
    account_id UUID NOT NULL REFERENCES accounts(id),
    order_type VARCHAR NOT NULL CHECK (order_type IN ('Market', 'Limit')),
    trade_action VARCHAR NOT NULL CHECK (trade_action IN ('Buy', 'Sell')),
    crypto_id BIGINT NOT NULL REFERENCES cryptos(id),
    quantity NUMERIC NOT NULL,
    unfilled NUMERIC NOT NULL,
    price NUMERIC NOT NULL,
    created_at BIGINT NOT NULL
);
