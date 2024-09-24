#!/bin/bash

# Define the paths to the relevant files
PACKAGE_JSON_PATH="./package.json"
CARGO_TOML_PATH="./src-tauri/Cargo.toml"
TAURI_CONF_JSON_PATH="./src-tauri/tauri.conf.json"

# Get the current version from tauri.conf.json
CURRENT_VERSION=$(jq -r '.version' "$TAURI_CONF_JSON_PATH")

# Echo the current version
echo "Current version: $CURRENT_VERSION"

# Ask the user to input a new version number
read -p "Enter new version number: " NEW_VERSION

# Update the version in package.json
jq --arg new_version "$NEW_VERSION" '.version = $new_version' "$PACKAGE_JSON_PATH" > "$PACKAGE_JSON_PATH.tmp" && mv "$PACKAGE_JSON_PATH.tmp" "$PACKAGE_JSON_PATH"

# Update the version in Cargo.toml
sed -i.bak "s/^version = \".*\"/version = \"$NEW_VERSION\"/" "$CARGO_TOML_PATH"
rm "$CARGO_TOML_PATH.bak"

# Update the version in tauri.conf.json
jq --arg new_version "$NEW_VERSION" '.version = $new_version' "$TAURI_CONF_JSON_PATH" > "$TAURI_CONF_JSON_PATH.tmp" && mv "$TAURI_CONF_JSON_PATH.tmp" "$TAURI_CONF_JSON_PATH"

# Run npm install to update dependencies
npm install

echo "Version updated to $NEW_VERSION in package.json, Cargo.toml, and tauri.conf.json."
