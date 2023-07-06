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

// clearing last nightly connect session id

export const clearSessionIdForNetwork = (network: string) => {
  const storage = getLocalStorage()

  storage.removeItem(getSessionIdLocalStorageKey(network))
}

// usage of eager connect for recent standard wallet

export const setUseStandardEagerForNetwork = (network: string) => {
  const storage = getLocalStorage()

  storage.setItem('NIGHTLY_CONNECT_SELECTOR_USE_STANDARD_EAGER_' + network, 'true')
}

export const getUserStandardEagerForNetwork = (network: string) => {
  const storage = getLocalStorage()

  const item = storage.getItem('NIGHTLY_CONNECT_SELECTOR_USE_STANDARD_EAGER_' + network)

  return item
}

export const clearUseStandardEagerForNetwork = (network: string) => {
  const storage = getLocalStorage()

  storage.removeItem('NIGHTLY_CONNECT_SELECTOR_USE_STANDARD_EAGER_' + network)
}
