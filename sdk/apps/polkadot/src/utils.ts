import { RELAY_ENDPOINT } from '@nightlylabs/nightly-connect-base'
import { AppPolkadotInitialize } from './app'

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
  url: RELAY_ENDPOINT
}
export const sleep = (ms: number) => {
  return new Promise((resolve) => setTimeout(resolve, ms))
}
