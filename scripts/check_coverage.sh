#!/bin/bash
set -e

# Set the minimum coverage threshold (same as in CI)
MIN_COVERAGE_PERCENT=80

# Install tarpaulin if not already installed
if ! command -v cargo-tarpaulin &> /dev/null; then
    echo "Installing cargo-tarpaulin..."
    cargo install cargo-tarpaulin --version 0.25.0
fi

# Create coverage directory if it doesn't exist
mkdir -p coverage

echo "Running tests with coverage..."
cargo tarpaulin --out Xml --output-dir coverage --workspace --exclude-files "tests/*" --verbose

# Check coverage threshold
echo "Checking coverage threshold..."
# Run tarpaulin and capture json output to a file
cargo tarpaulin --out Json --output-dir coverage --workspace --exclude-files "tests/*" --verbose

# Extract the coverage value from the JSON file
if [ -f "coverage/tarpaulin-report.json" ]; then
    coverage_value=$(cat coverage/tarpaulin-report.json | jq '.summary.line_coverage * 100' | cut -d. -f1)
    echo "Coverage value: $coverage_value%"
    
    if [ "$coverage_value" -lt "$MIN_COVERAGE_PERCENT" ]; then
        echo "❌ Test coverage is below threshold: $coverage_value% < $MIN_COVERAGE_PERCENT%"
        exit 1
    else
        echo "✅ Test coverage meets threshold: $coverage_value% >= $MIN_COVERAGE_PERCENT%"
    fi
else
    echo "❌ Error: Coverage report file not found!"
    exit 1
fi

echo "Coverage report saved to: coverage/cobertura.xml"
