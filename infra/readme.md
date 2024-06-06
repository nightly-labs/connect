# Overview

In order to store our data, we have decided to use PostgreSQL, and specifically self-hosted local deployment of extension to database called [TimescaleDB](https://docs.timescale.com/self-hosted/latest/). This extension is designed to handle time-series data, which is the main type of data we are going to store.

As this is Rust based project we had to utilize a crate to interact with the database. We have chosen Rust version of [sqlx](https://github.com/launchbadge/sqlx) which is a general purpose driver to interact with different types of databases.

# Local deployment

Make sure you have installed required tools as specified in the [Requirements](../readme.md#requirements) section.

If this is your first start of the project, you might want to start docker using custom script ```./scripts/clean_start.sh```. What does this scripts do:

- Create all folders which will be mounted to the docker container
- Setup backups using [pgBackRest](https://pgbackrest.org/)

> [!NOTE] 
> When starting the database docker we will be using custom entrypoint as original timescaledb container exits if something happens to the db, which will prevent us from accessing pgBackRest tools to lets say backup or restore db.

1. Setup ```ENV``` variables in ```./connect/infra/.env``` file.
   - ```ONLY_DATABASE``` - by default docker will start all services, if you want to skip them simply change this value to ```TRUE```.
   - Most of the database settings are already set to local development, but you can change them if needed. We don't recommend changing the variable ```PG_DATA```.
   - Be careful with ```TIMESCALEDB_IMAGE```, current version was proved to work with the rest of the services, but if you want to change it, make sure it is compatible with the rest of the services.
   - Ofelia configuration is required to properly receive notifications with backup schedule status. Invalid configuration won't crush the system, backup will be still created, but you won't be able to receive notifications. If you want to change ```backup``` schedule, you can do it in ```docker-compose.yaml``` labels section.

2. Navigate to the infra folder:

    ```bash
    cd ~/connect/infra
    ```

3. Run startup script
    
    ```bash
    ./scripts/clean_start.sh
    ```

> [!NOTE] 
> It is needed to run ```clean_start.sh``` script only once, running it again shouldn't cause any issues, but it is not needed.

4. Check if the containers are running:

    ```bash
    docker ps
    ```

5. To stop the containers:

    With only db running: 

    ```bash
    docker compose down
    ```

    With all services running:

    ```bash
    docker compose --profile full down
    ```

6. Next time you run the containers you can use:

    For only db:

    ```bash
    docker compose up
    ```

    For all services:

    ```bash
    docker compose --profile full up
    ```

After the data in container is all setup, the next step is to run the migrations. You can do this by running the following command:

```bash
cargo run --bin tables_migration
```

This command executes sql commands from ```./connect/database/migrations``` folder. You can check if the tables were created by connecting to the database using psql or any other tool of your choice.

> [!IMPORTANT]  
> You can add more migrations but you can't modify old ones once they are executed. If you want to modify the migration, you have to create a new one.


# Database backup and restoration

> [!WARNING] 
> Each of the provided methods differ in usage and purpose. While this short guide will provide you with basic information, it is recommended to read the official documentation of the tools used. Wrong usage of the tools may result in permanent data loss. 

## \#(Option 1 | Recommended) Scheduled Backups (Ofelia with pgBackRest)

### Backup

Running all services should start the ``Ofelia`` service which is responsible for creating scheduled backups. You can check if the service is running by executing:

```bash
docker logs -f $(docker ps --filter "ancestor=mcuadros/ofelia:988d988" --format "{{.ID}}")
```

You should see some messages about the backup schedule. By our setup a backup is created every day at 00:00 UTC and a differential backup every 15 minutes. Differential backups are deleted after full backup on which they are based is created. You may substitute backup commands with whole scripts to gather more info about the backup process.

You can also use commands to trigger backup by yourself, the same ones used by Ofelia:

For ```differential backup```:
```bash
    docker exec -it $(docker ps --filter "ancestor=timescale/timescaledb-ha:pg15-ts2.10" --format "{{.ID}}") pgbackrest --stanza=db --type=diff --log-level-stderr=info backup
```

and for ```full backup```:

```bash
    docker exec -it $(docker ps --filter "ancestor=timescale/timescaledb-ha:pg15-ts2.10" --format "{{.ID}}") pgbackrest --stanza=db --type=full --log-level-stderr=info backup
```

### Restore

In order to restore the database using pgBackRest, you need to have a backup created by pgBackRest. Check your current backup status by checking those:

 - ```./infra/backups/backup/db``` - folder containing all backups
 - ```./infra/logs/pgbackrest``` - folder containing pgBackRest logs
 - ```./infra/ofelia_logs``` - containing logs from all ofelia jobs

In order to restore the database, you have to stop the database inside the container:

```bash
    docker exec -it $(docker ps --filter "ancestor=timescale/timescaledb-ha:pg15-ts2.10" --format "{{.ID}}  ")  pg_ctl -D /home/postgres/pgdata/data stop
```

Check if database was stopped and container is still running:

```bash
    docker exec -it $(docker ps --filter "ancestor=timescale/timescaledb-ha:pg15-ts2.10" --format "{{.ID}}")  pg_ctl -D /home/postgres/pgdata/data status
```

You should see:

```bash
    pg_ctl: no server running
```

The next step would be to remove content of the data folder:

```bash
    docker exec -it $(docker ps --filter "ancestor=timescale/timescaledb-ha:pg15-ts2.10" --format "{{.ID}}") rm -rI /home/postgres/pgdata/data
```

WIth all of this you can perform the restoration:

```bash
    docker exec -it $(docker ps --filter "ancestor=timescale/timescaledb-ha:pg15-ts2.10" --format "{{.ID}}")  pgbackrest --stanza=db --log-level-stderr=info restore
```

Read more about restoring database using pgBackRest [here](https://pgbackrest.org/user-guide.html#restore).

## \#(Option 2) Manual backup using pg_dump and pg_restore

### Backup (pg_dump)

If you need to create a backup only for certain database, you can use the following command:

```bash
docker exec -it $(docker ps --filter "ancestor=timescale/timescaledb-ha:pg15-ts2.10" --format "{{.ID}}") sh -c 'pg_dump -U admin1234 -F t -d connect_db -f /var/lib/pgbackrest/manual_backup/connect_db_pg_dump_$(date "+%Y%m%d_%H%M%S").tar'
```

Navigate to ```./infra/backups/manual_backup``` to find your backup file.
You may read more about backup command at [PostgreSQL documentation](https://www.postgresql.org/docs/15/app-pgdump.html).

### Restore (pg_restore)

Replace ```backup_file_name.tar``` with the name of the backup file you want to restore.

```bash
docker exec -it $(docker ps --filter "ancestor=timescale/timescaledb-ha:pg15-ts2.10" --format "{{.ID}}") sh -c 'pg_restore -U admin1234 -Ft -d connect_db < /var/lib/pgbackrest/manual_backup/backup_file_name.tar'
```
You may read more about restore command at [PostgreSQL documentation](https://www.postgresql.org/docs/15/app-pgrestore.html).

### (Bonus) Backup and restoration using pg_dumpall and psql

You may want to create a backup of all databases, for this you can use the following command:

```bash
docker exec -it $(docker ps --filter "ancestor=timescale/timescaledb-ha:pg15-ts2.10" --format "{{.ID}}") sh -c 'pg_dumpall -U admin1234 -f /var/lib/pgbackrest/manual_backup/all_dbs_pg_dumpall_$(date "+%Y%m%d_%H%M%S").sql'
```

Navigate to ```./infra/backups/manual_backup``` to find your backup file.
You may read more about all databases backup command at [PostgreSQL documentation](https://www.postgresql.org/docs/15/app-pg-dumpall.html).

This command produces sql file which can be used to restore all databases, in order to do that we will utilize ```psql``` command:

```bash
docker exec -it $(docker ps --filter "ancestor=timescale/timescaledb-ha:pg15-ts2.10" --format "{{.ID}}") sh -c 'psql -U admin1234 -d postgres -f /var/lib/pgbackrest/manual_backup/backup_file_name.sql'
```

## \#(Option 3) Manual backup using pg_basebackup

In order to backup the whole cluster you can use ```pg_basebackup``` command. This command can be run on live database.

```bash
docker exec -it $(docker ps --filter "ancestor=timescale/timescaledb-ha:pg15-ts2.10" --format "{{.ID}}") sh -c 'pg_basebackup -h localhost -p 5432 -U admin1234  -D /var/lib/pgbackrest/manual_backup/pg_basebackup_$(date "+%Y%m%d_%H%M%S") -P -F t -Z 9 -X stream'
```

For the restoration part:

 1. Stop the database inside the container:
    ```bash
    docker exec -it $(docker ps --filter "ancestor=timescale/timescaledb-ha:pg15-ts2.10" --format "{{.ID}}  ")  pg_ctl -D /home/postgres/pgdata/data stop
    ```

 2. Check if database was stopped and container is still running:
    ```bash
    docker exec -it $(docker ps --filter "ancestor=timescale/timescaledb-ha:pg15-ts2.10" --format "{{.ID}}")  pg_ctl -D /home/postgres/pgdata/data status
    ```

    You should see:
    ```bash
    pg_ctl: no server running
    ```

> [!WARNING]  
> Backup Integrity: Ensure that the backup files are complete and not corrupted. If you used compression (e.g., gzip), verify that the archive can be successfully decompressed.

The database restoration will include replacement of the main data folder with the backup data. You may want to backup the data folder before proceeding with the restoration.

 3. Delete old data folder:
    ```bash
    docker exec -it $(docker ps --filter "ancestor=timescale/timescaledb-ha:pg15-ts2.10" --format "{{.ID}}") rm -rI /home/postgres/pgdata/data
    ```

4. Create a new data folder and set permissions:

    ```bash
    docker exec -it $(docker ps --filter "ancestor=timescale/timescaledb-ha:pg15-ts2.10" --format "{{.ID}}") mkdir /home/postgres/pgdata/data
    ```

    ```bash
    docker exec -it $(docker ps --filter "ancestor=timescale/timescaledb-ha:pg15-ts2.10" --format "{{.ID}}") chown -R postgres:postgres /home/postgres/pgdata/data
    ```

5. Navigate to directory with backup files and copy the backup to the data folder:

    ```bash
    docker exec -it $(docker ps --filter "ancestor=timescale/timescaledb-ha:pg15-ts2.10" --format "{{.ID}}") sh -c 'cd /var/lib/pgbackrest/manual_backup/your_pg_basebackup && tar -zxvf base.tar.gz -C /home/postgres/pgdata/data && tar -zxvf pg_wal.tar.gz -C /home/postgres/pgdata/data/pg_wal'
    ```

6. Restart docker container:

    ```bash
    docker restart $(docker ps --filter "ancestor=timescale/timescaledb-ha:pg15-ts2.10" --format "{{.ID}}")
    ```