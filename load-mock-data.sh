#!/bin/bash
set -e

# load environment variables from envfile if present
source "$(dirname -- "$0")/.env";

if [ "$DATABASE_URL" = "" ]; then
    echo 'DATABASE_URL is not set in environment, please set it' >&2
    exit 1
fi

sqlx migrate run
psql "$DATABASE_URL" < ./docs/mock-data.sql