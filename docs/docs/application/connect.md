---
title: Establishing a Connection
slug: application/connect
---

import Tabs from '@theme/Tabs';
import TabItem from '@theme/TabItem';

To get started, we need to connect the user to the application.
In order to do so, application generates the sessionId, a unique id that identifies each connection. This encrypted key ensures the connection is secure.

---

This process is initialized by one side displaying a sessionId through QR code (see the screenshot).
The other peer needs just to scan the QR code on its device.

![ConnectImage](../../static/img/connect.png#connectImage)

### Connect

Server sends a request to connect with the client. On the client side all information passed from application regarding the request will be displayed (website name, url, data).

Application builds a connection using `build()` function.
Once user accepts the Connection request (clicks on Connect), application will get public key and the connection is now confirmed.

```js
interface AppMetadata {
  name: string;
  url?: string;
  description?: string;
  icon?: string;
  additionalInfo?: string;
}

interface AppBaseInitialize {
  appMetadata: AppMetadata;
  network: Network;
  url?: string;
  timeout?: number;
  persistentSessionId?: string;
  persistent?: boolean;
}
```

<Tabs>
<TabItem value="Solana" label="Solana">

```js
import { AppSolana } from '@nightlylabs/nightly-connect-solana'

type AppSolanaInitialize = Omit<AppBaseInitialize, 'network'>

const app: AppSolana = await AppSolana.build(TEST_APP_INITIALIZE) // build take argument of AppSolanaInitialize type
```

</TabItem>

<TabItem value="SUI" label="SUI">

```js
import { AppSui } from '@nightlylabs/nightly-connect-sui'

type AppSuiInitialize = Omit<AppBaseInitialize, 'network'>

const app: AppSui = await AppSui.build(TEST_APP_INITIALIZE)
```

</TabItem>
</Tabs>

### Disconnect

:::info
Both client and application can initiate disconnection.<br />
Though when it is the client who disconnects, session will not be terminated.<br />
Only when application disconnects, the session will be closed.
:::
