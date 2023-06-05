import { assert, beforeAll, beforeEach, describe, expect, test, vi } from 'vitest'
import { BaseApp } from './app'
import { sleep, testAppBaseInitialize, testClientBaseInitialize } from './utils'
import { BaseClient, Connect } from './client'
import { MessageToSign, TransactionToSign } from './content'
import { SignedMessage, SignedTransaction } from './responseContent'

// Edit an assertion and save to see HMR in action

describe('Base Client tests', () => {
  let baseApp: BaseApp
  let client: BaseClient
  beforeAll(async () => {
    baseApp = await BaseApp.build(testAppBaseInitialize)
    expect(baseApp).toBeDefined()
    assert(baseApp.sessionId !== '')
    client = await BaseClient.build(testClientBaseInitialize)
  })
  beforeEach(async () => {
    await sleep(5)
    // Reset the events
    client.removeAllListeners()
  })
  test('#getInfo()', async () => {
    const info = await client.getInfo(baseApp.sessionId)
    expect(info).toBeDefined()
    assert(info.appMetadata.additionalInfo === testAppBaseInitialize.appMetadata.additionalInfo)
    assert(info.appMetadata.description === testAppBaseInitialize.appMetadata.description)
    assert(info.appMetadata.icon === testAppBaseInitialize.appMetadata.icon)
    assert(info.appMetadata.name === testAppBaseInitialize.appMetadata.name)
    assert(info.network === testAppBaseInitialize.network)
    // assert(info.version === testAppBaseInitialize.version)
  })
  test('#connect()', async () => {
    const msg: Connect = {
      publicKeys: ['1', '2'],
      sessionId: baseApp.sessionId
    }
    await client.connect(msg)
  })
  test('#on("signTransactions")', async () => {
    const randomSignTransaction: TransactionToSign[] = [{ transaction: '1' }, { transaction: '13' }]
    const randomResolveSignTransaction: SignedTransaction[] = [
      { transaction: 'signed-1' },
      { transaction: 'signed-13' }
    ]
    client.on('signTransactions', async (e) => {
      assert(e.transactions.length === 2)
      // resolve
      await client.resolveSignTransactions({
        sessionId: e.sessionId,
        requestId: e.responseId,
        signedTransactions: randomResolveSignTransaction
      })
    })
    // sleep(100)
    await sleep(0)
    const signedTxs = await baseApp.signTransactions(randomSignTransaction)
    assert(signedTxs.length === 2)
  })
  test('#on("signMessages")', async () => {
    const randomSignMessage: MessageToSign[] = [{ message: '1' }, { message: '13' }]
    const randomResolveSignMessage: SignedMessage[] = [
      { message: 'signed-1' },
      { message: 'signed-13' }
    ]
    client.on('signMessages', async (e) => {
      assert(e.messages.length === 2)
      // resolve
      await client.resolveSignMessages({
        sessionId: e.sessionId,
        requestId: e.responseId,
        signedMessages: randomResolveSignMessage
      })
    })
    // sleep(100)
    await sleep(0)
    const signed = await baseApp.signMessages(randomSignMessage)
    assert(signed.length === 2)
  })
  test('#reject', async () => {
    const randomSignTransaction: TransactionToSign[] = [{ transaction: '1' }, { transaction: '13' }]

    client.on('signTransactions', async (e) => {
      assert(e.transactions.length === 2)
      // resolve
      await client.reject({
        sessionId: e.sessionId,
        requestId: e.responseId,
        reason: 'rejected'
      })
    })
    // sleep(100)
    await sleep(0)
    try {
      await baseApp.signTransactions(randomSignTransaction)
      assert(false) // should not reach here
    } catch (error) {
      assert(true) // should  reach here
    }
  })
  test('#on("appDisconnected")', async () => {
    const disconnecFn = vi.fn()
    client.on('appDisconnected', async () => {
      disconnecFn()
    })
    baseApp.ws.terminate()
    baseApp.ws.close()
    await sleep(100)
    expect(disconnecFn).toHaveBeenCalledOnce()
  })
})
