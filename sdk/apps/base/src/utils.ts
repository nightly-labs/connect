import { v4 as uuidv4 } from 'uuid'
import { AppBaseInitialize } from './app'
import { ClientBaseInitialize } from './client'
export const getRandomId = () => uuidv4()
export function sleep(ms: number) {
  return new Promise((resolve) => setTimeout(resolve, ms))
}

export const testAppBaseInitialize: AppBaseInitialize = {
  additionalInfo: 'test-additional-info',
  appDescription: 'test-app-description',
  appIcon: 'test-app-icon',
  appName: 'test-app-name',
  network: 'test-network',
  persistent: false,
  persistentSessionId: undefined,
  version: 'test-version',
  timeout: undefined,
  wsUrl: 'ws://localhost:6969'
}
export const testClientBaseInitialize: ClientBaseInitialize = {
  persistent: false,
  version: 'test-version',
  timeout: undefined,
  wsUrl: 'ws://localhost:6969'
}
