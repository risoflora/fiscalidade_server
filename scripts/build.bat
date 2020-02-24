:::::::::::::::::::::::::::::::::::::::::::::::::
:: Copyright (c) 2020 Silvio Clecio (silvioprog)
:::::::::::::::::::::::::::::::::::::::::::::::::

:: Download the PostgreSQL zip file for Windows from:
:: https://www.enterprisedb.com/download-postgresql-binaries

@echo off
set PQ_LIB_DIR=C:\pgsql\lib
set RUSTFLAGS=-Ctarget-feature=+crt-static
cargo build --release
