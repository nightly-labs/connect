import { type AppBaseInitialize } from '@nightlylabs/nightly-connect-base'
import { type Deeplink } from '@nightlylabs/nightly-connect-base/dist/types/bindings/Deeplink'
import { type Wallet } from '@wallet-standard/core'
import { type WalletType } from '../../../bindings/WalletType'
import { type WalletMetadata as WalletMetadataOriginal } from '../../../bindings/WalletMetadata'
import { Images } from '../../../bindings/Images'

export interface Adapter {
  connect: () => Promise<void>
}
export type AppInitData = Omit<AppBaseInitialize, 'network'>

export interface MetadataWallet {
  slug: string
  name: string
  icon: string
  deeplink: Deeplink | null
  link: string
  walletType: WalletType
}

export interface WalletMetadata extends Partial<WalletMetadataOriginal> {
  slug: string
  name: string
  walletType: WalletType
  mobile: Deeplink | null
  desktop: Deeplink | null
  image: Images
}

export interface IWalletListItem extends WalletMetadata {
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

export interface ConnectionOptions {
  disableModal?: boolean // default: false
  initOnConnect?: boolean // default: false
  disableEagerConnect?: boolean // default: false
}
export const defaultConnectionOptions: ConnectionOptions = {
  disableModal: false,
  initOnConnect: false,
  disableEagerConnect: false
}
