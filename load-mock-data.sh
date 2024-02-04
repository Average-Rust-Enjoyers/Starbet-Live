#!/bin/bash
set -e

# load environment variables from envfile if present
ENVFILE="$(dirname -- "$0")/.env"
[ -f "$ENVFILE" ] && source "$ENVFILE";

if [ "$DATABASE_URL" = "" ]; then
    echo 'DATABASE_URL is not set in environment, please set it' >&2
    exit 1
fi

sqlx database setup
psql "$DATABASE_URL" < ./docs/mock-data.sql