import { assert, describe, test } from 'vitest'
import { TriggerDeeplink, createDeeplinkUrl, parseDeeplink } from './deeplinks'

describe('Deeplink tests', () => {
  test('Deeplink -> create and parse request', async () => {
    const deeplink: TriggerDeeplink = {
      path: 'nightly',
      deeplinkParams: {
        relay: 'https://nc2.nightly.app',
        requestId: '5e280437-22d9-4379-ab01-aea0f248c5f9',
        sessionId: '6a82dc5a-c013-4c17-b6ff-45fe0f45bddb'
      }
    }

    const url = createDeeplinkUrl(deeplink)
    assert.ok(
      url ===
        'nightly://nc?sessionId=6a82dc5a-c013-4c17-b6ff-45fe0f45bddb&relay=https://nc2.nightly.app&requestId=5e280437-22d9-4379-ab01-aea0f248c5f9'
    )
    const params = parseDeeplink(url)
    assert.ok(params.sessionId === deeplink.deeplinkParams.sessionId)
    assert.ok(params.requestId === deeplink.deeplinkParams.requestId)
    assert.ok(params.relay === deeplink.deeplinkParams.relay)
  })
  test('Deeplink -> create and parse connect', async () => {
    const deeplink: TriggerDeeplink = {
      path: 'nightly',
      deeplinkParams: {
        relay: 'https://nc2.nightly.app',
        sessionId: '6a82dc5a-c013-4c17-b6ff-45fe0f45bddb'
      }
    }
    const url = createDeeplinkUrl(deeplink)
    assert.ok(
      url ===
        'nightly://nc?sessionId=6a82dc5a-c013-4c17-b6ff-45fe0f45bddb&relay=https://nc2.nightly.app'
    )
    const params = parseDeeplink(url)
    assert.ok(params.sessionId === deeplink.deeplinkParams.sessionId)
    assert.ok(params.requestId === undefined)
    assert.ok(params.relay === deeplink.deeplinkParams.relay)
  })
  test('SmartLink -> create and parse request', async () => {
    const deeplink: TriggerDeeplink = {
      path: 'https://nightly.app',
      deeplinkParams: {
        relay: 'https://nc2.nightly.app',
        requestId: '5e280437-22d9-4379-ab01-aea0f248c5f9',
        sessionId: '6a82dc5a-c013-4c17-b6ff-45fe0f45bddb'
      }
    }

    const url = createDeeplinkUrl(deeplink)
    assert.ok(
      url ===
        'https://nightly.app/nc?sessionId=6a82dc5a-c013-4c17-b6ff-45fe0f45bddb&relay=https://nc2.nightly.app&requestId=5e280437-22d9-4379-ab01-aea0f248c5f9'
    )
    const params = parseDeeplink(url)
    assert.ok(params.sessionId === deeplink.deeplinkParams.sessionId)
    assert.ok(params.requestId === deeplink.deeplinkParams.requestId)
    assert.ok(params.relay === deeplink.deeplinkParams.relay)
  })
  test('SmartLink -> create and parse connect', async () => {
    const deeplink: TriggerDeeplink = {
      path: 'https://nightly.app',
      deeplinkParams: {
        relay: 'https://nc2.nightly.app',
        sessionId: '6a82dc5a-c013-4c17-b6ff-45fe0f45bddb'
      }
    }
    const url = createDeeplinkUrl(deeplink)
    assert.ok(
      url ===
        'https://nightly.app/nc?sessionId=6a82dc5a-c013-4c17-b6ff-45fe0f45bddb&relay=https://nc2.nightly.app'
    )
    const params = parseDeeplink(url)
    assert.ok(params.sessionId === deeplink.deeplinkParams.sessionId)
    assert.ok(params.requestId === undefined)
    assert.ok(params.relay === deeplink.deeplinkParams.relay)
  })
})
