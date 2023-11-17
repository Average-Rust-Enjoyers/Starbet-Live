CREATE TABLE IF NOT EXISTS Odds (
    id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    ---------------------------------------------
    match_id uuid NOT NULL,
    odds_a float NOT NULL,
    odds_b float NOT NULL,
    created_at timestamptz NOT NULL DEFAULT now(),
    edited_at timestamptz NOT NULL DEFAULT now(),
    deleted_at timestamptz,
    ---------------------------------------------
    FOREIGN KEY (match_id) REFERENCES Match(id)
);