import { AppBaseInitialize } from '@nightlylabs/nightly-connect-base'
import { Deeplink } from '@nightlylabs/nightly-connect-base/dist/types/bindings/Deeplink'
import { QueryNetwork } from '@nightlylabs/wallet-selector-modal'
import { Wallet } from '@wallet-standard/core'

export interface Adapter {
  connect: () => Promise<void>
}

export type AppInitData = Omit<AppBaseInitialize, 'network'>

export interface MetadataWallet {
  name: string
  icon: string
  deeplink: Deeplink | null
  link: string
}

export interface IWalletListItem extends MetadataWallet {
  recent?: boolean
  detected?: boolean
  standardWallet?: Wallet
}

export interface NetworkData {
  network: QueryNetwork
  name: string
  icon: string
}

export { QueryNetwork }
