import { beforeEach, describe, test } from 'vitest'
import { smartDelay } from '../../../commonTestUtils'
import { NightlyAnalytics } from './app'

describe('Base Client tests', () => {
  const analytics: NightlyAnalytics = new NightlyAnalytics({
    sessionId: '6a82dc5a-c013-4c17-b6ff-45fe86877b76',
    network: 'solana',
    endpoint: 'https://analytics.nightly.app'
  })
  beforeEach(async () => {
    await smartDelay()
  })
  test('event/appConnected', async () => {
    // TODO somehow check if analytics is sent
    await analytics.appConnected({
      sessionId: '6a82dc5a-c013-4c17-b6ff-45fe86877b76',
      deviceMetadata: {
        mobile: {
          system: 'Unknown',
          version: '15.0'
        }
      },
      language: 'en',
      timezone: 'Europe/London',
      network: 'solana',
      newSession: true
    })
  })
})
