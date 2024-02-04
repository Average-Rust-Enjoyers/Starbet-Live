@echo off
setlocal

FOR /F "tokens=*" %%i in ('type .env') do SET %%i

if not defined DATABASE_URL (
    echo DATABASE_URL is not set in environment, please set it 1>&2
    exit /b 1
)

sqlx migrate run
psql "%DATABASE_URL%" < .\docs\mock-data.sql