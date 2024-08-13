#!/bin/bash

# Step [1]: For full clean start, remove the below directories
# Step [2]: Run the script
# Step [3]: Run the binary located in ./database/src/bin/tables_migration.rs

# Source the .env
# Assuming env_loader.sh is in the same directory as this script
source "$(dirname "$0")/env_loader.sh"
read_env

# Just in case stop the docker-compose
CONTAINER_ID=$(docker ps --filter "ancestor=$TIMESCALEDB_IMAGE" --format "{{.ID}}")

# Define maximum wait time in seconds (20 seconds)
MAX_WAIT=20
# Define sleep interval in seconds
SLEEP_INTERVAL=2

if [[ -n "$CONTAINER_ID" ]]; then
  echo "Container found for $TIMESCALEDB_IMAGE, initiating shutdown..."

  # Attempt to gracefully stop the container
  docker compose --profile full down

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
  "${BASE_DIR}/${TIMESCALEDB_DATA}"
  "${BASE_DIR}/${TIMESCALEDB_BACKUPS}"
  "${BASE_DIR}/${TIMESCALEDB_LOGS}"
  "${BASE_DIR}/${TIMESCALEDB_PGBACKREST_CONFIG}"
  "${BASE_DIR}/${OFELIA_LOGS}"
)

printf "\n------------- Tidying up the directories -------------\n"
for dir in "${directories[@]}"; do
  if [ ! -d "$dir" ]; then
    echo "Creating directory $dir"
    mkdir -p "$dir"
  fi

  chown -R $USER:$USER "$dir"
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
POSTGRESQL_DATA_DIR="${PG_DATA}/data"  # Source this path from .env
# PostgreSQL configuration file
POSTGRESQL_CONF="$POSTGRESQL_DATA_DIR/postgresql.conf"
BACKUP_MARKER="db"

# Start Docker Compose in detached mode
echo "Starting Docker Compose..."

if [ "$ONLY_DATABASE" = "TRUE" ]; then
  echo "Starting only the TimescaleDB service..."
  docker compose up -d
else
  echo "Starting all services including Ofelia..."
  docker compose --profile full up -d
