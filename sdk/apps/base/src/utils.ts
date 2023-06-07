import { v4 as uuidv4 } from 'uuid'
import { AppBaseInitialize } from './app'
import { ClientBaseInitialize } from './client'
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
