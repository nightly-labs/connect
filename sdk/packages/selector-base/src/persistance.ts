import { ILocalStorage, getStorage } from 'isomorphic-localstorage'

let _localStorage: ILocalStorage | null = null

export const getLocalStorage = () => {
  if (_localStorage === null) {
    _localStorage = getStorage('./localstorage')
  }

  return _localStorage
}

export const persistRecentWalletForNetwork = (walletName: string, network: string) => {
  const storage = getLocalStorage()

  storage.setItem('NIGHTLY_CONNECT_SELECTOR_RECENT_WALLET_' + network, walletName)
}

export const getRecentWalletForNetwork = (network: string) => {
  const storage = getLocalStorage()

  return storage.getItem('NIGHTLY_CONNECT_SELECTOR_RECENT_WALLET_' + network)
}
