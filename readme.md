<!-- # Nightly Connect -->

<p align="center">
    <img src="./images/nightly_connect/Nightly Connect Logo Circle.svg" height="96">
</p>

[![Build Status][actions-badge]][actions-url]
[![Discord Chat][discord-badge]][discord-url]

[actions-badge]: https://github.com/nightly-labs/connect/actions/workflows/connect-test-production.yml/badge.svg
[actions-url]: https://github.com/nightly-labs/connect/actions/workflows/connect-test-production.yml
[discord-badge]: https://img.shields.io/discord/500028886025895936.svg?logo=discord&style=flat-square
[discord-url]: https://discord.gg/aWAbWjWVdY


# Overview

Nightly Connect is a permissionless, open-source solution that serves as both a wallet adapter and a bridge wallet, enabling connections through QR codes or deep links.

We built this tool to lighten dApp developers in continuously adding new wallets. Now, after implementing Nightly Connect once, all standard-compliant wallets will be added automatically without any action required from dApp.

# Server

Connect service was written in <a href="https://www.rust-lang.org/">Rust</a> language and uses <a href="https://axum.rs/">Axum</a> as a web framework. The service is designed to be lightweight and fast, with a focus on security and performance. It has been extended to allow applications to track the statistics of their users' interactions with the service.

From now on those two names will be used to distinguish between the two versions of the service:

- ### Nightly Connect
  Basic but fully functional relay service with accompanying packages.

- ### Nightly Cloud
  Extended version of the service which utilizes TimescaleDB along with Grafana to visualize the data for registered apps integrating Nightly Connect.

# Local deployment

If you only want to implement Nightly Connect in your dApp instead, you may refer to the documentation [here](https://connect.nightly.app/docs/).

## Prerequisites

As a prerequisite, you need to have the following installed on your machine:

- [Rust](https://www.rust-lang.org/tools/install)
- [pnpm](https://pnpm.io/installation)

Nightly Cloud only:
- [Docker Compose](https://docs.docker.com/compose/install/)
- You may also need to install terminal to interact with the database like [psql](https://www.timescale.com/blog/how-to-install-psql-on-mac-ubuntu-debian-windows/) or any other tool of your choice.


All Rust development is done using the latest stable version of Rust. You may check your current version by running:

```bash
rustc --version
```

If needed you can update it by running:

```bash
rustup update
```

## How to run Nightly Connect

1. Clone the repository:

    Recommended way to clone the repository is to use SSH:
    ```bash
    git clone git@github.com:nightly-labs/connect.git
    ```

    If you prefer less secure option using HTTPS:
    ```bash
    git clone https://github.com/nightly-labs/connect.git
    ```

2. Navigate to the main ```.env``` file located in the main catalog of the repository and check if the ```ONLY_RELAY_SERVICE``` value is set to ```TRUE```

    Along with the variable ```ENV```, those two are the only variables which will be used by the Nightly Connect service.

3. Build and run:

    For maximum performance, you can build the project with the following command:

    ```bash
    cargo build --release --bin nightly-connect-server
    ```

    If build is failing you may want to try and run tests first:

    ```bash
    cargo test
    ```

    After building the project, you can run it with:

    ```bash
    cargo run --release --bin nightly-connect-server
    ```
    By default, the server will be running on ```http://127.0.0.1:6969``` which can be changed in ```./connect/server/src/bin/nightly-connect-server-rs```.

4. You may test if service is working by calling one of the http endpoints with curl command:

    ```bash
    curl -X GET http://127.0.0.1:6969/get_wallets_metadata
    ```

    If everything is working correctly, you should see huge JSON response with the metadata of all wallets.

5. (Optional) In order to interact with connect you might want to use our TS Sdk, if so navigate to ```./connect/sdk``` and then:

    First verify if the bindings are up to date:

    ```bash
    pnpm bindings
    ```

    Then install the dependencies:

    ```bash
    pnpm install
    ```

    After installing the dependencies navigate to the ```./connect/sdk/packages/base``` catalog. This directory includes all of the basic packages which are then used in SDK.

    ```bash
    pnpm test
    ```

    If you want to use the SDK in your project, you can build it with:

    ```bash
    pnpm build
    ```

## How to run Nightly Cloud

  As Nightly Cloud is an extended version of the service, it requires additional setup. The service is designed to be run in a Docker container, which is why you need to have Docker Compose installed on your machine. 

  1. Setup and start Database, you may refer to the [Database](./connect/infra/README.md) documentation for more information.

  2. Setup and start Grafana, you may refer to the [Grafana](./connect/grafana/README.md) documentation for more information.

  3. Setup ```ENV``` variables in ```./connect/.env``` file.
      - ```ENV``` - set to ```DEV``` for development environment. 
      - ```ONLY_RELAY_SERVICE``` - set to ```FALSE```.
      - ```NONCE``` - set to any random string, it will be used to encrypt passwords.
        
      Grafana related environment variables:

      - ```GRAFANA_BASE_PATH``` - set to the URL of the Grafana instance.
      - ```GRAFANA_CLIENT_LOGIN``` - admin login to the Grafana instance. It needs to be a Grafana admin as only this level of access allows to create users accounts.
      - ```GRAFANA_CLIENT_PASSWORD``` - admin password to the Grafana instance.

      Grafana JWT tokens setup:
      1. Navigate to ```./connect/jwt_tokens``` catalog.

      2. Generate a new rsa key pair:
          ```bash
          ssh-keygen -t rsa -b 4096 -m PEM -f grafana.key -N ""
          ```
      3. Extract public key from the private key:
          ```bash
          openssl rsa -in grafana.key -pubout -outform PEM -out grafana.key.pub
          ```

      4. Run the following command to get the keys in one line version:
          ```bash
          cat grafana.key | awk '{printf "%s", $0}' && echo "" && cat grafana.key.pub | awk '{printf "\n%s", $0}' && echo ""
          ```

      - ```JWT_SECRET``` - set to the private key generated in the previous step.
      - ```JWT_PUBLIC_KEY``` - set to the public key generated in the previous step.

      Mailer setup:
      - ```MAILER_ADDRESS``` - set to the email address from which the emails will be sent.
      - ```MAILER_PASSWORD``` - set to the password of the email address from which the emails will be sent.

        When it comes to the e-mails, the service is using the SMTP protocol to send them out, make sure that the email address you are using is configured to allow sending emails via SMTP.



> [!NOTE] 
> With env ```ENV``` set to ```DEV``` the service will skip any usage of the mailer.

  4. Build and run:

      Just like when running Nightly Connect, you can build the project with the following command:

      ```bash
      cargo build --release --bin nightly-connect-server
      ```

      After building the project, you can run it with:

      ```bash
      cargo run --release --bin nightly-connect-server
      ```

      By default all Rust rests utilizing the database are disabled by feature flag, to run all tests you can use custom command from ```./connect/.cargo/config.toml```:

      ```bash
      cargo test-integration
      ```

5. (Optional) In order to interact with Nightly Cloud you might want to use our TS Sdk, if so navigate to ```./connect/sdk``` and then:

    First verify if the bindings are up to date:

    ```bash
    pnpm bindings
    ```

    Then install the dependencies:

    ```bash
    pnpm install
    ```

    After installing the dependencies navigate to the ```./connect/sdk/packages/``` catalog. In order to fully utilize Nightly Cloud functionalities, you need to make use of those two packages:

    - ### Analytics 

      Used to send events to the Nightly Cloud service. They are then used to show the statistics of the users' interactions with the service for apps.

    - ### Cloud

      Used to interact with the Nightly Cloud endpoints related to the team and apps management.

    




      


    


