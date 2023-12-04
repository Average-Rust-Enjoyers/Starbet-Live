CREATE TABLE IF NOT EXISTS Bet (
    id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    ----------------------------------------------
    app_user_id uuid NOT NULL,
    game_match_id uuid NOT NULL,
    ----------------------------------------------
    amount int NOT NULL,
    created_at timestamptz NOT NULL DEFAULT now(),
    edited_at timestamptz NOT NULL DEFAULT now(),
    deleted_at timestamptz,
    ----------------------------------------------
    FOREIGN KEY (app_user_id) REFERENCES AppUser (id),
    FOREIGN KEY (game_match_id) REFERENCES GameMatch (id)
)