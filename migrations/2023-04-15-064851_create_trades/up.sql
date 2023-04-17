CREATE TABLE trades (
    id BIGSERIAL PRIMARY KEY,
    user_id UUID NOT NULL REFERENCES users(id),
    trade_type VARCHAR NOT NULL CHECK (trade_type IN ('Buy', 'Sell')),
    crypto_id BIGINT NOT NULL REFERENCES cryptos(id),
    quantity NUMERIC NOT NULL,
    price NUMERIC NOT NULL,
    executed BOOLEAN NOT NULL,
    executed_at BIGINT NOT NULL
);
