# Local deployment

Make sure you have installed required tools as specified in the [Requirements](../readme.md#requirements) section.

## Starting the service

Before starting the docker container create a folder which is going to store the grafana state data.

1. Navigate to the `grafana` directory:


    ```bash
    cd ~/connect/grafana
    ```

2. Create a folder for the data:

    ```bash
    mkdir -p grafana-data
    ```

3. Change folder permissions:

    ```bash
    chmod -R 777 ./grafana-data
    ``` 

Then you can start the service by running:

```bash
docker compose up
```

Check if you can access the Grafana service by navigating to [http://localhost:3005](http://localhost:3005) in your browser.

> [!IMPORTANT]
> Mind you that default admin credentials are `admin:admin`, make sure to change to a more secure password.

## Basic grafana configuration

Once you have the service running you can configure the Grafana instance.
We will start the process by adding a data source.

1. Navigate to [http://localhost:3005/connections/datasources](http://localhost:3005/connections/datasources) in your browser.

2. Click on the `Add data source` button.

3. Choose the `PostgreSQL` data source.

4. Configure the data source with default values:

    - Connection
        - Host URL - in our current setup host url is the docker bridge network address, instead of container address. Yuo can use `ifconfig`, find entry `docker0` and use the `inet` address. You may also use command:

            ```bash
            docker network inspect bridge
            ```

            Copy the `Gateway` address and use it as the host url.
            The final version of `Host URL` should include this address with port, for example: `172.17.0.1:5432`.

        - Database name: `connect_db`
    - Authentication
        - Username: `admin1234`
        - Password: `password12345`
        - TLS/SSL Mode: `disable`
    - Additional settings
        - Version: `15`
        - TimescaleDB: `on`

5. You may set this as default data source at the top. Click on the `Save & Test` button.

6. Navigate to [http://localhost:3005/dashboards](http://localhost:3005/dashboards) to create a new folder. Name it for example `TEMPLATES`, for now the name of the folder does not matter.

    Inside the folder we will place the template dashboard which will get used for every new registered application.

7. Now find the `Import` button at the top right corner of the page.

    Template dashboard can be found here:
    
    ```bash
    ./connect/grafana/TEMPLATE_DASHBOARD.json
    ```

    Name the dashboard in the `Name` field, for example `TEMPLATE`, name of the dashboard does not really matter. What matters is the dashboard uid which is unique and will be used in the connect service. 

    The `UID` field is hardcoded in `~/connect/server/src/statics.rs` under `DASHBOARD_TEMPLATE_UID` variable. Make sure that your dashboard uid is the same as the one in the file.

    As the last step choose the data source you have created in the previous steps.


## Stopping the service

To stop the service run:

```bash
docker compose down
```

## Updating the existing dashboards

For now if any dashboard update is needed, you need to manually update the every dashboard in every team folder.

## Login with JWT token

Grafana config `grafana.ini` was configured to use JWT token for login. You can test this functionality by navigating to `jwt_keys` folder and running the `Python` script with following command:

```bash
/bin/python3 /home/giems/connect/jwt_keys/test.py
```

This command might not work on your machine, adjust it based on your setup.

After executing the script you will get quite long link which you can copy-paste to your browser.
   