import { AppSuiInitialize } from './app'

export const SUI_NETWORK = 'Sui'

export const TEST_APP_INITIALIZE: AppSuiInitialize = {
  appMetadata: {
    additionalInfo: 'test-sui-additional-info',
    description: 'test-sui-app-description',
    icon: 'test-sui-app-icon',
    name: 'test-sui-app-name'
  },
  persistent: false,
  persistentSessionId: undefined,
  timeout: undefined,
  wsUrl: 'ws://localhost:6969'
}
export function sleep(ms: number) {
  return new Promise((resolve) => setTimeout(resolve, ms))
}
