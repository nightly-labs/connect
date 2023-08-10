import { type AppBaseInitialize } from '@nightlylabs/nightly-connect-base'
import { type Deeplink } from '@nightlylabs/nightly-connect-base/dist/types/bindings/Deeplink'
import { type Wallet } from '@wallet-standard/core'

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
  name: string
  icon: string
}

export enum ConnectionType {
  Nightly = 'Nightly',
  WalletStandard = 'WalletStandard'
}
