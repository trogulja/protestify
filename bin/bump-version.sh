#!/bin/bash

# Check for staged or unstaged changes
if [[ -n $(git status --porcelain) ]]; then
  echo "There are uncommitted changes in the repository. Please commit or stash them before bumping the version."
  exit 1
fi

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

# Run npm install to bump package-lock.json as well
npm install

echo "Version updated to $NEW_VERSION in package.json, Cargo.toml, and tauri.conf.json."

# Ask the user if they want to stage and commit the changes
read -n 1 -r -p "Do you want to stage and commit these changes? (y/n): " COMMIT_CHANGES

if [[ "$COMMIT_CHANGES" == "y" || "$COMMIT_CHANGES" == "Y" ]]; then
  git add .
  git commit -m "Bump version to $NEW_VERSION"
  echo "Changes have been staged and committed."
else
  echo "Skipping staging and committing changes."
fi
