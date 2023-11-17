CREATE TYPE MatchStatus AS ENUM ('PENDING', 'LIVE', 'FINISHED', 'CANCELED');

CREATE TABLE IF NOT EXISTS Match (
    id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    ---------------------------------------------
    name_a text NOT NULL,
    name_b text NOT NULL,
    starts_at timestamptz NOT NULL,
    ends_at timestamptz NOT NULL,
    status MatchStatus NOT NULL DEFAULT 'PENDING',
    game_id uuid NOT NULL,
    created_at timestamptz NOT NULL DEFAULT now(),
    edited_at timestamptz NOT NULL DEFAULT now(),
    deleted_at timestamptz,
    ----------------------------------------------
    FOREIGN KEY (game_id) REFERENCES Game(id)
);