@echo off
REM Set Rust compiler flags
set RUSTFLAGS=-A unused
echo RUSTFLAGS set to %RUSTFLAGS%

REM Run cargo check
cargo run

REM Reset RUSTFLAGS
set RUSTFLAGS=
