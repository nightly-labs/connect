import { AppSolanaInitialize } from './app'

export const SOLANA_NETWORK = 'Solana'

export const TEST_APP_INITIALIZE: AppSolanaInitialize = {
  additionalInfo: 'test-solana-additional-info',
  appDescription: 'test-solana-app-description',
  appIcon: 'test-solana-app-icon',
  appName: 'test-solana-app-name',
  persistent: false,
  persistentSessionId: undefined,
  timeout: undefined,
  wsUrl: 'ws://localhost:6969'
}
export function sleep(ms: number) {
  return new Promise((resolve) => setTimeout(resolve, ms))
}
