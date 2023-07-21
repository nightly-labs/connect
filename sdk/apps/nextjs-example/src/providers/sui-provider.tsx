'use client'

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
