import { getWallets, type Wallet } from '@wallet-standard/core'
import { isWalletAdapterCompatibleStandardWallet } from '@solana/wallet-adapter-base'
import { WalletMetadata } from '../../../bindings/WalletMetadata'
import { WalletType } from '../../../bindings/WalletType'
import { Deeplink } from '../../../bindings/Deeplink'
import { Images } from '../../../bindings/Images'

export interface IWalletListItem extends Partial<WalletMetadata> {
  slug: string
  name: string
  walletType: WalletType
  mobile: Deeplink | null
  desktop: Deeplink | null
  image: Images
  recent?: boolean
  detected?: boolean
  standardWallet?: Wallet
}

export const solanaWalletsFilter = (wallet: Wallet) =>
  isWalletAdapterCompatibleStandardWallet(wallet)

export const getSolanaWalletsList = (presetList: WalletMetadata[], recentWalletName?: string) => {
  const { get } = getWallets()
  const windowWallets = get()

  const walletsData: Record<string, IWalletListItem> = {}

  presetList.forEach((wallet) => {
    walletsData[wallet.name] = {
      ...wallet,
      recent: recentWalletName === wallet.name
    }
  })

  windowWallets.filter(solanaWalletsFilter).forEach((wallet) => {
    walletsData[wallet.name] = {
      ...(walletsData?.[wallet.name] ?? {
        name: wallet.name,
        icon: wallet.icon,
        link: '',
        deeplink: null,
        recent: recentWalletName === wallet.name,
        walletType: 'hybrid'
      }),
      detected: true,
      standardWallet: wallet
    }
  })

  return Object.values(walletsData)
}
