import { AppAptosInitialize } from './utils'
import { TEST_RELAY_ENDPOINT } from '../../../commonTestUtils'

export const TEST_APP_INITIALIZE: AppAptosInitialize = {
  appMetadata: {
    additionalInfo: 'test-aptos-additional-info',
    description: 'test-aptos-app-description',
    icon: 'test-aptos-app-icon',
    name: 'test-aptos-app-name'
  },
  persistent: false,
  persistentSessionId: undefined,
  timeout: undefined,
  url: TEST_RELAY_ENDPOINT
}
