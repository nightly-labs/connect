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

# First, get the tag reference
TAG_URL="https://api.github.com/repos/grafana/grafana/git/refs/tags/v$GRAFANA_VERSION_CLEAN"
echo "Fetching from tag URL: $TAG_URL"
TAG_DATA=$(curl -s "$TAG_URL")
echo "Tag API Response:"
echo "$TAG_DATA"

# Get the SHA of the tag object
TAG_SHA=$(echo "$TAG_DATA" | grep -o '"sha": "[^"]*"' | head -1 | cut -d'"' -f4)
echo "Tag SHA: $TAG_SHA"

# Now get the actual commit SHA that this tag points to
TAG_URL="https://api.github.com/repos/grafana/grafana/git/tags/$TAG_SHA"
echo "Fetching tag details from: $TAG_URL"
TAG_DETAILS=$(curl -s "$TAG_URL")
echo "Tag Details Response:"
echo "$TAG_DETAILS"

# Extract the actual commit SHA
COMMIT_HASH=$(echo "$TAG_DETAILS" | grep -o '"sha": "[^"]*"' | tail -1 | cut -d'"' -f4)

if [ -z "$COMMIT_HASH" ] || [ "$COMMIT_HASH" = "null" ]; then
    echo "Failed to get proper commit hash for version $GRAFANA_VERSION_CLEAN"
    exit 1
fi

echo "Found commit hash: $COMMIT_HASH"

# Download the OpenAPI spec for the specific version
echo "Downloading OpenAPI spec for commit $COMMIT_HASH..."
SPEC_URL="https://raw.githubusercontent.com/grafana/grafana/$COMMIT_HASH/public/openapi3.json"
echo "Downloading from: $SPEC_URL"
curl -L -o $OPENAPI_GENERATOR_DIR/openapi3.json "$SPEC_URL"

if [ -f $OPENAPI_GENERATOR_DIR/openapi3.json ]; then
    echo "OPENAPI file downloaded successfully."
else
    echo "Failed to download OPENAPI file."
    exit 1
fi

# Get current user's UID and GID
USER_ID=$(id -u)
GROUP_ID=$(id -g)

echo "Running Docker to generate code..."
docker run --rm \
    --user $USER_ID:$GROUP_ID \
    -v ${PWD}/${OPENAPI_GENERATOR_DIR}:/local \
    openapitools/openapi-generator-cli generate \
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
    
    echo "Setting permissions for $TARGET_DIR..."
    chmod -R 777 "$TARGET_DIR"
    echo "Permissions set for all files and directories in $TARGET_DIR."
else
    echo "Code generation did not complete successfully; src directory not found."
    exit 1
fi