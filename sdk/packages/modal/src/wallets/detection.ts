import { getWallets } from '@wallet-standard/core'

export interface IWalletListItem {
    name: string
    icon: string
    recent?: boolean
    detected?: boolean
}

export const getWalletsList = (
    presetList: Omit<IWalletListItem, 'recent' | 'detected'>[],
    recentWalletName?: string
) => {
    const { get } = getWallets()
    const windowWallets = get()

    const walletsData: Record<string, IWalletListItem> = {}

    presetList.forEach(wallet => {
        walletsData[wallet.name] = wallet
    })

    windowWallets.forEach(wallet => {
        walletsData[wallet.name] = {
            ...(walletsData?.[wallet.name] ?? wallet),
            detected: true,
            recent: recentWalletName === wallet.name
        }
    })

    return Object.values(walletsData)
}