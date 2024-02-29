import { AppPolkadotInitialize } from './app'
import { TEST_RELAY_ENDPOINT } from '../../../commonTestUtils'

export const TEST_APP_INITIALIZE: AppPolkadotInitialize = {
  appMetadata: {
    additionalInfo: 'test-polkadot-additional-info',
    description: 'test-polkadot-app-description',
    icon: 'test-polkadot-app-icon',
    name: 'test-polkadot-app-name'
  },
  network: 'POLKADOT',
  persistent: false,
  persistentSessionId: undefined,
  timeout: undefined,
  url: TEST_RELAY_ENDPOINT
}
