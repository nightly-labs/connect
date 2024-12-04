#!/bin/bash

# Source both .env files
if [ -f .env ]; then
    export $(cat .env | xargs)
fi

# Source Grafana version from the other directory
if [ -f ../grafana/.env ]; then
    export $(cat ../grafana/.env | xargs)
elif [ -f ../grafana-client-gen/.env ]; then
    export $(cat ../grafana-client-gen/.env | xargs)
fi

if [ -z "$GRAFANA_VERSION" ]; then
    echo "Error: GRAFANA_VERSION not found in environment variables"
    exit 1
fi

echo "Build path: $OPENAPI_GENERATOR_DIR"
echo "Language: $OPENAPI_LANGUAGE"
echo "Grafana Version: $GRAFANA_VERSION"

# Remove v prefix if present in version
GRAFANA_VERSION_CLEAN=${GRAFANA_VERSION#v}

echo "Removing existing build directory..."
rm -rf $OPENAPI_GENERATOR_DIR

echo "Setting up new build directory..."
mkdir -p $OPENAPI_GENERATOR_DIR

# Get the commit hash for the version tag
echo "Getting commit hash for Grafana version $GRAFANA_VERSION_CLEAN..."
COMMIT_HASH=$(curl -s "https://api.github.com/repos/grafana/grafana/git/refs/tags/v$GRAFANA_VERSION_CLEAN" | grep -o '"sha": "[^"]*' | cut -d'"' -f4)

if [ -z "$COMMIT_HASH" ]; then
    echo "Failed to get commit hash for version $GRAFANA_VERSION_CLEAN"
    exit 1
fi

echo "Found commit hash: $COMMIT_HASH"

# Download the OpenAPI spec for the specific version
echo "Downloading OpenAPI spec for commit $COMMIT_HASH..."
curl -o $OPENAPI_GENERATOR_DIR/openapi3.json "https://raw.githubusercontent.com/grafana/grafana/$COMMIT_HASH/public/openapi3.json"

if [ -f $OPENAPI_GENERATOR_DIR/openapi3.json ]; then
    echo "OPENAPI file downloaded successfully."
else
    echo "Failed to download OPENAPI file."
    exit 1
fi

echo "Running Docker to generate code..."
docker run --rm -v ${PWD}/${OPENAPI_GENERATOR_DIR}:/local openapitools/openapi-generator-cli generate \
-i /local/openapi3.json \
-g $OPENAPI_LANGUAGE \
-o /local/grafana-rust-client

echo "Code generation complete."

if [ -d "$OPENAPI_GENERATOR_DIR/grafana-rust-client/src" ]; then
    echo "Removing unwanted files..."
    rm -rf "$OPENAPI_GENERATOR_DIR/grafana-rust-client/.openapi-generator"
    rm -rf "$OPENAPI_GENERATOR_DIR/grafana-rust-client/.openapi-generator-ignore"
    rm -rf "$OPENAPI_GENERATOR_DIR/grafana-rust-client/.travis.yml"
    rm -f "$OPENAPI_GENERATOR_DIR/grafana-rust-client/.gitignore"
    rm -f "$OPENAPI_GENERATOR_DIR/grafana-rust-client/git_push.sh"
    
    echo "Copying generated package to the target directory..."
    rm -rf "$TARGET_DIR"
    cp -r "$OPENAPI_GENERATOR_DIR/grafana-rust-client" "$TARGET_DIR"
    echo "Files copied successfully to $TARGET_DIR."
    
    echo "Setting full permissions for $TARGET_DIR..."
    chmod -R 777 "$TARGET_DIR"
    echo "Permissions set to 777 for all files and directories in $TARGET_DIR."
else
    echo "Code generation did not complete successfully; src directory not found."
    exit 1
fi