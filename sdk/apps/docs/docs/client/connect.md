---
title: Build & Connect
slug: client/connect
---

import Tabs from '@theme/Tabs';
import TabItem from '@theme/TabItem';

:::info
This part of documentation is targeted to clients/wallets that want to enable nightly connect
as way of interaction with external applications.
:::

To get started, first we establish Connection with server `create()`. This enables use interactions with our sessions.

After that we can query session info with `getInfo()`, which requires 1 argument, sessionId (the one from the QR code).

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

const message: Connect = {
  publicKeys: [
    '9mtkm594sexac7G6jct3PZqyEVe3eUWMx6SUcEhYBRxr',
    '8MtpTNvQfr7iAWYLjJeyMw19vHw7bx7jrmoamkootfvA'
  ],
  sessionId: sessionId
}
await client.connect(message)
```

</TabItem>

<TabItem value="SUI" label="SUI">

```js
import { ClientSui } from '@nightlylabs/nightly-connect-sui'

const client: ClientSui = await ClientSui.create({
  url: RELAY_ENDPOINT // default: https://nc2.nightly.app
})
const info: GetInfoResponse = await client.getInfo(sessionId)

const message: Connect = {
  publicKeys: [
    '0x9353aa5322295a6542b69a05e873177b2594373a5ac58efa5055562630434a9e',
    '0x46f4dba3f180b8237119989d798c108f5d4c87b6aea02e6a093dd402a07083bd'
  ],
  sessionId: sessionId
}
await client.connect(message)
```

</TabItem>
<TabItem value="Substrate" label="Substrate">

```js
import { ClientPolkadot } from '@nightlylabs/nightly-connect-polkadot'

const client: ClientPolkadot = await ClientPolkadot.create({
  url: RELAY_ENDPOINT // default: https://nc2.nightly.app
})
const info: GetInfoResponse = await client.getInfo(sessionId)

const message: Connect = {
  publicKeys: ['5EnRWxJwqLuexBZtbJVTmfAzzc6Fwpw2Gv9AYs1gYHsgvzfH'],
  sessionId: sessionId,
  walletsMetadata: [
    {
      address: '5EnRWxJwqLuexBZtbJVTmfAzzc6Fwpw2Gv9AYs1gYHsgvzfH',
      name: 'Alice',
      type: 'ed25519'
    }
  ]
}
await client.connect(message)
```

</TabItem>
</Tabs>

### Disconnect

:::info
Both client and application can initiate disconnection.<br />
Though when it is the client who disconnects, the session will not be terminated.<br />
Only when application disconnects, the session will be closed.
:::
