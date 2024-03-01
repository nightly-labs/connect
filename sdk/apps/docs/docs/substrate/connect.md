---
title: Build & Connect
slug: substrate/connect
---

import Tabs from '@theme/Tabs';
import TabItem from '@theme/TabItem';

To try Nightly Connect easily, just go to the Aleph Zero Web3 template app's source code and follow the instructions there. It's the quickest way to get started and explore its features.

:::info
We have ready to use templates that you can try here.

Preview: https://aleph-zero-web3-template.nightly.app/

Source code: https://github.com/nightly-labs/aleph-zero-web3-template
:::

If you wish to enable nightly connect as a way of interacting with external applications, or to implement it as a wallet interface, use one of the ways below.

<Tabs>
<TabItem value="Client" label="Client">

:::info
This part of documentation is targeted to clients/ wallets that want to enable nightly connect
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

### Disconnect

:::info
Both client and application can initiate disconnection.<br />
Though when it is the client who disconnects, the session will not be terminated.<br />
Only when application disconnects, the session will be closed.
:::

</TabItem>

<TabItem value="Application" label="Application">

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

Application builds a connection using `build()` or `buildLazy()` function that returns interface to communicated with remote user. It is important to note, that the `buildLazy()` function allows for the modal to appear even when the sessionId is still undefined. App should define `AppMetadata` so wallets will be able to show it to user.

To start sending request like `signTransaction` user first need to connect to session.
Once user establishes connection, application will get public key and the connection will be confirmed.

API of application client is fit to match currently existing standards of corresponding blockchains

```js
interface AppMetadata {
  name: string;
  url?: string;
  description?: string;
  icon?: string; // Url of app image
  additionalInfo?: string;
}
```

You may also want to specify some additional connection options. This can be achieved by creating an object that implements the below interface, and using it inside the `build()` or `buildLazy()` function. Note, that the `disableModal` property can be used for implementing a custom [External modal](../../customization/customization/external_modal).

```js
interface ConnectionOptions {
  disableModal?: boolean // default: false
    //   Used for disabling modal in case you want to use your own
  initOnConnect?: boolean // default: false
    //   Ensures that the app is only build upon running the connect function
  disableEagerConnect?: boolean // default: false
    //   Do not connect eagerly, even if the previous session is saved
}
```

You can find example usage of this addapter here: https://github.com/nightly-labs/connect/blob/main/sdk/apps/modal-example/src/routes/aleph.tsx

```js
import { NightlyConnectAdapter } from '@nightlylabs/wallet-selector-polkadot'
const adapter = await NightlyConnectAdapter.build(
  {
    appMetadata: {
      name: 'NC TEST AlephZero',
      description: 'Nightly Connect Test',
      icon: 'https://docs.nightly.app/img/logo.png',
      additionalInfo: 'Courtesy of Nightly Connect team'
    },
    network: 'AlephZero'
    //   persistent: false  -  Add this if you want to make the session non-persistent
  }
  // { initOnConnect: true, disableModal: true, disableEagerConnect: true }  -  You may specify the connection options object here
  // document.getElementById("modalAnchor")  -  You can pass an optional anchor element for the modal here
)
// Trigger connection
await adapter.connect()
// After connection adapter turns into remote signer
// Sign transaction
await payload.signAsync(publicKey, { signer: adapter.signer })
// Disconnect client if you want to end session
await adapter.disconnect()
```

### Disconnect

:::info
Both client and application can initiate disconnection.
User can force session termination in case of abuse.
Only when application disconnects and session is not persistent, session is completely removed.
:::

</TabItem>
</Tabs>
