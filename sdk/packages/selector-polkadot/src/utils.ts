import { NetworkData } from '@nightlylabs/wallet-selector-base'
export type SupportedNetworks = 'Polkadot' | 'AlephZero' | 'Vara'
export const networkToData = (network: SupportedNetworks): NetworkData => {
  switch (network) {
    case 'Polkadot':
      return {
        name: network,
        icon: 'https://registry.nightly.app/networks/polkadot.png'
      }
    case 'AlephZero':
      return {
        name: network,
        icon: 'https://registry.nightly.app/networks/alephzero.png'
      }
    case 'Vara':
      return {
        name: network,
        icon: 'https://registry.nightly.app/networks/vara.png'
      }
  }
}