fi

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
    if docker logs $CONTAINER_ID 2>&1 | grep -q "database system is ready to accept connections"; then
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

  docker exec -u postgres $CONTAINER_ID bash -c "
    echo 'Using configuration file at: $POSTGRESQL_CONF'

    # Create a temporary file for the new configuration and check its validity
    TEMP_CONF=\$(mktemp)
    if [[ -z \$TEMP_CONF ]]; then
      echo 'Failed to create a temporary file. Exiting.'
      exit 1
    else
      echo 'Temporary file created at: '\$TEMP_CONF''
    fi

    # Read the config file and make changes line by line
    while IFS= read -r line || [[ -n \$line ]]; do
      if echo \"\$line\" | grep -q 'timescaledb.telemetry_level='; then
        echo 'timescaledb.telemetry_level=off'
      elif echo \"\$line\" | grep -q 'timescaledb.max_background_workers ='; then
        echo 'timescaledb.max_background_workers = 32'
      else
        echo \"\$line\"
      fi
    done < \"$POSTGRESQL_CONF\" > \"\$TEMP_CONF\"

    if [ -s \"\$TEMP_CONF\" ]; then
      echo 'Configuration file has been successfully updated.'
      mv \"\$TEMP_CONF\" \"$POSTGRESQL_CONF\"
    else
      echo 'Failed to update configuration file. Temporary file is empty.'
      rm -f \"\$TEMP_CONF\"
      exit 1
    fi
  "



  docker exec -u root $CONTAINER_ID bash -c "
    echo -e '\n# Custom PostgreSQL Configurations' >> $POSTGRESQL_CONF
    echo 'archive_mode = on' >> $POSTGRESQL_CONF
    echo 'archive_command = '\"'pgbackrest --stanza=$BACKUP_MARKER archive-push %p'\"'' >> $POSTGRESQL_CONF
    echo 'max_wal_senders = 10' >> $POSTGRESQL_CONF
    echo 'wal_level = logical' >> $POSTGRESQL_CONF
  "
  echo "PostgreSQL configuration updated."

  # Restart docker to apply the changes
  echo "Restarting the container..."
  docker restart $CONTAINER_ID
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
  docker exec -u root $CONTAINER_ID bash -c "
    pgbackrest --stanza=$BACKUP_MARKER --log-level-console=info --pg1-path=/home/postgres/pgdata/data --repo1-path=/var/lib/pgbackrest stanza-create
  "
  echo "pgBackRest stanza-create executed."

  # Fix permissions for the pgBackRest backup directory
  docker exec -u root $CONTAINER_ID bash -c "
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
    mkdir -p /var/lib/pgbackrest/manual_backup
    chown -R postgres:postgres /var/lib/pgbackrest/manual_backup
  "
  echo "Permissions fixed for pgBackRest backup and log directories."

  # Execute pgBackRest check
  docker exec -u postgres $CONTAINER_ID bash -c "
    pgbackrest --stanza=$BACKUP_MARKER --log-level-console=info check
  "
  echo "pgBackRest check executed successfully."

  # Run docker logs if the env is not github CI
  # if [[ -z "$CI_ENVIRONMENT" ]]; then
  #   echo "Not a CI, displaying the container logs..."
  #   docker logs -f $CONTAINER_ID
  # fi

  echo "Creating a restricted user for Grafana in the database..."

  # Verify the variables are set
  if [ -z "$GRAFANA_DB_USERNAME" ] || [ -z "$GRAFANA_DB_PASSWORD" ]; then
    echo "Error: GRAFANA_DB_USERNAME or GRAFANA_DB_PASSWORD is not set. Please set these variables."
    exit 1
  fi
  printf "DATABASE NAME: $POSTGRES_DB\n"

  docker exec -u postgres $CONTAINER_ID psql -d "$POSTGRES_DB" -c "CREATE USER $GRAFANA_DB_USERNAME WITH PASSWORD '$GRAFANA_DB_PASSWORD';"
  docker exec -u postgres $CONTAINER_ID psql -d "$POSTGRES_DB" -c "GRANT CONNECT ON DATABASE $POSTGRES_DB TO $GRAFANA_DB_USERNAME;"
  docker exec -u postgres $CONTAINER_ID psql -d "$POSTGRES_DB" -c "GRANT USAGE ON SCHEMA public TO $GRAFANA_DB_USERNAME;"
  docker exec -u postgres $CONTAINER_ID psql -d "$POSTGRES_DB" -c "GRANT SELECT ON ALL TABLES IN SCHEMA public TO $GRAFANA_DB_USERNAME;"
  docker exec -u postgres $CONTAINER_ID psql -d "$POSTGRES_DB" -c "ALTER DEFAULT PRIVILEGES IN SCHEMA public GRANT SELECT ON TABLES TO $GRAFANA_DB_USERNAME;"
  docker exec -u postgres $CONTAINER_ID psql -d "$POSTGRES_DB" -c "REVOKE DELETE ON ALL TABLES IN SCHEMA public FROM $GRAFANA_DB_USERNAME;"
  docker exec -u postgres $CONTAINER_ID psql -d "$POSTGRES_DB" -c "ALTER DEFAULT PRIVILEGES IN SCHEMA public REVOKE DELETE ON TABLES FROM $GRAFANA_DB_USERNAME;"

  echo "Restricted user for Grafana has been created with SELECT privileges only."

  # Check if the user was created successfully
  echo "Verifying the user creation and connection..."
  # This query will return a row if the user exists
  user_exists=$(docker exec -u postgres $CONTAINER_ID psql -d "$POSTGRES_DB" -tAc "SELECT 1 FROM pg_roles WHERE rolname = '$GRAFANA_DB_USERNAME';")

  if [[ "$user_exists" == "1" ]]; then
    echo "User $GRAFANA_DB_USERNAME exists in the database."
  else
    echo "User $GRAFANA_DB_USERNAME does not exist. Please check the creation process."
    exit 1
  fi

  # Check if the new user can connect and run a query
  docker exec -u postgres $CONTAINER_ID psql -U "$GRAFANA_DB_USERNAME" -d "$POSTGRES_DB" -c "SELECT 1;" &>/dev/null

  if [ $? -eq 0 ]; then
    echo "User $GRAFANA_DB_USERNAME created and verified successfully."
  else
    echo "Failed to verify user $GRAFANA_DB_USERNAME. Please check the PostgreSQL logs."
    exit 1
  fi
else
  echo "Failed to confirm TimescaleDB readiness after restart. Check logs for more details."
  exit 1
fi
