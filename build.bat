@echo off
REM Set Rust compiler flags
set RUSTFLAGS=-A unused
echo RUSTFLAGS set to %RUSTFLAGS%

REM Run cargo build
cargo build

REM Reset RUSTFLAGS
set RUSTFLAGS=
