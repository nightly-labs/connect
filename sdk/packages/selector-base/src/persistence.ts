import { getSessionIdLocalStorageKey } from '@nightlylabs/nightly-connect-base'
import { ILocalStorage, getStorage } from 'isomorphic-localstorage'
import { AccountWalletType } from './types'

let _localStorage: ILocalStorage | null = null

type WalletInfo = {
  walletName: string
  walletType: AccountWalletType
}

export const getLocalStorage = () => {
  if (_localStorage === null) {
    _localStorage = getStorage('./nightly-connect-session')
  }

  return _localStorage
}

// recent wallet from standard

export const persistRecentWalletForNetwork = (network: string, walletInfo: WalletInfo) => {
  const storage = getLocalStorage()

  storage.setItem(
    'NIGHTLY_CONNECT_SELECTOR_RECENT_STANDARD_WALLET_' + network,
    JSON.stringify(walletInfo)
  )
}

export const getRecentWalletForNetwork = (network: string) => {
  const storage = getLocalStorage()

  const item = storage.getItem('NIGHTLY_CONNECT_SELECTOR_RECENT_STANDARD_WALLET_' + network)

  return item
}

export const clearRecentWalletForNetwork = (network: string) => {
  const storage = getLocalStorage()

  storage.removeItem('NIGHTLY_CONNECT_SELECTOR_RECENT_STANDARD_WALLET_' + network)
}

// clearing last nightly connect session id

export const clearSessionIdForNetwork = (network: string) => {
  const storage = getLocalStorage()

  storage.removeItem(getSessionIdLocalStorageKey(network))
}
