import { assert, beforeAll, describe, expect, test, vi } from 'vitest'
import { BaseApp } from './app'
import { sleep, testAppBaseInitialize, testClientBaseInitialize } from './utils'
import { BaseClient, Connect } from './client'
import { TransactionToSign } from '@bindings/TransactionToSign'
import { SignedTransaction } from '@bindings/SignedTransaction'
import { MessageToSign } from '@bindings/MessageToSign'
import { SignedMessage } from '@bindings/SignedMessage'

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
  test('#getInfo()', async () => {
    const info = await client.getInfo(baseApp.sessionId)
    expect(info).toBeDefined()
    assert(info.additionalInfo === testAppBaseInitialize.additionalInfo)
    assert(info.appDescription === testAppBaseInitialize.appDescription)
    assert(info.appIcon === testAppBaseInitialize.appIcon)
    assert(info.appName === testAppBaseInitialize.appName)
    assert(info.network === testAppBaseInitialize.network)
    assert(info.version === testAppBaseInitialize.version)
  })
  test('#connect()', async () => {
    const msg: Connect = {
      publicKeys: ['1', '2'],
      sessionId: baseApp.sessionId
    }
    await client.connect(msg)
  })
  test('#on("signTransactions")', async () => {
    const msg: Connect = {
      publicKeys: ['1', '2'],
      sessionId: baseApp.sessionId
    }
    const randomSignTransaction: TransactionToSign[] = [
      { network: 'solana', transaction: '1', publicKeys: ['1'] },
      { network: 'solana', transaction: '13', publicKeys: ['1'] }
    ]
    const randomResolveSignTransaction: SignedTransaction[] = [
      { network: 'solana', transaction: 'signed-1', publicKeys: ['1'] },
      { network: 'solana', transaction: 'signed-13', publicKeys: ['1'] }
    ]
    client.on('signTransactions', async (e) => {
      assert(e.transactions.length === 2)
      // resolve
      await client.resolveSignTransactions({
        requestId: e.requestId,
        signedTransactions: randomResolveSignTransaction
      })
    })
    // sleep(100)
    await sleep(0)
    const signed = await baseApp.signTransactions(randomSignTransaction)
    assert(signed.signed_transactions.length === 2)
  })
  test('#on("signMessages")', async () => {
    const msg: Connect = {
      publicKeys: ['1', '2'],
      sessionId: baseApp.sessionId
    }
    const randomSignMessage: MessageToSign[] = [
      { message: '1', publicKey: '1' },
      { message: '13', publicKey: '1' }
    ]
    const randomResolveSignMessage: SignedMessage[] = [
      { signedMessage: 'signed-1', publicKey: '1' },
      { signedMessage: 'signed-13', publicKey: '1' }
    ]
    client.on('signMessages', async (e) => {
      assert(e.messages.length === 2)
      // resolve
      await client.resolveSignMessages({
        requestId: e.requestId,
        signedMessages: randomResolveSignMessage
      })
    })
    // sleep(100)
    await sleep(0)
    const signed = await baseApp.signMessages(randomSignMessage)
    assert(signed.signedMessages.length === 2)
  })
})
