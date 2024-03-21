import { describe, test } from 'vitest'
import { NightlyCloud } from './app'

describe('Base Client tests', () => {
  const analytics: NightlyCloud = new NightlyCloud({
    endpoint: 'https://analytics.nightly.app'
  })

  test('event/appConnected', async () => {
    await analytics.registerWithPassword({ email: 'test@test.com', password: 'test' })
  })
})
