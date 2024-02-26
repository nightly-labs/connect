import { getSessionIdLocalStorageKey } from '@nightlylabs/nightly-connect-base'
import { ILocalStorage, getStorage } from 'isomorphic-localstorage'
import { ConnectionType } from './types'

let _localStorage: ILocalStorage | null = null

type WalletInfo = {
  walletName: string
  walletType: ConnectionType
}

export const getLocalStorage = () => {
  if (_localStorage === null) {
    _localStorage = getStorage('./nightly-connect-session')
  }

  return _localStorage
}

// recent wallet from standard
export const NIGHTLY_CONNECT_RECENT_WALLET = 'NIGHTLY_CONNECT_RECENT_WALLET_'
export const persistRecentWalletForNetwork = (network: string, walletInfo: WalletInfo) => {
  const storage = getLocalStorage()
  storage.setItem(NIGHTLY_CONNECT_RECENT_WALLET + network, JSON.stringify(walletInfo))
}

export const getRecentWalletForNetwork = (network: string) => {
  const storage = getLocalStorage()
  const item = storage.getItem(NIGHTLY_CONNECT_RECENT_WALLET + network)
  if (!item) return null
  try {
    return JSON.parse(item) as WalletInfo
  } catch (error) {
    console.warn('Error parsing recent wallet from local storage', error)
    return null
  }
}

export const clearRecentWalletForNetwork = (network: string) => {
  const storage = getLocalStorage()
  storage.removeItem(NIGHTLY_CONNECT_RECENT_WALLET + network)
}

// clearing last nightly connect session id

export const clearSessionIdForNetwork = (network: string) => {
  const storage = getLocalStorage()
  storage.removeItem(getSessionIdLocalStorageKey(network))
}
