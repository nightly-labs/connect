import { AppSuiInitialize } from './utils'
import { TEST_RELAY_ENDPOINT } from '../../../commonTestUtils'

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
  url: TEST_RELAY_ENDPOINT
}
