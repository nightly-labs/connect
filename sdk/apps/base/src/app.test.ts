import { assert, beforeAll, beforeEach, describe, expect, test, vi } from 'vitest'
import { BaseApp } from './app'
import { smartDelay, testAppBaseInitialize, testClientBaseInitialize } from './utils'
import { BaseClient, Connect } from './client'
// Edit an assertion and save to see HMR in action
describe('Base App tests', () => {
  let baseApp: BaseApp
  beforeAll(async () => {
    baseApp = await BaseApp.build(testAppBaseInitialize)
    expect(baseApp).toBeDefined()
  })
  beforeEach(async () => {
    // Reset the events
    baseApp.removeAllListeners()
  })
  test('persistent session', async () => {
    const persistInitialize = testAppBaseInitialize
    // Random string as app name to avoid conflicts
    const appName = Math.random().toString(36)
    persistInitialize.appMetadata.name = appName
    persistInitialize.persistent = true
    const baseApp = await BaseApp.build(persistInitialize)
    expect(baseApp).toBeDefined()
    assert(baseApp.sessionId !== '')
    const sessionId = baseApp.sessionId // Save the session id
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
    expect(baseApp2).toBeDefined()
    assert(baseApp2.sessionId == sessionId)
  })
  test('#on("userConnected")', async () => {
    const baseApp = await BaseApp.build(testAppBaseInitialize)
    expect(baseApp).toBeDefined()
    assert(baseApp.sessionId !== '')
    const userConnectedFn = vi.fn()
    baseApp.on('userConnected', async (e) => {
      e
      userConnectedFn(e)
    })
    await smartDelay()

    // Create client
    const client = await BaseClient.build(testClientBaseInitialize)
    const msg: Connect = {
      publicKeys: ['1', '2'],
      sessionId: baseApp.sessionId
    }
    await client.connect(msg)
    await smartDelay()
    // We should get public keys
    expect(userConnectedFn.mock.lastCall[0].publicKeys).toStrictEqual(msg.publicKeys)
    expect(userConnectedFn).toHaveBeenCalledOnce()
  })
})
