#!/bin/bash

# The first argument is the path where the pgbackrest.conf should be created/overwritten
config_path="$1"

# Check if the configuration path was provided
if [ -z "$config_path" ]; then
    echo "Usage: $0 <path_to_pgbackrest.conf>"
    exit 1
fi

# Creating the pgbackrest configuration file
echo "Creating pgbackrest configuration at $config_path"

# Use 'cat' to write the contents to the configuration file
cat > "$config_path" <<EOF
[db]
pg1-path=/home/postgres/pgdata/data
pg1-socket-path=/var/run/postgresql
[global]
repo1-retention-full=2
repo1-path=/var/lib/pgbackrest
EOF

echo "pgbackrest configuration has been created."
