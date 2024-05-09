#!/bin/bash
if [ -f .env ]; then
    export $(cat .env | xargs)
fi

echo "Build path: "$OPENAPI_GENERATOR_DIR
echo "Language: "$OPENAPI_LANGUAGE

echo "Removing existing build directory..."
rm -rf $OPENAPI_GENERATOR_DIR

echo "Setting up new build directory and downloading the OpenAPI spec..."
mkdir -p $OPENAPI_GENERATOR_DIR
curl -o $OPENAPI_GENERATOR_DIR/openapi3.json https://raw.githubusercontent.com/grafana/grafana/main/public/openapi3.json

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

