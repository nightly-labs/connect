---
title: External modal
slug: customization/external_modal
---

You may want to use your own design and/or change some logic for the modal. In that case passing all of the overrides to the `build()` or `buildLazy()` function can prove ineffective. Another option therfore is to omit the default modal altogether, by specyfing the `disableModal` option as true insid ethe `connectionOptions`.

You can then use the modal connect function, instead of using the default one from the adapter.

:::info
Example below is written for [Solana](../../solana/solana/start), but you can use it for [Sui](../../sui/sui/start) and [Substrate](../../substrate/substrate/start) as well.
:::

```js
import { NightlyConnectAdapter } from '@nightlylabs/wallet-selector-solana'
// You have to import the modal class separately
import { AppInitData, NightlyConnectSelectorModal } from '@nightlylabs/wallet-selector-base'

const appInitData: AppInitData = {
  appMetadata: {
    name: 'NCTestSolana',
    description: 'Nightly Connect Test',
    icon: 'https://docs.nightly.app/img/logo.png',
    additionalInfo: 'Courtesy of Nightly Connect team'
  }
}

const adapter = await NightlyConnectAdapter.build(
  appInitData,
  { disableModal: true } // ensures that the default modal will be disabled
)

// creates a new modal
const modal = new NightlyConnectSelectorModal(
  adapter.walletsList,
  appInitData.url ?? 'https://nc2.nightly.app',
  {
    name: SOLANA_NETWORK,
    icon: 'https://assets.coingecko.com/coins/images/4128/small/solana.png'
  },
  document.getElementById('modalAnchor')
)

// we can also use events to determine,
// what the current state of the app and react accordingly
adapter.on('connect', (pk) => {
  modal.closeModal()
})
```

The aforedescribed code sets up the adapter and the modal for later use.

To connect using custom modal, we can run:

```js
if (modal)
  modal.openModal(adapter?.sessionId ?? undefined, async (walletName) => {
    try {
      modal.setStandardWalletConnectProgress(true)
      await adapter?.connectToWallet(walletName)
    } catch (err) {
      modal.setStandardWalletConnectProgress(false)
      console.log('error')
      modal.closeModal()
    }
  })
```

:::info
You may include some additional functionality on top of the basic code. For more customization freedom, visit the source code for any adapter, e.g https://github.com/nightly-labs/connect/blob/main/sdk/packages/selector-solana/src/adapter.ts.
:::
