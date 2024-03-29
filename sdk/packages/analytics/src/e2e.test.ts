import { assert, beforeEach, describe, test } from 'vitest'
import { smartDelay } from '../../../commonTestUtils'
import { NightlyAnalytics } from './app'
import { NightlyCloud } from '@nightlylabs/nightly-cloud'
import { setupTest } from './test_utils'
import { HttpGetAppEventsRequest } from '../../../bindings/HttpGetAppEventsEventRequest'

const TEST_ENDPOINT = 'http://127.0.0.1:6969/cloud/public/events'

describe('Base Client tests', () => {
  let cloudClient: NightlyCloud
  let teamId: string
  let appId: string
  let analytics: NightlyAnalytics

  beforeEach(async () => {
    cloudClient = new NightlyCloud({
      url: 'http://127.0.0.1:6969/cloud'
    })

    const response = await setupTest(cloudClient)

    teamId = response.teamId
    appId = response.appId

    console.log('appId', appId)

    analytics = new NightlyAnalytics({
      sessionId: '6a82dc5a-c013-4c17-b6ff-45fe86877b76',
      network: 'solana',
      endpoint: TEST_ENDPOINT,
      appId: appId
    })

    await smartDelay()
  })

  test('event/appConnected', async () => {
    // Send event
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
      newSession: false
    })

    // Get events
    const payload = {
      appId: appId
    } as HttpGetAppEventsRequest

    const events = await cloudClient.getAppEvents(payload)

    assert(events.events.length === 1)
    assert(events.events[0].eventType === 'AppConnect')
    assert(events.cursor === null)
  })
})
