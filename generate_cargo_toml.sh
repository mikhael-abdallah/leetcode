#!/bin/bash

# Package name
PACKAGE_NAME="leetcode"
VERSION="0.1.0"
EDITION="2024"

# Cargo.toml header
cat <<EOF > Cargo.toml
[package]
name = "$PACKAGE_NAME"
version = "$VERSION"
edition = "$EDITION"

EOF

# Find all .rs files and generate [[bin]] entries
find . -type f -name "*.rs" | while read -r file; do
    # Remove the './' prefix and the .rs extension
    bin_name=$(basename "$file" .rs)
    # Add to Cargo.toml
    echo "[[bin]]" >> Cargo.toml
    echo "name = \"$bin_name\"" >> Cargo.toml
    echo "path = \"$file\"" >> Cargo.toml
    echo >> Cargo.toml
done

echo "Cargo.toml generated successfully!"