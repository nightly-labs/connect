import { AppBaseInitialize } from '@nightlylabs/nightly-connect-base'
import { TEST_RELAY_ENDPOINT } from '../../../commonTestUtils'
import { APTOS_NETWORK } from './utils'

export const TEST_APP_INITIALIZE: AppBaseInitialize = {
  appMetadata: {
    additionalInfo: 'test-aptos-additional-info',
    description: 'test-aptos-app-description',
    icon: 'test-aptos-app-icon',
    name: 'test-aptos-app-name'
  },
  network: APTOS_NETWORK,
  persistent: false,
  persistentSessionId: undefined,
  timeout: undefined,
  url: TEST_RELAY_ENDPOINT
}
