-- Section AppUser --

/* AppUser table */
CREATE TABLE IF NOT EXISTS AppUser (
    id              uuid PRIMARY KEY     DEFAULT gen_random_uuid(),
    ---------------------------------------------------------------
    username        text        NOT NULL,
    email           text        NOT NULL,
    name            text        NOT NULL,
    surname         text        NOT NULL,
    profile_picture text        NOT NULL,
    password_hash   text        NOT NULL,
    password_salt   text        NOT NULL,
    balance         int         NOT NULL,
    created_at      timestamptz NOT NULL DEFAULT now(),
    edited_at       timestamptz NOT NULL DEFAULT now(),
    deleted_at      timestamptz
);


-- Section Game --

/* GameGenre enum */
CREATE TYPE GameGenre AS ENUM (
    'MOBA (MultiPlayer Online Battle Arena)',
    'FPS (First Person Shooter)'
);

/* Game table */
CREATE TABLE IF NOT EXISTS Game (
    id          uuid PRIMARY KEY     DEFAULT gen_random_uuid(),
    -----------------------------------------------------------
    name        text        NOT NULL,
    description text        NOT NULL,
    logo        text        NOT NULL,
    genre       GameGenre   NOT NULL,
    created_at  timestamptz NOT NULL DEFAULT now(),
    edited_at   timestamptz NOT NULL DEFAULT now(),
    deleted_at  timestamptz
);


-- Section GameMatch --

/* GameMatchStatus enum */
CREATE TYPE GameMatchStatus AS ENUM (
    'PENDING',
    'LIVE',
    'FINISHED',
    'CANCELED'
);

/* GameMatchOutcome enum */
CREATE TYPE GameMatchOutcome AS ENUM (
    'WIN_A',
    'WIN_B',
    'DRAW'
);

/* GameMatch table */
CREATE TABLE IF NOT EXISTS GameMatch (
    id         uuid PRIMARY KEY          DEFAULT gen_random_uuid(),
    ---------------------------------------------------------------
    game_id    uuid             NOT NULL,
    ---------------------------------------------------------------
    name_a     text             NOT NULL,
    name_b     text             NOT NULL,
    starts_at  timestamptz      NOT NULL,
    ends_at    timestamptz      NOT NULL,
    status     GameMatchStatus  NOT NULL DEFAULT 'PENDING',
    outcome    GameMatchOutcome,
    created_at timestamptz      NOT NULL DEFAULT now(),
    edited_at  timestamptz      NOT NULL DEFAULT now(),
    deleted_at timestamptz,
    ---------------------------------------------------------------
    FOREIGN KEY (game_id) REFERENCES Game (id)
);

/* GameMatch indexes */
CREATE INDEX IF NOT EXISTS GameMatchGameId ON GameMatch (game_id);
CREATE INDEX IF NOT EXISTS GameMatchRange  ON GameMatch (starts_at, ends_at);


-- Section Odds --

/* Odds table */
CREATE TABLE IF NOT EXISTS Odds (
    id            uuid PRIMARY KEY     DEFAULT gen_random_uuid(),
    -------------------------------------------------------------
    game_match_id uuid        NOT NULL,
    -------------------------------------------------------------
    odds_a        float       NOT NULL,
    odds_b        float       NOT NULL,
    created_at    timestamptz NOT NULL DEFAULT now(),
    edited_at     timestamptz NOT NULL DEFAULT now(),
    deleted_at    timestamptz,
    -------------------------------------------------------------
    FOREIGN KEY (game_match_id) REFERENCES GameMatch (id)
);

/* Odds indexes */
CREATE INDEX IF NOT EXISTS OddsGameMatchId ON Odds (game_match_id);


-- Section Bet --

/* BetStatus enum */
CREATE TYPE BetStatus AS ENUM (
    'PENDING',
    'WON',
    'LOST',
    'CANCELED'
);

/* Bet table */
CREATE TABLE IF NOT EXISTS Bet (
    id               uuid PRIMARY KEY          DEFAULT gen_random_uuid(),
    ---------------------------------------------------------------------
    app_user_id      uuid             NOT NULL,
    game_match_id    uuid             NOT NULL,
    ---------------------------------------------------------------------
    amount           int              NOT NULL,
    status           BetStatus        NOT NULL DEFAULT 'PENDING',
    expected_outcome GameMatchOutcome NOT NULL,
    created_at       timestamptz      NOT NULL DEFAULT now(),
    edited_at        timestamptz      NOT NULL DEFAULT now(),
    deleted_at       timestamptz,
    ---------------------------------------------------------------------
    FOREIGN KEY (app_user_id)   REFERENCES AppUser   (id),
    FOREIGN KEY (game_match_id) REFERENCES GameMatch (id)
);

/* Bet indexes */
CREATE INDEX IF NOT EXISTS BetAppUserId        ON Bet (app_user_id);
CREATE INDEX IF NOT EXISTS BetGameMatchId      ON Bet (game_match_id);
CREATE INDEX IF NOT EXISTS BetCreatedDeletedAt ON Bet (created_at DESC, deleted_at NULLS LAST);


-- Section MoneyTransaction --

/* Currency enum */
CREATE TYPE Currency AS ENUM (
    'CZK',
    'EUR',
    'USD'
);

/* MoneyTransactionStatus enum */
CREATE TYPE MoneyTransactionStatus AS ENUM (
    'PENDING',
    'COMPLETED',
    'CANCELED'
);

/* MoneyTransaction table */
CREATE TABLE IF NOT EXISTS MoneyTransaction (
    id              uuid PRIMARY KEY                DEFAULT gen_random_uuid(),
    --------------------------------------------------------------------------
    app_user_id     uuid                   NOT NULL,
    --------------------------------------------------------------------------
    status          MoneyTransactionStatus NOT NULL DEFAULT 'PENDING',
    amount_tokens   int                    NOT NULL,
    amount_currency float                  NOT NULL,
    currency        Currency               NOT NULL,
    deposit         boolean                NOT NULL,
    created_at      timestamptz            NOT NULL DEFAULT now(),
    edited_at       timestamptz            NOT NULL DEFAULT now(),
    deleted_at      timestamptz,
    --------------------------------------------------------------------------
    FOREIGN KEY (app_user_id) REFERENCES AppUser (id)
);

/* MoneyTransaction indexes */
CREATE INDEX IF NOT EXISTS MoneyTransactionAppUserId        ON MoneyTransaction (app_user_id);
CREATE INDEX IF NOT EXISTS MoneyTransactionCreatedDeletedAt ON MoneyTransaction (created_at DESC, deleted_at NULLS LAST);
