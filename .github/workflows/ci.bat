@echo off

set CARGO=cargo
if "%CROSS%"=="1" (
    set CARGO_NET_RETRY=5
    set CARGO_NET_TIMEOUT=10

    cargo install cross
    set CARGO=cross
)

rem If a test crashes, we want to know which one it was.
set RUST_TEST_THREADS=1
set RUST_BACKTRACE=1

%CARGO% test --target "%TARGET%"
%CARGO% test --target "%TARGET%" --release

%CARGO% test --target "%TARGET%" --all-features
%CARGO% test --target "%TARGET%" --all-features --release

cd "%PROJECT_DIR%"/examples/axum-example
%CARGO% run || exit /b 0
cd "%PROJECT_DIR%"/examples/actix-web-example
%CARGO% run || exit /b 0
