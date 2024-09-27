import { type AppBaseInitialize } from '@nightlylabs/nightly-connect-base'
import { type Deeplink } from '@nightlylabs/nightly-connect-base/dist/types/bindings/Deeplink'
import { type Wallet } from '@wallet-standard/core'
import { type Images } from '../../../bindings/Images'
import { WalletMetadata } from '../../../bindings/WalletMetadata'
import { type WalletType } from '../../../bindings/WalletType'
export { type WalletMetadata } from '../../../bindings/WalletMetadata'

export interface Adapter {
  connect: () => Promise<void>
}
export type AppInitData = Omit<AppBaseInitialize, 'network'>
export type AppAptosInitData = AppInitData & { network?: string }

export interface MetadataWallet {
  slug: string
  name: string
  icon: string
  deeplink: Deeplink | null
  link: string
  walletType: WalletType
}

export interface IWalletListItem
  extends Pick<
    WalletMetadata,
    'name' | 'slug' | 'walletType' | 'mobile' | 'desktop' | 'image' | 'homepage'
  > {
  recent?: boolean
  detected?: boolean
  standardWallet?: Wallet
}

export interface ISelectedWallet {
  name: string
  homepage: string
  walletType: WalletType
  image: Images
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
