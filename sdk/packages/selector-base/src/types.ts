import { AppBaseInitialize } from '@nightlylabs/nightly-connect-base'
import { Deeplink } from '@nightlylabs/nightly-connect-base/dist/browser/cjs/types/bindings/Deeplink'

export enum NETWORK {
  SOLANA = 'SOLANA',
  SUI = 'SUI'
}

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

export interface NetworkData {
    network: NETWORK,
    name: string,
    icon: string
}
