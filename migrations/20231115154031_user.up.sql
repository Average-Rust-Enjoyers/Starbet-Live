CREATE TABLE IF NOT EXISTS User (
    id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    ----------------------------------------------
    username text NOT NULL,
    email text NOT NULL,
    name text NOT NULL,
    surname text NOT NULL,
    profile_picture text NOT NULL,
    password_hash text NOT NULL,
    password_salt text NOT NULL,
    balance int NOT NULL,
    created_at timestamptz NOT NULL DEFAULT now(),
    edited_at timestamptz NOT NULL DEFAULT now(),
    deleted_at timestamptz
);