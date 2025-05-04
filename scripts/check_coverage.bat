@echo off
setlocal enabledelayedexpansion

:: Set the minimum coverage threshold (same as in CI)
set MIN_COVERAGE_PERCENT=80

:: Check if cargo-tarpaulin is installed
cargo tarpaulin --version >nul 2>&1
if %ERRORLEVEL% NEQ 0 (
    echo Installing cargo-tarpaulin...
    cargo install cargo-tarpaulin --version 0.25.0
)

:: Create coverage directory if it doesn't exist
if not exist coverage mkdir coverage

echo Running tests with coverage...
cargo tarpaulin --out Xml --output-dir coverage --workspace --exclude-files "tests/*" --verbose

:: Generate JSON report for coverage threshold checking
echo Running tests for coverage threshold check...
cargo tarpaulin --out Json --output-dir coverage --workspace --exclude-files "tests/*" --verbose

:: Check if the JSON report file exists
if not exist coverage\tarpaulin-report.json (
    echo Error: Coverage report file not found!
    exit /b 1
)

:: Check coverage threshold using the JSON file
echo Checking coverage threshold...
for /f "tokens=*" %%a in ('type coverage\tarpaulin-report.json ^| findstr "line_coverage"') do (
    set line=%%a
    :: Extract the number, multiply by 100, and remove decimal part
    set line=!line:line_coverage":=!
    set line=!line:,=!
    set coverage_value=!line:~1!
    set coverage_value=!coverage_value:~0,-1!
    :: Multiply by 100 (using powershell for floating point math)
    for /f %%b in ('powershell -command "[math]::Floor(!coverage_value! * 100)"') do (
        set coverage_value=%%b
    )
)

echo Coverage value: !coverage_value!%%

if !coverage_value! LSS %MIN_COVERAGE_PERCENT% (
    echo ❌ Test coverage is below threshold: !coverage_value!%% ^< %MIN_COVERAGE_PERCENT!%%
    exit /b 1
) else (
    echo ✅ Test coverage meets threshold: !coverage_value!%% ^>= %MIN_COVERAGE_PERCENT!%%
)

echo Coverage report saved to: coverage\cobertura.xml
