#!/bin/bash

# Step [1]: For full clean start, remove the below directories
# Step [2]: Run the script
# Step [3]: Run the binary located in ./database/src/bin/tables_migration.rs

# Define the TimescaleDB image name as a variable
TIMESCALEDB_IMAGE="timescale/timescaledb-ha:pg15-ts2.10"

# Just in case stop the docker-compose
CONTAINER_ID=$(docker ps --filter "ancestor=$TIMESCALEDB_IMAGE" --format "{{.ID}}")

# Define maximum wait time in seconds (20 seconds)
MAX_WAIT=20
# Define sleep SLEEP_INTERVAL in seconds
SLEEP_INTERVAL=2

if [[ -n "$CONTAINER_ID" ]]; then
  echo "Container found for $TIMESCALEDB_IMAGE, initiating shutdown..."

  # Attempt to gracefully stop the container
  sudo docker compose down

  WAIT_TIME=0

  # Wait for the container to stop
  while docker ps --filter "id=$CONTAINER_ID" --format "{{.ID}}" | grep -q "$CONTAINER_ID"; do
    echo "Waiting for container $CONTAINER_ID to stop... ($WAIT_TIME seconds)"
    sleep $SLEEP_INTERVAL
    ((WAIT_TIME += SLEEP_INTERVAL))
    if ((WAIT_TIME >= MAX_WAIT)); then
      echo "Maximum wait time reached. Proceeding with the next steps."
      break
    fi
  done

  if ((WAIT_TIME < MAX_WAIT)); then
    echo "Container $CONTAINER_ID has been successfully stopped."
  fi
else
  echo "No running container found for $TIMESCALEDB_IMAGE."
fi

# Define the base directory as the path to the infra directory
BASE_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." &>/dev/null && pwd)"

directories=(
  "$BASE_DIR/target"
  "$BASE_DIR/backups"
  "$BASE_DIR/logs"
  "$BASE_DIR/config"
)

printf "\n------------- Tyding up the directories -------------\n"
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

# Function to check database readiness
wait_for_db_ready() {
  echo "Waiting for TimescaleDB to be ready..."
  local WAIT_TIME=0
  local CONTAINER_ID=""

  while ((WAIT_TIME < MAX_WAIT)); do
    CONTAINER_ID=$(docker ps --filter "ancestor=$TIMESCALEDB_IMAGE" --format "{{.ID}}")
    if [[ -z "$CONTAINER_ID" ]]; then
      echo "No container found for $TIMESCALEDB_IMAGE, retrying..."
      sleep $SLEEP_INTERVAL
      ((WAIT_TIME += SLEEP_INTERVAL))
      continue
    fi

    # Check the Docker logs for the readiness message
    if sudo docker logs $CONTAINER_ID 2>&1 | grep -q "database system is ready to accept connections"; then
      echo "TimescaleDB is now ready."
      return 0
    fi

    echo "Waiting for TimescaleDB to be ready... ${WAIT_TIME}s elapsed"
    sleep $SLEEP_INTERVAL
    ((WAIT_TIME += SLEEP_INTERVAL))
  done

  echo "Timeout waiting for TimescaleDB to be ready after ${MAX_WAIT}s."
  return 1
}

# Wait for TimescaleDB to be ready
if wait_for_db_ready; then
  printf "\n------------- Updating PostgreSQL configuration -------------\n"

  CONTAINER_ID=$(docker ps --filter "ancestor=$TIMESCALEDB_IMAGE" --format "{{.ID}}")
  if [[ -z "$CONTAINER_ID" ]]; then
    echo "Failed to find a running container for $TIMESCALEDB_IMAGE. Exiting."
    exit 1
  fi

  sudo docker exec -u root $CONTAINER_ID bash -c "
    echo -e '\n# Custom PostgreSQL Configurations' >> $POSTGRESQL_CONF
    echo 'archive_mode = on' >> $POSTGRESQL_CONF
    echo 'archive_command = '\"'pgbackrest --stanza=$BACKUP_MARKER archive-push %p'\"'' >> $POSTGRESQL_CONF
    echo 'max_wal_senders = 3' >> $POSTGRESQL_CONF
    echo 'wal_level = logical' >> $POSTGRESQL_CONF
  "
  echo "PostgreSQL configuration updated."

  # Restart docker to apply the changes
  echo "Restarting the container..."
  sudo docker restart $CONTAINER_ID
else
  echo "Failed to confirm TimescaleDB readiness. Check logs for more details."
fi

# Wait for TimescaleDB to be ready after the restart
if wait_for_db_ready; then
  printf "\n------------- Proceeding with pgBackRest setup -------------\n"
  CONTAINER_ID=$(docker ps --filter "ancestor=$TIMESCALEDB_IMAGE" --format "{{.ID}}")
  if [[ -z "$CONTAINER_ID" ]]; then
    echo "Failed to find a running container for $TIMESCALEDB_IMAGE after restart. Exiting."
    exit 1
  fi

  sleep 2
  # Execute pgBackRest stanza-create
  sudo docker exec -u root $CONTAINER_ID bash -c "
    pgbackrest --stanza=$BACKUP_MARKER --log-level-console=info --pg1-path=/home/postgres/pgdata/data --repo1-path=/var/lib/pgbackrest stanza-create
  "
  echo "pgBackRest stanza-create executed."

  # Fix permissions for the pgBackRest backup directory
  sudo docker exec -u root $CONTAINER_ID bash -c "
    chown -R postgres:postgres /var/lib/pgbackrest
    chmod -R 700 /var/lib/pgbackrest
    mkdir -p /var/log/pgbackrest
    chown -R postgres:postgres /var/log/pgbackrest
    chmod -R 770 /var/log/pgbackrest
    chown -R postgres:postgres /tmp/pgbackrest
    chmod -R 770 /tmp/pgbackrest
    mkdir -p /var/log/pgbackrest
    chown -R postgres:postgres /var/lib/pgbackrest
    chmod -R 770 /var/lib/pgbackrest
  "
  echo "Permissions fixed for pgBackRest backup and log directories."

  # Execute pgBackRest check
  sudo docker exec -u postgres $CONTAINER_ID bash -c "
    pgbackrest --stanza=$BACKUP_MARKER --log-level-console=info check
  "
  echo "pgBackRest check executed successfully."

else
  echo "Failed to confirm TimescaleDB readiness after restart. Check logs for more details."
  exit 1
fi
