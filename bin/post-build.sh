#!/bin/bash

# Define the path to the latest.json file
LATEST_JSON_PATH="./latest.json"

# Get the version from package.json
if [ -f ./package.json ]; then
    VERSION=$(jq -r '.version' ./package.json)
else
    echo "Error: package.json not found."
    exit 1
fi

# Built files
INSTALLER_FILE="src-tauri/target/release/bundle/dmg/protestify_${VERSION}_aarch64.dmg"
UPDATER_FILE="src-tauri/target/release/bundle/macos/protestify.app.tar.gz"
SIGNATURE_FILE="src-tauri/target/release/bundle/macos/protestify.app.tar.gz.sig"

if [ ! -f "$INSTALLER_FILE" ]; then
    echo "Error: $INSTALLER_FILE not found."
    exit 1
fi

if [ ! -f "$UPDATER_FILE" ]; then
    echo "Error: $UPDATER_FILE not found."
    exit 1
fi

if [ ! -f "$SIGNATURE_FILE" ]; then
    echo "Error: $SIGNATURE_FILE not found."
    exit 1
fi

# Get info
CURRENT_DATE=$(date -u +"%Y-%m-%dT%H:%M:%SZ")
SIGNATURE=$(cat "$SIGNATURE_FILE")
URL="https://github.com/trogulja/protestify/releases/download/v$VERSION/protestify.app.tar.gz"
NOTES="Release notes for version $VERSION"

# Update the latest.json file
jq ".version = \"$VERSION\" | .notes = \"$NOTES\" | .pub_date = \"$CURRENT_DATE\" | .platforms.\"darwin-aarch64\".signature = \"$SIGNATURE\" | .platforms.\"darwin-aarch64\".url = \"$URL\"" \
   "$LATEST_JSON_PATH" > "$LATEST_JSON_PATH.tmp" && mv "$LATEST_JSON_PATH.tmp" "$LATEST_JSON_PATH"

echo "latest.json has been updated successfully."

# TODO: first make release, then update latest.json
# Create a new release using the gh CLI
gh release create "v$VERSION" "$INSTALLER_FILE" "$UPDATER_FILE" --generate-notes --title "Release v$VERSION"

echo "Release v$VERSION has been created and files have been uploaded successfully."
