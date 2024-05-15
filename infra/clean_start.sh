#!/bin/bash

# Step [1]: For full clean start remove the below directories
# Step [2]: Run the script
# Step [3]: Run the binary located in ./database/src/bin/tables_migration.rs
directories=(
    "./target"
    "./backups"
)

for dir in "${directories[@]}"; do
    if [ ! -d "$dir" ]; then
        echo "Creating directory $dir"
        mkdir -p "$dir"
    fi

    sudo chown -R $USER:$USER "$dir"
    chmod -R 755 "$dir"
done

sudo docker compose up --no-deps --force-recreate --remove-orphans
