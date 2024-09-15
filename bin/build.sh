#!/bin/bash

# Check if the file 'secret.sh' exists in the current directory
if [ -f "./secret.sh" ]; then
    # If the file exists, source it to set the environment variables
    source ./secret.sh
    # Check if the environment variables are set
    if [ -z "$TAURI_SIGNING_PRIVATE_KEY" ] || [ -z "$TAURI_SIGNING_PRIVATE_KEY_PASSWORD" ]; then
        # If the environment variables are not set, return an error
        echo "Error: TAURI_SIGNING_PRIVATE_KEY or TAURI_SIGNING_PRIVATE_KEY_PASSWORD not set in secret.sh."
        exit 1
    else
        # If the environment variables are set, continue with the build process
        echo "Building the application..."
        # Run the build command
        npm run tauri build
        # Check if the build was successful
        if [ $? -eq 0 ]; then
            # If the build was successful, return a success message
            ./bin/post-build.sh
            echo "Build successful."
        else
            # If the build failed, return an error
            echo "Error: Build failed."
            exit 1
        fi
    fi
else
    # If the file does not exist, return an error
    echo "Error: secret.sh not found in the current directory."
    exit 1
fi
