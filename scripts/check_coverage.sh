#!/bin/bash
set -e

# Set the minimum coverage threshold (same as in CI)
MIN_COVERAGE_PERCENT=80

# Install tarpaulin if not already installed
if ! command -v cargo-tarpaulin &> /dev/null; then
    echo "Installing cargo-tarpaulin..."
    cargo install cargo-tarpaulin
fi

# Create coverage directory if it doesn't exist
mkdir -p coverage

echo "Running tests with coverage..."
cargo tarpaulin --out Xml --output-dir coverage --workspace

# Check coverage threshold
echo "Checking coverage threshold..."
coverage_value=$(cargo tarpaulin --out Json --workspace | jq '.report.coverage' | sed 's/\..*$//')

if [ "$coverage_value" -lt "$MIN_COVERAGE_PERCENT" ]; then
    echo "❌ Test coverage is below threshold: $coverage_value% < $MIN_COVERAGE_PERCENT%"
    exit 1
else
    echo "✅ Test coverage meets threshold: $coverage_value% >= $MIN_COVERAGE_PERCENT%"
fi

echo "Coverage report saved to: coverage/cobertura.xml"
