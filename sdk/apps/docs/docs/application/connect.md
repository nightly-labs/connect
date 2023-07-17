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
  network?: Network;
  url?: string; // Relay endpoint default nc2.nightly.app
  timeout?: number;
  persistentSessionId?: string; // Apps can reuse old connection to skip connect step.
  persistent?: boolean; // Makes session valid for 14 days
}
```

<Tabs>
<TabItem value="Solana" label="Solana">

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
          },
          url: 'https://nc2.nightly.app'
        },
        true
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
