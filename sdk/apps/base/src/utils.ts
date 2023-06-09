import { v4 as uuidv4 } from 'uuid'
import { AppBaseInitialize } from './app'
import { ClientBaseInitialize } from './client'
import { getStorage, ILocalStorage } from 'isomorphic-localstorage'

export const getRandomId = () => uuidv4()
export function sleep(ms: number) {
  return new Promise((resolve) => setTimeout(resolve, ms))
}

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
  url: 'http://127.0.0.1:6969'
}
export const testClientBaseInitialize: ClientBaseInitialize = {
  timeout: undefined,
  url: 'http://127.0.0.1:6969'
}

let _localStorage: ILocalStorage | null = null
export const getLocalStorage = () => {
  if (_localStorage === null) {
    _localStorage = getStorage('./.nightly-connect-session')
  }

  return _localStorage
}