import { getSessionIdLocalStorageKey } from '@nightlylabs/nightly-connect-base'
import { ILocalStorage, getStorage } from 'isomorphic-localstorage'

let _localStorage: ILocalStorage | null = null

export const getLocalStorage = () => {
  if (_localStorage === null) {
    _localStorage = getStorage('./nightly-connect-session')
  }

  return _localStorage
}

export interface IBaseSessionData {
  publicKey: string
  walletName: string
}

export const persistSessionDataForNetwork = <T extends IBaseSessionData>(
  sessionData: T,
  network: string
) => {
  const storage = getLocalStorage()

  storage.setItem('NIGHTLY_CONNECT_SELECTOR_SESSION_DATA_' + network, JSON.stringify(sessionData))
}

export const getSessionDataForNetwork = <T extends IBaseSessionData>(network: string) => {
  const storage = getLocalStorage()

  const item = storage.getItem('NIGHTLY_CONNECT_SELECTOR_SESSION_DATA_' + network)

  return item === null ? null : (JSON.parse(item) as T)
}

export const clearSessionDataForNetwork = (network: string) => {
  localStorage.removeItem('NIGHTLY_CONNECT_SELECTOR_SESSION_DATA_' + network)
}

export const clearSessionIdForNetwork = (network: string) => {
  const storage = getLocalStorage()

  storage.removeItem(getSessionIdLocalStorageKey(network))
}
