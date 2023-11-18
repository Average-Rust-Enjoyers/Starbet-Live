-- Add up migration script here
CREATE TYPE GameGenre AS ENUM (
    'MOBA (MultiPlayer Online Battle Arena)',
    'FPS (First Person Shooter)'
);

CREATE TABLE IF NOT EXISTS Game (
    id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    -----------------------------------------
    name text NOT NULL,
    description text NOT NULL,
    logo text NOT NULL,
    genre GameGenre NOT NULL,
    created_at timestamptz NOT NULL DEFAULT now(),
    edited_at timestamptz NOT NULL DEFAULT now(),
    deleted_at timestamptz
)