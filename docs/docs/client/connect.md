---
title: Build & Connect
slug: client/connect
---

import Tabs from '@theme/Tabs';
import TabItem from '@theme/TabItem';

To get started, first we establish Connection with server `create()`. Create function takes optional parameters: `clientId?: string; url?: string; timeout?: number;`.

After that we can query the info with `getInfo()`, which required 1 argument, sessionId (the one from the QR code). Response received is `GetInfoResponse` type, it provides you with all required information on this connection request.

Once client decides to connect and approves the request, call the `connect()` method.

```js
export interface AppMetadata {
  name: string;
  url?: string;
  description?: string;
  icon?: string;
  additionalInfo?: string;
}

interface GetInfoResponse {
  responseId: string;
  network: Network;
  version: Version; // string
  appMetadata: AppMetadata;
}

type Connect = {
  publicKeys: string[],
  sessionId: string,
  notification?: Notification | undefined, // for notification purposes
  device?: Device | undefined,
  metadata?: string | undefined
}
```

### Build & Connect

<Tabs>
<TabItem value="Solana" label="Solana">

```js
import { ClientSolana } from '@nightlylabs/nightly-connect-solana'

const client: ClientSolana = await ClientSolana.create({
  url: RELAY_ENDPOINT // default: https://nc2.nightly.app
})
const info: GetInfoResponse = await client.getInfo(sessionId)

const msg: Connect = {
  publicKeys: ['1', '2'],
  sessionId: sessionId
}
await client.connect(msg)
```

</TabItem>

<TabItem value="SUI" label="SUI">

```js
import { ClientSui } from '@nightlylabs/nightly-connect-sui'

const client: ClientSui = await ClientSui.create({
  url: RELAY_ENDPOINT // default: https://nc2.nightly.app
})
const info: GetInfoResponse = await client.getInfo(sessionId)

const msg: Connect = {
  publicKeys: ['1', '2'],
  sessionId: sessionId
}
await client.connect(msg)
```

</TabItem>
</Tabs>

### Disconnect

:::info
Both client and application can initiate disconnection.<br />
Though when it is the client who disconnects, the session will not be terminated.<br />
Only when application disconnects, the session will be closed.
:::
