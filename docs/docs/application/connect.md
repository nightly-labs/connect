---
title: Establishing a Connection
slug: application/connect
---

import Tabs from '@theme/Tabs';
import TabItem from '@theme/TabItem';

To get started, we need to connect the user to the application.
In order to do so, application generates the sesssionId, a unique id that identifies each connection. This encrypted key ensures the connection is secure.

---

This process is initialized by one side displaying a sessionId through QR code (see the screenshot).
The other peer needs just to scan the QR code on its device. In other cases the sessionId can be just copied (by using the Copy QR button).

![Welcome to Nightly](../../static/img/connect.png#connectImage)

### Connect

Server sends a request to connect with the client. On the client side all information passed from application regarding the request will be displayed (website name, url, data).

Application builds a connection using `build()` function.
Once user accepts the Connection request (clicks on Connect), application will get public key and the connection is now confirmed.

<Tabs>
<TabItem value="Solana" label="Solana">

```js
import { AppSolana } from '@nightlylabs/connect'

interface SolanaOnConnect {
  publicKey: SolanaPublicKey
}

interface SolanaAppInfo {
  application: string // 'Application name'
  description: string // 'Description'
  additionalInfo: string // Some Additional info
  icon: string // https://application/logo.png
  url?: string // default: wss://relay.nightly.app/app
  onUserConnect: (params: SolanaOnConnect) => void // userConnectedCallback
  timeout?: number //  40s default timeout (timer for Server answer request). On timeout throws Error 'Connection timed out'.
}

application = await AppSolana.build(testSolanaAppInfo)
```

</TabItem>

<TabItem value="Near" label="Near">

```js
import { AppNear } from '@nightlylabs/connect'

interface NearOnConnect {
  publicKey: NearPublicKey
  accountId: string // account ID required to build connection
}

interface NearAppInfo {
  application: string // 'Application name'
  description: string // 'Description'
  additionalInfo: string // Some Additional info
  icon: string // https://application/logo.png
  url?: string // default: wss://relay.nightly.app/app
  onUserConnect: (params: NearOnConnect) => void // userConnectedCallback
  timeout?: number //  40s default timeout (timer for Server answer request). On timeout throws Error 'Connection timed out'.
}

application = await AppNear.build(testNearAppInfo)
```

</TabItem>
</Tabs>

### Disconnect

:::info
Both client and application can initiate disconnection.<br />
Though when it is the client who disconnects, session will not be terminated.<br />
Only when application disconnects, the session will be closed.
:::
