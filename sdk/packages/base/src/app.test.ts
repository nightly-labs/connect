import { assert, beforeAll, beforeEach, describe, expect, test, vi } from 'vitest'
import { BaseApp } from './app'
import { testAppBaseInitialize, testClientBaseInitialize } from './testUtils'
import { smartDelay } from '../../../commonTestUtils'
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
  test('#on("userConnected")', async () => {
    const baseApp = await BaseApp.build({ ...testAppBaseInitialize, persistent: false })
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
  test('#getWalletsMetadata()', async () => {
    const walletsMetadata = await BaseApp.getWalletsMetadata()
    expect(walletsMetadata).toBeDefined()
    assert(walletsMetadata.length > 0)

    const filteredWalletsMetadata = await BaseApp.getWalletsMetadata(undefined, 'sui')
    assert(filteredWalletsMetadata.length > 0)
    assert(filteredWalletsMetadata.length < walletsMetadata.length)
  })
})
