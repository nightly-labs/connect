import { type AppBaseInitialize } from '@nightlylabs/nightly-connect-base'
import { type Deeplink } from '@nightlylabs/nightly-connect-base/dist/types/bindings/Deeplink'
import { type Wallet } from '@wallet-standard/core'

export interface Adapter {
  connect: () => Promise<void>
}
// TODO we have two types of QueryNetwork
export enum QueryNetwork {
  SOLANA = 'SOLANA',
  SUI = 'SUI',
  POLKADOT = 'POLKADOT'
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

export enum ConnectionType {
  Nightly = 'Nightly',
  WalletStandard = 'WalletStandard'
}
