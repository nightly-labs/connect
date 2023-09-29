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
  install?: boolean
}

export enum SelectorView {
  DESKTOP_MAIN,
  MOBILE_MAIN,
  MOBILE_QR,
  MOBILE_ALL,
  CONNECTING
}
