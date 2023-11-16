CREATE TABLE IF NOT EXISTS Bet (
    id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    ----------------------------------------------
    user_id uuid NOT NULL,
    match_id uuid NOT NULL,
    amount int NOT NULL,
    created_at timestamptz NOT NULL DEFAULT now(),
    edited_at timestamptz NOT NULL DEFAULT now(),
    deleted_at timestamptz,
    ----------------------------------------------
    FOREIGN KEY (user_id) REFERENCES User (id),
    FOREIGN KEY (match_id) REFERENCES Match (id)
)