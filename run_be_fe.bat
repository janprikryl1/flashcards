@echo off
setlocal

start "frontend" cmd /k "cd /d %~dp0client && trunk serve"

cd /d "%~dp0api"
cargo run