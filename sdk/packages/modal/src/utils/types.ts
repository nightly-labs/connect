export enum QueryNetwork {
  SOLANA = 'SOLANA',
  SUI = 'SUI',
  POLKADOT = 'POLKADOT'
}

export interface WalletSelectorItem {
  name: string
  icon: string
  link: string
  detected?: boolean
  recent?: boolean
}

export enum WalletStatus {
  RECENT = 'Recent',
  DETECTED = 'Detected'
}

export enum SelectorView {
  DESKTOP_SELECT,
  MOBILE_INIT,
  MOBILE_QR,
  MOBILE_ALL,
  CONNECTING
}
