import { NetworkData } from '@nightlylabs/wallet-selector-base'
export type SupportedNetworks = 'Polkadot' | 'AlephZero'
export const networkToData = (network: SupportedNetworks): NetworkData => {
  switch (network) {
    case 'Polkadot':
      return {
        name: network,
        icon: 'https://registry.nightly.app/networks/polkadot.png' // TODO add polka icon
      }
    case 'AlephZero':
      return {
        name: network,
        icon: 'https://registry.nightly.app/networks/alephzero.png' // TODO add polka icon
      }
  }
}
