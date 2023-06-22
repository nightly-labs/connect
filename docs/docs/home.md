---
title: Home
slug: /
---

import Tabs from '@theme/Tabs';
import TabItem from '@theme/TabItem';

<head>
    <meta name="DC.title" content="Nightly Connect - your key to the metaverse" />
    <meta name="title"content="Nightly Connect - future is multichain" />
    <meta property="og:description" content="Nightly Connect - protocol which establishes a connection on chain between Nightly Mobile Wallet and dApp through scanning QR code on NEAR, Solana and Aptos." />
    <meta property="twitter:description" content="Nightly Connect - protocol which establishes a connection on chain between Nightly Mobile Wallet and dApp through scanning QR code on NEAR, Solana and Aptos." />
    <meta name="DC.description" content="Nightly Wallet - The fastest multichain wallet to manage your tokens and NFTs via an extension or mobile app. Your key to the metaverse available on NEAR, Solana and Aptos." />
</head>

![Welcome to Nightly](../static/img/connect_landing_narrow.png)

### Nightly created to Connect

**Nightly Connect** is an easy and user friendly way to connect applications with client's wallet. Scan, connect and approve. In these few steps the application is connected to the wallet.

Secure way to communicate between wallets and dapps.
Nightly Connect can be easily adapted on any blockchain. Right now we are supporting wallets on Near and Solana.

Nightly Connect is a remote signer protocol which communicates securely between dapps and web3 wallets. The protocol establishes a remote pairing between two dapps and/or devices. Payloads are encrypted through a shared sessionId between the two sides. The connection is initiated by one peer displaying a QR Code and is established when the counter-party approves the request.

### Get started

:::info
Install with code below in your terminal to get started.

<Tabs>
<TabItem value="Solana" label="Solana">

Simply integrate Nightly Connect to your application or wallet with our [Nightly Solana Connect package](https://www.npmjs.com/package/@nightlylabs/nightly-connect-solana).

```bash
# Using NPM
npm i @nightlylabs/nightly-connect-solana

# Using Yarn
yarn add @nightlylabs/nightly-connect-solana

```

</TabItem>
<TabItem value="SUI" label="SUI">

Simply integrate Nightly Connect to your application or wallet with our [Nightly Sui Connect package](https://www.npmjs.com/package/@nightlylabs/nightly-connect-sui).

```bash
# Using NPM
npm i @nightlylabs/nightly-connect-sui

# Using Yarn
yarn add @nightlylabs/nightly-connect-sui
```

</TabItem>
</Tabs>

:::
