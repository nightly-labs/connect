import { getSessionIdLocalStorageKey } from '@nightlylabs/nightly-connect-base'
import { ILocalStorage, getStorage } from 'isomorphic-localstorage'

let _localStorage: ILocalStorage | null = null

export const getLocalStorage = () => {
  if (_localStorage === null) {
    _localStorage = getStorage('./nightly-connect-session')
  }

  return _localStorage
}

// recent wallet from standard

export const persistRecentStandardWalletForNetwork = (walletName: string, network: string) => {
  const storage = getLocalStorage()

  storage.setItem('NIGHTLY_CONNECT_SELECTOR_RECENT_STANDARD_WALLET_' + network, walletName)
}

export const getRecentStandardWalletForNetwork = (network: string) => {
  const storage = getLocalStorage()

  const item = storage.getItem('NIGHTLY_CONNECT_SELECTOR_RECENT_STANDARD_WALLET_' + network)

  return item
}

export const clearRecentStandardWalletForNetwork = (network: string) => {
  const storage = getLocalStorage()

  storage.removeItem('NIGHTLY_CONNECT_SELECTOR_RECENT_STANDARD_WALLET_' + network)
}

// clearing last nightly connect session id

export const clearSessionIdForNetwork = (network: string) => {
  const storage = getLocalStorage()

  storage.removeItem(getSessionIdLocalStorageKey(network))
}

// info if any wallet from standard is connected

export const persistStandardConnectForNetwork = (network: string) => {
  const storage = getLocalStorage()

  storage.setItem('NIGHTLY_CONNECT_SELECTOR_IS_DESKTOP_CONNECTED_' + network, 'true')
}

export const isStandardConnectedForNetwork = (network: string) => {
  const storage = getLocalStorage()

  const item = storage.getItem('NIGHTLY_CONNECT_SELECTOR_IS_DESKTOP_CONNECTED_' + network)

  return item !== null
}

export const persistStandardDisconnectForNetwork = (network: string) => {
  const storage = getLocalStorage()

  storage.removeItem('NIGHTLY_CONNECT_SELECTOR_IS_DESKTOP_CONNECTED_' + network)
}
