import { AppSolanaInitialize } from './utils'
import { TEST_RELAY_ENDPOINT } from '../../../commonTestUtils'

export const TEST_APP_INITIALIZE: AppSolanaInitialize = {
  appMetadata: {
    additionalInfo: 'test-solana-additional-info',
    description: 'test-solana-app-description',
    icon: 'test-solana-app-icon',
    name: 'test-solana-app-name'
  },
  persistent: false,
  persistentSessionId: undefined,
  timeout: undefined,
  url: TEST_RELAY_ENDPOINT
}
export function sleep(ms: number) {
  return new Promise((resolve) => setTimeout(resolve, ms))
}
