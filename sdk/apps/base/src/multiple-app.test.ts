import { describe, expect, test, vi } from 'vitest'
import { BaseApp } from './app'
import { testAppBaseInitialize, testClientBaseInitialize } from './testUtils'
import { smartDelay } from '../../../commonTestUtils'
import { BaseClient, Connect } from './client'
import { MessageToSign } from './content'
import { SignedMessage } from './responseContent'
// Edit an assertion and save to see HMR in action
describe('multiple session tests', () => {
  let client: BaseClient
  let app1: BaseApp
  let app2: BaseApp
  let app3: BaseApp
  test('userConnected', async () => {
    app1 = await BaseApp.build({ ...testAppBaseInitialize, network: 'para', persistent: true })
    app2 = await BaseApp.build({
      ...testAppBaseInitialize,
      network: 'para',
      persistent: true,
      persistentSessionId: app1.sessionId
    })
    app3 = await BaseApp.build({
      ...testAppBaseInitialize,
      network: 'para',
      persistent: true,
      persistentSessionId: app1.sessionId
    })
    const app1UserConnected = vi.fn()
    const app2UserConnected = vi.fn()
    const app3UserConnected = vi.fn()

    app1.on('userConnected', async (e) => {
      e
      app1UserConnected(e)
    })
    app2.on('userConnected', async (e) => {
      e
      app2UserConnected(e)
    })
    app3.on('userConnected', async (e) => {
      e
      app3UserConnected(e)
    })
    // Create client
    client = await BaseClient.build(testClientBaseInitialize)
    const msg: Connect = {
      publicKeys: ['1', '2'],
      sessionId: app1.sessionId
    }
    await client.connect(msg)
    await smartDelay()
    // We should get public keys
    expect(app1UserConnected.mock.lastCall[0].publicKeys).toStrictEqual(msg.publicKeys)
    expect(app1UserConnected).toHaveBeenCalledOnce()

    expect(app2UserConnected.mock.lastCall[0].publicKeys).toStrictEqual(msg.publicKeys)
    expect(app2UserConnected).toHaveBeenCalledOnce()

    expect(app3UserConnected.mock.lastCall[0].publicKeys).toStrictEqual(msg.publicKeys)
    expect(app3UserConnected).toHaveBeenCalledOnce()
  })
  test('#on("signMessages")', async () => {
    const randomSignMessage: MessageToSign[] = [{ message: '1' }, { message: '13' }]
    const randomResolveSignMessage: SignedMessage[] = [
      { message: 'signed-1' },
      { message: 'signed-13' }
    ]
    client.on('signMessages', async (e) => {
      // resolve
      await client.resolveSignMessages({
        sessionId: e.sessionId,
        requestId: e.responseId,
        signedMessages: randomResolveSignMessage
      })
    })
    await smartDelay()
    const signed = await app1.signMessages(randomSignMessage)
  })
})
