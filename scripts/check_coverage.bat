@echo off
setlocal enabledelayedexpansion

:: Set the minimum coverage threshold (same as in CI)
set MIN_COVERAGE_PERCENT=80

:: Check if cargo-tarpaulin is installed
cargo tarpaulin --version >nul 2>&1
if %ERRORLEVEL% NEQ 0 (
    echo Installing cargo-tarpaulin...
    cargo install cargo-tarpaulin
)

:: Create coverage directory if it doesn't exist
if not exist coverage mkdir coverage

echo Running tests with coverage...
cargo tarpaulin --out Xml --output-dir coverage --workspace

:: Check coverage threshold
echo Checking coverage threshold...
for /f "tokens=*" %%a in ('cargo tarpaulin --out Json --workspace ^| findstr "coverage"') do (
    set line=%%a
    set line=!line:~13!
    for /f "delims=, tokens=1" %%b in ("!line!") do (
        set coverage_value=%%b
        set coverage_value=!coverage_value:.=!
        set coverage_value=!coverage_value:~0,2!
    )
)

if !coverage_value! LSS %MIN_COVERAGE_PERCENT% (
    echo ❌ Test coverage is below threshold: !coverage_value!%% ^< %MIN_COVERAGE_PERCENT!%%
    exit /b 1
) else (
    echo ✅ Test coverage meets threshold: !coverage_value!%% ^>= %MIN_COVERAGE_PERCENT!%%
)

echo Coverage report saved to: coverage\cobertura.xml
