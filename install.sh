#!/usr/bin/env bash

set -e

REPO="https://github.com/GHexxerBrdv/brahma"
BINARY_NAME="brahma"

echo "Installing $BINARY_NAME..."

# Check if Rust is installed
if ! command -v cargo >/dev/null 2>&1; then
    echo "Rust is not installed."
    echo "Installing Rust..."

    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

    # Load cargo into current shell
    source "$HOME/.cargo/env"
fi

echo "Rust detected."

echo "Installing $BINARY_NAME from GitHub..."

cargo install --git "$REPO" --locked

echo ""
echo "$BINARY_NAME installed successfully!"
echo ""

# Suggest PATH update if needed
if [[ ":$PATH:" != *":$HOME/.cargo/bin:"* ]]; then
    echo "Add this to your shell config:"
    echo 'export PATH="$HOME/.cargo/bin:$PATH"'
fi