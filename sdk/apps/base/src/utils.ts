import { v4 as uuidv4 } from 'uuid'
import { getStorage, ILocalStorage } from 'isomorphic-localstorage'
import { WalletMetadata } from '../../../bindings/WalletMetadata'
import { fetch } from 'cross-fetch'
import { AppBaseInitialize, ClientBaseInitialize } from './initializeTypes'

export const getRandomId = () => uuidv4()
export function sleep(ms: number) {
  return new Promise((resolve) => setTimeout(resolve, ms))
}
export const RELAY_ENDPOINT = process.env.PRODUCTION
  ? 'https://nc2.nightly.app'
  : 'http://127.0.0.1:6969'
export const testAppBaseInitialize: AppBaseInitialize = {
  appMetadata: {
    additionalInfo: 'test-additional-info',
    description: 'test-app-description',
    icon: 'test-app-icon',
    name: 'test-app-name'
  },
  network: 'test-network',
  persistent: false,
  persistentSessionId: undefined,
  timeout: undefined,
  url: RELAY_ENDPOINT
}
export const testClientBaseInitialize: ClientBaseInitialize = {
  timeout: undefined,
  url: RELAY_ENDPOINT
}

export const smartDelay = async (ms?: number) => {
  if (process.env.PRODUCTION) {
    await sleep(ms || 100)
  } else {
    if (process.env.IS_CI) {
      await sleep(ms || 100)
    } else {
      await sleep(ms || 5)
    }
  }
}
export const getWalletsMetadata = async (url?: string): Promise<WalletMetadata[]> => {
  const endpoint = url ?? RELAY_ENDPOINT + '/get_wallets_metadata'
  const result = await (
    await fetch(endpoint, {
      method: 'GET',
      headers: {
        Accept: 'application/json',
        'Content-Type': 'application/json'
      }
    })
  ).json()
  return result
}

let _localStorage: ILocalStorage | null = null
export const getLocalStorage = () => {
  if (_localStorage === null) {
    _localStorage = getStorage('./.nightly-connect-session')
  }

  return _localStorage
}

export const getSessionIdLocalStorageKey = (network: string) =>
  'NIGHTLY_CONNECT_SESSION_ID_' + network
