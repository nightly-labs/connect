---
title: Build & Connect
slug: sui/connect
---

import Tabs from '@theme/Tabs';
import TabItem from '@theme/TabItem';

The easiest way of trying nightly connect is by visiting the source code of the template web app for Solana, and following the implementation instructions.

:::info
We have ready to use templates that you can try here.

Preview: https://sui-web3-template.nightly.app/

Source code: https://github.com/nightly-labs/sui-web3-template
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

```js
import { WalletStandardAdapterProvider } from '@mysten/wallet-adapter-wallet-standard'
import { WalletKitProvider } from '@mysten/wallet-kit'
import { NightlyConnectSuiAdapter } from '@nightlylabs/wallet-selector-sui'
import dynamic from 'next/dynamic'
export const SuiProvider = ({ children }: any) => {
  return (
    <WalletKitProvider
      adapters={[
        new WalletStandardAdapterProvider(),
        NightlyConnectSuiAdapter.buildLazy(
          {
            appMetadata: {
              name: 'NCTestSui',
              description: 'Nightly Connect Test',
              icon: 'https://docs.nightly.app/img/logo.png',
              additionalInfo: 'Courtesy of Nightly Connect team'
            }
            //   persistent: false  -  Add this if you want to make the session non-persistent
          }
          // { initOnConnect: true, disableModal: true, disableEagerConnect: true }  -  You may specify the connection options object here
          // document.getElementById("modalAnchor")  -  You can pass an optional anchor element for the modal here
        )
      ]}>
      {children}
    </WalletKitProvider>
  )
}
export default dynamic(() => Promise.resolve(SuiProvider), {
  ssr: false
})
```

### Disconnect

:::info
Both client and application can initiate disconnection.
User can force session termination in case of abuse.
Only when application disconnects and session is not persistent, session is completely removed.
:::

</TabItem>
</Tabs>
