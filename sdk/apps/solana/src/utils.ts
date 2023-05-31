import { AppSolanaInitialize } from './app'

export const SOLANA_NETWORK = 'Solana'

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
  wsUrl: 'ws://localhost:6969'
}
export function sleep(ms: number) {
  return new Promise((resolve) => setTimeout(resolve, ms))
}
