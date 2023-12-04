CREATE TYPE Currency AS ENUM ('CZK', 'EUR', 'USD');

CREATE TABLE IF NOT EXISTS MoneyTransaction (
    id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    -------------------------------------------
    app_user_id uuid NOT NULL,
    -------------------------------------------
    amount_tokens int NOT NULL,
    amount_currency float NOT NULL,
    currency Currency NOT NULL,
    deposit boolean NOT NULL,
    created_at timestamptz NOT NULL DEFAULT now(),
    edited_at timestamptz NOT NULL DEFAULT now(),
    deleted_at timestamptz,
    ---------------------------------------------
    FOREIGN KEY (app_user_id) REFERENCES AppUser (id)
);