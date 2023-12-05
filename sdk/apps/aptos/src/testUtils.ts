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
export function sleep(ms: number) {
  return new Promise((resolve) => setTimeout(resolve, ms))
}

export const NODE_URL = process.env.APTOS_NODE_URL || 'https://fullnode.devnet.aptoslabs.com'
export const FAUCET_URL = process.env.APTOS_FAUCET_URL || 'https://faucet.devnet.aptoslabs.com'

export const aptosCoinStore = '0x1::coin::CoinStore<0x1::aptos_coin::AptosCoin>'
