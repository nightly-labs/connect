import { assert, beforeAll, beforeEach, describe, expect, test, vi } from 'vitest'
import { BaseApp } from './app'
import { getRandomId } from './utils'
import { testAppBaseInitialize } from './testUtils'
import { smartDelay, TEST_RELAY_ENDPOINT } from '../../../commonTestUtils'
import { Connect } from './client'
import { MessageToSign, TransactionToSign } from './content'
import { SignedMessage, SignedTransaction } from './responseContent'
import { HttpBaseClient } from './http-client'

// Edit an assertion and save to see HMR in action

describe('Http Base Client tests', () => {
  let baseApp: BaseApp
  let client: HttpBaseClient
  const clientId = getRandomId()
  beforeEach(async () => {
    baseApp.removeAllListeners()
  })
  beforeAll(async () => {
    baseApp = await BaseApp.build(testAppBaseInitialize)
    expect(baseApp).toBeDefined()
    assert(baseApp.sessionId !== '')

    client = new HttpBaseClient({ url: TEST_RELAY_ENDPOINT, clientId: clientId })
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
    const userConnectedFn = vi.fn()
    baseApp.on('userConnected', async (e) => {
      e
      userConnectedFn(e)
    })

    const msg: Connect = {
      publicKeys: ['1', '2'],
      sessionId: baseApp.sessionId
    }
    await client.connect(msg)
    await smartDelay()
    expect(userConnectedFn).toHaveBeenCalledOnce()
  })
  test('#resolveSignTransactions()', async () => {
    const randomSignTransaction: TransactionToSign[] = [{ transaction: '1' }, { transaction: '13' }]
    const randomResolveSignTransaction: SignedTransaction[] = [
      { transaction: 'signed-1' },
      { transaction: 'signed-13' }
    ]
    // send sign transactions
    const promiseSignedTxs = baseApp.signTransactions(randomSignTransaction)
    await smartDelay()
    // Query for sign transactions
    const pendingRequest = (await client.getPendingRequests({ sessionId: baseApp.sessionId }))[0]
    await client.resolveSignTransactions({
      requestId: pendingRequest.requestId,
      sessionId: baseApp.sessionId,
      signedTransactions: randomResolveSignTransaction
    })

    await smartDelay()
    const signedTxs = await promiseSignedTxs
    assert(signedTxs.length === 2)
  })
  test('#resolveSignMessages()', async () => {
    const randomSignMessages: MessageToSign[] = [{ message: '1' }, { message: '13' }]
    const randomResolveSignMessages: SignedMessage[] = [
      { message: 'signed-1' },
      { message: 'signed-13' }
    ]
    // send sign Messagess
    const promiseSigned = baseApp.signMessages(randomSignMessages)
    await smartDelay()
    // Query for sign Messagess
    const pendingRequest = (await client.getPendingRequests({ sessionId: baseApp.sessionId }))[0]
    await client.resolveSignMessages({
      requestId: pendingRequest.requestId,
      sessionId: baseApp.sessionId,
      signedMessages: randomResolveSignMessages
    })

    await smartDelay()
    const signed = await promiseSigned
    assert(signed.length === 2)
  })
  test('#reject()', async () => {
    try {
      const randomSignMessages: MessageToSign[] = [{ message: '1' }, { message: '13' }]

      // eslint-disable-next-line no-async-promise-executor
      const promiseSigned = new Promise<void>(async (resolve) => {
        expect(() => baseApp.signMessages(randomSignMessages)).rejects.toThrow('test-error')
        resolve()
      })
      await smartDelay()
      // Query for sign Messagess
      const pendingRequest = (await client.getPendingRequests({ sessionId: baseApp.sessionId }))[0]
      await client.reject({
        requestId: pendingRequest.requestId,
        sessionId: baseApp.sessionId,
        reason: 'test-error'
      })
      await smartDelay()
      await promiseSigned
    } catch (error) {
      console.log(error)
    }
  })
})
