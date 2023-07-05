---
title: Establishing a Connection
slug: application/connect
---

import Tabs from '@theme/Tabs';
import TabItem from '@theme/TabItem';

:::info
This part of documentation is targeted to applications that want to implement nightly connect
as wallet interface.
:::

To get started, we need to connect the user to the application.
In order to do so, application generates the sessionId, a unique id that identifies each connection.

---

This process is initialized by one side displaying a sessionId through QR code (see the screenshot).
The other peer needs just to scan the QR code on its device. Extension wallets are auto detected so you are always up to date and dont need to upgrade your dapp.

![ConnectImage](../../static/img/connect.png#connectImage)

### Connect

Server sends a request to connect with the client. On the client side all information passed from application regarding the request will be displayed (website name, url, data).

Application builds a connection using `build()` function that returns interface to communicated with remote user.
To start sending request like `signTransaction` user first need to connect to session.
Once user establishes connection, application will get public key and the connection is now confirmed.

API of application client is fit to match currently existing standards of corresponding blockchains

```js
interface AppMetadata {
  name: string;
  url?: string;
  description?: string;
  icon?: string; // Url of app image
  additionalInfo?: string;
}

interface AppBaseInitialize {
  appMetadata: AppMetadata;
  network: Network;
  url?: string; // Relay endpoint default nc2.nightly.app
  timeout?: number;
  persistentSessionId?: string; // Apps can reuse old connection to skip connect step.
  persistent?: boolean; // Makes session valid for 14 days
}
```

<Tabs>
<TabItem value="Solana" label="Solana">

```js
import { AppSolana } from '@nightlylabs/nightly-connect-solana'

type AppSolanaInitialize = Omit<AppBaseInitialize, 'network'>

const app: AppSolana = await AppSolana.build(TEST_APP_INITIALIZE) // build take argument of AppSolanaInitialize type
app.on('userConnected', async (userPublicKeys) => {
  console.log(userPublicKeys)
})
```

</TabItem>

<TabItem value="SUI" label="SUI">

```js
import { AppSui } from '@nightlylabs/nightly-connect-sui'

type AppSuiInitialize = Omit<AppBaseInitialize, 'network'>

const app: AppSui = await AppSui.build(TEST_APP_INITIALIZE)
app.on('userConnected', async (userPublicKeys) => {
  console.log(userPublicKeys)
})
```

</TabItem>
</Tabs>

### Disconnect

:::info
Both client and application can initiate disconnection.
User can force session termination in case of abuse.
Only when application disconnects and session is not persistent, session is completely removed.
:::
