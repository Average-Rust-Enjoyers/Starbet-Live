CREATE TYPE GameMatchStatus AS ENUM ('PENDING', 'LIVE', 'FINISHED', 'CANCELED');

CREATE TABLE IF NOT EXISTS GameMatch (
    id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    ---------------------------------------------
    game_id uuid NOT NULL,
    ---------------------------------------------
    name_a text NOT NULL,
    name_b text NOT NULL,
    starts_at timestamptz NOT NULL,
    ends_at timestamptz NOT NULL,
    status GameMatchStatus NOT NULL DEFAULT 'PENDING',
    created_at timestamptz NOT NULL DEFAULT now(),
    edited_at timestamptz NOT NULL DEFAULT now(),
    deleted_at timestamptz,
    ----------------------------------------------
    FOREIGN KEY (game_id) REFERENCES Game (id)
);