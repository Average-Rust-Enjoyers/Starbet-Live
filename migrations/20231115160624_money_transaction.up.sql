CREATE TYPE IF NOT EXISTS currency AS ENUM ('CZK', 'EUR', 'USD');

CREATE TABLE IF NOT EXISTS money_transaction (
    id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    -------------------------------------------
    user_id uuid NOT NULL,
    amount_tokens int NOT NULL,
    amount_currency float NOT NULL,
    currency currency NOT NULL,
    deposit boolean NOT NULL,
    created_at timestamptz NOT NULL DEFAULT now(),
    edited_at timestamptz NOT NULL DEFAULT now(),
    deleted_at timestamptz,
    ---------------------------------------------
    FOREIGN KEY (user_id) REFERENCES user (id),
);