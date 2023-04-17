CREATE TABLE positions (
    account_id UUID NOT NULL REFERENCES accounts(id),
    crypto_id BIGINT NOT NULL REFERENCES cryptos(id),
    quantity NUMERIC NOT NULL,
    sellable_quantity NUMERIC NOT NULL,
    buy_price NUMERIC NOT NULL,
    purchase NUMERIC NOT NULL,
    created_at BIGINT NOT NULL,
    PRIMARY KEY (account_id, crypto_id)
);
