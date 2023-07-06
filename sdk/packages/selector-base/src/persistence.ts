import { getSessionIdLocalStorageKey } from '@nightlylabs/nightly-connect-base'
import { ILocalStorage, getStorage } from 'isomorphic-localstorage'

let _localStorage: ILocalStorage | null = null

export const getLocalStorage = () => {
  if (_localStorage === null) {
    _localStorage = getStorage('./nightly-connect-session')
  }

  return _localStorage
}

export const persistRecentWalletForNetwork = (walletName: string, network: string) => {
  const storage = getLocalStorage()

  storage.setItem('NIGHTLY_CONNECT_SELECTOR_RECENT_WALLET_' + network, walletName)
}

export const getRecentWalletForNetwork = (network: string) => {
  const storage = getLocalStorage()

  const item = storage.getItem('NIGHTLY_CONNECT_SELECTOR_RECENT_WALLET_' + network)

  return item
}

export const clearSessionIdForNetwork = (network: string) => {
  const storage = getLocalStorage()

  storage.removeItem(getSessionIdLocalStorageKey(network))
}
