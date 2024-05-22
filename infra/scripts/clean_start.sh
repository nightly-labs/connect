#!/bin/bash

# Define the base directory as the path to the infra directory
BASE_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )/.." &> /dev/null && pwd )"

# Step [1]: For full clean start, remove the below directories
# Step [2]: Run the script
# Step [3]: Run the binary located in ./database/src/bin/tables_migration.rs

directories=(
    "$BASE_DIR/target"
    "$BASE_DIR/backups"
    "$BASE_DIR/logs"
    "$BASE_DIR/config"
)

for dir in "${directories[@]}"; do
    if [ ! -d "$dir" ]; then
        echo "Creating directory $dir"
        mkdir -p "$dir"
    fi

    sudo chown -R $USER:$USER "$dir"
    chmod -R 777 "$dir"
done

# Configuration file path
pgbackrest_conf="$BASE_DIR/config/pgbackrest.conf"

# Call the setup script with the configuration file path
echo "Generating pgbackrest_config..."
"$BASE_DIR/scripts/generate_pgbackrest_config.sh" "$pgbackrest_conf"

# Change to the infra directory where the docker-compose.yaml file is located
cd "$BASE_DIR"

# Define the TimescaleDB image name as a variable
TIMESCALEDB_IMAGE="timescale/timescaledb-ha:pg15-ts2.10"

# Define PostgreSQL data directory
POSTGRESQL_DATA_DIR="/home/postgres/pgdata/data"
# PostgreSQL configuration file
POSTGRESQL_CONF="$POSTGRESQL_DATA_DIR/postgresql.conf"
BACKUP_MARKER="db"

# Change to the infra directory where the docker-compose.yaml file is located
cd "$BASE_DIR"

# Start Docker Compose in detached mode
echo "Starting Docker Compose..."
sudo docker compose up -d --no-deps --force-recreate --remove-orphans
sleep 5

# Function to check database readiness
wait_for_db_ready() {
  echo "Waiting for TimescaleDB to be ready..."
  local timeout=20
  local container_id=""
  for (( i=0; i<timeout; i++ )); do
    container_id=$(docker ps --filter "ancestor=$TIMESCALEDB_IMAGE" --format "{{.ID}}")
    if [[ -z "$container_id" ]]; then
      echo "No container found for $TIMESCALEDB_IMAGE, retrying..."
      sleep 2
      continue
    fi
    if sudo docker logs $container_id 2>&1 | grep -q "database system is ready to accept connections"; then
      echo "TimescaleDB is now ready."
      return 0
    fi
    echo "Waiting... ($i)"
    sleep 5
  done
  echo "Timeout waiting for TimescaleDB to be ready."
  return 1
}

# Wait for TimescaleDB to be ready
if wait_for_db_ready; then
  container_id=$(docker ps --filter "ancestor=$TIMESCALEDB_IMAGE" --format "{{.ID}}")
  if [[ -z "$container_id" ]]; then
    echo "Failed to find a running container for $TIMESCALEDB_IMAGE. Exiting."
    exit 1
  fi
  echo "Updating PostgreSQL configuration..."
  sudo docker exec -u root $container_id bash -c "
    echo -e '\n# Custom PostgreSQL Configurations' >> $POSTGRESQL_CONF
    echo 'archive_mode = on' >> $POSTGRESQL_CONF
    echo 'archive_command = '\"'pgbackrest --stanza=db archive-push %p'\"'' >> $POSTGRESQL_CONF
    echo 'max_wal_senders = 3' >> $POSTGRESQL_CONF
    echo 'wal_level = logical' >> $POSTGRESQL_CONF
  "
  echo "PostgreSQL configuration updated."

  # Restart docker to apply the changes
  echo "Restarting the container..."
  sudo docker restart $container_id
else
  echo "Failed to confirm TimescaleDB readiness. Check logs for more details."
fi

# Wait for TimescaleDB to be ready after the restart
if wait_for_db_ready; then
  echo "Database is ready. Proceeding with pgBackRest setup..."
  container_id=$(docker ps --filter "ancestor=$TIMESCALEDB_IMAGE" --format "{{.ID}}")
  if [[ -z "$container_id" ]]; then
    echo "Failed to find a running container for $TIMESCALEDB_IMAGE after restart. Exiting."
    exit 1
  fi
  # Execute pgBackRest stanza-create
  sudo docker exec -u root $container_id bash -c "
    pgbackrest --stanza=$BACKUP_MARKER --log-level-console=info --pg1-path=/home/postgres/pgdata/data --repo1-path=/var/lib/pgbackrest stanza-create
  "
  echo "pgBackRest stanza-create executed."

  # Fix permissions for the pgBackRest backup directory
  sudo docker exec -u root $container_id bash -c "
    chown -R postgres:postgres /var/lib/pgbackrest
    chmod -R 700 /var/lib/pgbackrest
    mkdir -p /var/log/pgbackrest
    chown -R postgres:postgres /var/log/pgbackrest
    chmod -R 770 /var/log/pgbackrest
    chown -R postgres:postgres /tmp/pgbackrest
    chmod -R 770 /tmp/pgbackrest
    # chown -R postgres:postgres /var/lib/pgbackrest/backup/$BACKUP_MARKER
    # chmod -R 770 /var/lib/pgbackrest/backup/$BACKUP_MARKER
  "
  echo "Permissions fixed for pgBackRest backup and log directories."


  # Execute pgBackRest check
  sudo docker exec -u postgres $container_id bash -c "
    pgbackrest --stanza=$BACKUP_MARKER --log-level-console=info check
  "
  echo "pgBackRest check executed successfully."

#   sudo docker exec -u root $container_id bash -c "
#     cd /var/lib/pgbackrest/backup
#     ls -la
#   "

else
  echo "Failed to confirm TimescaleDB readiness after restart. Check logs for more details."
  exit 1
fi