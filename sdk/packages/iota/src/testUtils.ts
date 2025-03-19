import { AppIotaInitialize } from './utils'
import { TEST_RELAY_ENDPOINT } from '../../../commonTestUtils'

export const TEST_APP_INITIALIZE: AppIotaInitialize = {
  appMetadata: {
    additionalInfo: 'test-iota-additional-info',
    description: 'test-iota-app-description',
    icon: 'test-iota-app-icon',
    name: 'test-iota-app-name'
  },
  persistent: false,
  persistentSessionId: undefined,
  timeout: undefined,
  url: TEST_RELAY_ENDPOINT
}
