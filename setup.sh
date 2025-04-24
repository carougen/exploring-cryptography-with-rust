#!/bin/bash

echo "🔧 Setting up the Exploring Cryptography with Rust project..."

# Exit on error
set -e

# Check for basic build tools (Linux)
if [[ "$OSTYPE" == "linux-gnu"* ]]; then
  if ! dpkg -s build-essential &>/dev/null; then
    echo "🛠️  build-essential not found. Installing..."
    sudo apt update
    sudo apt install -y build-essential
  else
    echo "✅ build-essential already installed."
  fi
fi

# Check Rust installation
if ! command -v cargo &> /dev/null
then
    echo "❌ Rust is not installed. Please install it via https://rustup.rs/"
    exit 1
fi

echo "✅ Rust is installed."

# Install mdBook and plugins
if ! command -v mdbook &> /dev/null
then
    echo "📚 Installing mdBook..."
    cargo install mdbook --version 0.4.45 --force
else
    echo "✅ mdBook already installed. (make sure it is version 0.4.45)"
fi

if ! command -v mdbook-katex &> /dev/null
then
    echo "➕ Installing mdbook-katex..."
    cargo install mdbook-katex
else
    echo "✅ mdbook-katex already installed."
fi

# Build all crates
echo "📦 Building all crates..."
cargo build --all

# Run tests
echo "🧪 Running tests..."
cargo test --all

# Build the book
echo "📘 Building the mdBook documentation..."
cd book
mdbook build
cd ..

echo "🎉 Setup complete!"
echo ""
echo "➡️  To serve the documentation locally, run:"
echo "   mdbook serve book"
echo ""
echo "▶️  To run examples:"
echo "   cargo run --example <example-name> --package <crate-name>"
