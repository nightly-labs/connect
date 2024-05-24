#!/bin/bash

# The first argument is the path where the pgbackrest.conf should be created/overwritten
config_path="$1"

# Check if the configuration path was provided
if [ -z "$config_path" ]; then
    echo "Usage: $0 <path_to_pgbackrest.conf>"
    exit 1
fi

# Source the .env
source .env

# Creating the pgbackrest configuration file
echo "Creating pgbackrest configuration at $config_path"

# Use 'cat' to write the contents to the configuration file
cat >"$config_path" <<EOF
[db]
pg1-path=${PG_DATA}/data
pg1-socket-path=/var/run/postgresql
[global]
repo1-retention-full=3
repo1-retention-diff=4
repo1-path=/var/lib/pgbackrest
[global:archive-push]
compress-level=3 
EOF

echo "pgbackrest configuration has been created."
