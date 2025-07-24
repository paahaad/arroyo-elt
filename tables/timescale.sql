CREATE TABLE pump_mint (
    name TEXT,
    symbol TEXT,
    uri TEXT,
    mint TEXT,
    bonding_curve TEXT,
    "user" TEXT,
    creator TEXT,
    timestamp TIMESTAMP NOT NULL,
    virtual_token_reserves TEXT,
    virtual_sol_reserves TEXT,
    real_token_reserves TEXT,
    token_total_supply TEXT 
);

SELECT create_hypertable('pump_mint', 'timestamp');