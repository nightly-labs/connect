import { assert, describe, expect, test, vi } from 'vitest'
import { BaseApp } from './app'
import { BaseClient, Connect } from './client'
import { testAppBaseInitialize, testClientBaseInitialize } from './testUtils'
import { smartDelay } from '../../../commonTestUtils'
// Edit an assertion and save to see HMR in action
describe('Base App tests', () => {
  test('persistent session', async () => {
    const persistInitialize = { ...testAppBaseInitialize, persist: true }
    // Random string as app name to avoid conflicts
    const appName = Math.random().toString(36)
    persistInitialize.appMetadata.name = appName
    persistInitialize.persistent = true
    const baseApp = await BaseApp.build(persistInitialize)
    expect(baseApp).toBeDefined()
    assert(baseApp.sessionId !== '')
    const sessionId = baseApp.sessionId // Save the session id
    // Connect user
    const client = await BaseClient.build(testClientBaseInitialize)
    const msg: Connect = {
      publicKeys: ['1', '2'],
      sessionId: baseApp.sessionId
    }
    await client.connect(msg)
    await smartDelay()
    // Disconnect
    await baseApp.ws.terminate() // close() does not emit close event
    const disconnecFn = vi.fn()
    baseApp.on('serverDisconnected', () => {
      disconnecFn()
    })
    await smartDelay()
    expect(disconnecFn).toHaveBeenCalledOnce()
    // Reconnect
    persistInitialize.persistentSessionId = sessionId // Set the session id
    const baseApp2 = await BaseApp.build(persistInitialize)
    baseApp2.hasBeenRestored
    assert(baseApp2.hasBeenRestored == true)
    // Check public keys
    assert(baseApp2.connectedPublicKeys[0] === '1')
    assert(baseApp2.connectedPublicKeys[1] === '2')

    expect(baseApp2).toBeDefined()
    assert(baseApp2.sessionId == sessionId)
  })
})
