export enum QueryNetwork {
  SOLANA = 'SOLANA',
  SUI = 'SUI'
}

export interface WalletSelectorItem {
  name: string
  icon: string
  link: string
  detected?: boolean
  recent?: boolean
}
