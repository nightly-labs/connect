import { type AppBaseInitialize } from '@nightlylabs/nightly-connect-base'
import { type Deeplink } from '@nightlylabs/nightly-connect-base/dist/types/bindings/Deeplink'
import { type Wallet } from '@wallet-standard/core'
import { type WalletType } from '../../../bindings/WalletType'

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

interface Images {
  default: string
  sm: string
  md: string
  lg: string
}

type Network = string

type Platform =
  | 'browser'
  | 'ios'
  | 'android'
  | 'macos'
  | 'windows'
  | 'linux'
  | 'chrome'
  | 'firefox'
  | 'opera'
  | 'edge'
  | 'brave'
  | 'safari'
  | 'other'

type Version = string

export interface WalletMetadata {
  slug: string
  name: string
  description?: string
  homepage?: string
  chains?: Array<Network>
  version?: Version
  walletType: WalletType
  mobile: Deeplink | null
  desktop: Deeplink | null
  image: Images
  app?: Record<Platform, string>
  injectPath?: Record<Network, string>
  lastUpdatedTimestamp?: bigint
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
