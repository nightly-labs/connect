---
title: Build & Connect
slug: solana/connect
---

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

You can implement nightly connect as full selector or use it with popular solana adapter https://github.com/solana-labs/wallet-adapter

```js
import { NightlyConnectAdapter } from '@nightlylabs/wallet-selector-solana'
import { WalletAdapterNetwork } from '@solana/wallet-adapter-base'
import { ConnectionProvider, WalletProvider } from '@solana/wallet-adapter-react'
import { WalletModalProvider, WalletMultiButton } from '@solana/wallet-adapter-react-ui'
import { UnsafeBurnerWalletAdapter } from '@solana/wallet-adapter-wallets'
import { clusterApiUrl } from '@solana/web3.js'
import type { FC, ReactNode } from 'react'
import React, { useMemo } from 'react'
export const App: FC = () => {
  return (
    <Context>
      <Content />
    </Context>
  )
}
const Context: FC<{ children: ReactNode }> = ({ children }) => {
  // The network can be set to 'devnet', 'testnet', or 'mainnet-beta'.
  const network = WalletAdapterNetwork.Devnet
  // You can also provide a custom RPC endpoint.
  const endpoint = useMemo(() => clusterApiUrl(network), [network])
  const wallets = useMemo(
    () => [
      /**
       * Wallets that implement either of these standards will be available automatically.
       *
       *   - Solana Mobile Stack Mobile Wallet Adapter Protocol
       *     (https://github.com/solana-mobile/mobile-wallet-adapter)
       *   - Solana Wallet Standard
       *     (https://github.com/solana-labs/wallet-standard)
       *
       * If you wish to support a wallet that supports neither of those standards,
       * instantiate its legacy wallet adapter here. Common legacy adapters can be found
       * in the npm package `@solana/wallet-adapter-wallets`.
       */
      new UnsafeBurnerWalletAdapter(),
      NightlyConnectAdapter.buildLazy(
        {
          appMetadata: {
            name: 'SolanaAdapter',
            description: 'Solana Adapter Test',
            icon: 'https://docs.nightly.app/img/logo.png',
            additionalInfo: 'Courtesy of Nightly Connect team'
          }
          //   persistent: false  -  Add this if you want to make the session non-persistent
        }
        // { initOnConnect: true, disableModal: true, disableEagerConnect: true }  -  You may specify the connection options object here
        // document.getElementById("modalAnchor")  -  You can pass an optional anchor element for the modal here
      )
    ],
    // eslint-disable-next-line react-hooks/exhaustive-deps
    [network]
  )
  return (
    <ConnectionProvider endpoint={endpoint}>
      <WalletProvider wallets={wallets} autoConnect>
        <WalletModalProvider>{children}</WalletModalProvider>
      </WalletProvider>
    </ConnectionProvider>
  )
}
const Content: FC = () => {
  return <WalletMultiButton />
}
```

### Disconnect

:::info
Both client and application can initiate disconnection.
User can force session termination in case of abuse.
Only when application disconnects and session is not persistent, session is completely removed.
:::
