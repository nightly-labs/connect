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

Application builds a connection using `build()` or `buildLazy()` function that returns interface to communicated with remote user. App should define `AppMetadata` so wallets will be able to show it to user.

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

<Tabs>
<TabItem value="Solana" label="Solana">

:::info
We have ready to use templates that you can try here.

Preview: https://solana-web3-template.nightly.app/

Source code: https://github.com/nightly-labs/solana-web3-template
:::

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

:::info
We have ready to use templates that you can try here.

Preview: https://sui-web3-template.nightly.app/

Source code: https://github.com/nightly-labs/sui-web3-template
:::

You can implement nightly connect as full selector or use it with popular sui adapter https://github.com/MystenLabs/sui/tree/main/sdk/wallet-adapter

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
          },
          true
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

</TabItem>
<TabItem value="Substrate" label="Substrate">

:::info
We have ready to use templates that you can try here.

Preview: https://aleph-zero-web3-template.nightly.app/

Source code: https://github.com/nightly-labs/aleph-zero-web3-template
:::

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
  },
  true // should session be persisted
)
// Trigger connection
await adapter.connect()
// After connection adapter turns into remote signer
// Sign transaction
await payload.signAsync(publicKey, { signer: adapter.signer })
// Disconnect client if you want to end session
await adapter.disconnect()
```

</TabItem>
</Tabs>

### Disconnect

:::info
Both client and application can initiate disconnection.
User can force session termination in case of abuse.
Only when application disconnects and session is not persistent, session is completely removed.
:::
