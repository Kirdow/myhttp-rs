@echo off
REM Set Rust compiler flags
set RUSTFLAGS=-A unused
echo RUSTFLAGS set to %RUSTFLAGS%

REM Run cargo check
cargo check

REM Reset RUSTFLAGS
set RUSTFLAGS=