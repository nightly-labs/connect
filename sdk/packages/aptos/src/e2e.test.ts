import { Connect, ContentType } from '@nightlylabs/nightly-connect-base'
import { assert, beforeAll, beforeEach, describe, expect, test, vi } from 'vitest'
import { AppAptos } from './app'
import { ClientAptos } from './client'
import { TEST_APP_INITIALIZE } from './testUtils'
import { APTOS_NETWORK } from './utils'

import { Account, Aptos, Ed25519PrivateKey } from '@aptos-labs/ts-sdk'
import { AptosSignMessageInput, UserResponseStatus } from '@aptos-labs/wallet-standard'
import { TEST_RELAY_ENDPOINT, smartDelay } from '../../../commonTestUtils'
import { SignTransactionsAptosRequest } from './requestTypes'

const aptos = new Aptos() // default to devnet
const alice: Account = Account.fromPrivateKey({
  privateKey: new Ed25519PrivateKey(
    '200fb003a6e97c8ff2bd8691fa48b03e7ace251aae1b9a7365ac05d7db93bdc1' // PLEASE DO NOT USE THIS KEY
  )
})

describe('Aptos client tests', () => {
  let app: AppAptos
  let client: ClientAptos

  beforeAll(async () => {
    // create the account on chain
    await aptos.fundAccount({ accountAddress: alice.accountAddress, amount: 1_000_000 })
    app = await AppAptos.build(TEST_APP_INITIALIZE)
    expect(app).toBeDefined()
    assert(app.sessionId !== '')
    client = await ClientAptos.create({ url: TEST_RELAY_ENDPOINT })
    await smartDelay()
  })
  beforeEach(async () => {
    await smartDelay()
  })
  test('#getInfo()', async () => {
    const info = await client.getInfo(app.sessionId)
    expect(info).toBeDefined()
    assert(info.appMetadata.additionalInfo === TEST_APP_INITIALIZE.appMetadata.additionalInfo)
    assert(info.appMetadata.description === TEST_APP_INITIALIZE.appMetadata.description)
    assert(info.appMetadata.icon === TEST_APP_INITIALIZE.appMetadata.icon)
    assert(info.appMetadata.name === TEST_APP_INITIALIZE.appMetadata.name)
    assert(info.network === APTOS_NETWORK)
  })
  test('#connect()', async () => {
    const connectFn = vi.fn()
    app.on('userConnected', (a) => {
      connectFn(a.publicKeys[0])
    })
    const msg: Connect = {
      publicKeys: [alice.accountAddress.toString()],
      sessionId: app.sessionId
    }
    await client.connect(msg)
    await smartDelay()
    expect(connectFn).toHaveBeenCalledOnce()
    expect(connectFn).toHaveBeenCalledWith(alice.accountAddress.toString())
  })
  test('#on("signTransaction")', async () => {
    const bobAddress = '0xb0b'

    client.on('signTransaction', async (e) => {
      const tx = e.transactions[0]
      const senderAuthenticator = aptos.transaction.sign({
        signer: alice,
        transaction: tx
      })
      // resolve
      await client.resolveSignTransaction({
        requestId: e.requestId,
        signedTransactions: [senderAuthenticator]
      })
    })

    await smartDelay()

    const transaction = await aptos.transaction.build.simple({
      sender: alice.accountAddress,
      data: {
        function: '0x1::coin::transfer',
        typeArguments: ['0x1::aptos_coin::AptosCoin'],
        functionArguments: [bobAddress, 100]
      }
    })

    const signedTx = await app.signTransaction({
      rawTransaction: transaction.rawTransaction
    })
    // Verify the transaction was signed
    if (signedTx.status !== UserResponseStatus.APPROVED) {
      throw new Error('Transaction was not approved')
    }
    // Try to submit the transaction
    const pendingTransaction = await aptos.transaction.submit.simple({
      transaction,
      senderAuthenticator: signedTx.args
    })
    // Verify the transaction was submitted
    expect(pendingTransaction.hash).toBeDefined()
  })
  test('#on("signMessages")', async () => {
    client.on('signMessage', async (e) => {
      const payload = e.messages[0]
      const signature = alice.sign(new Buffer(payload.message).toString('hex'))
      // TODO fix this to match aptos schema
      await client.resolveSignMessage({
        requestId: e.requestId,
        signedMessages: [
          {
            message: payload.message,
            signature: signature,
            fullMessage: payload.message,
            nonce: payload.nonce,
            prefix: 'APTOS'
          }
        ]
      })
    })
    const msgToSign: AptosSignMessageInput = {
      message: 'I love Nightly',
      nonce: 'YOLO',
      address: true
    }
    await smartDelay()
    const _signedMessage = await app.signMessage(msgToSign)
  })
  test('#on("signAndSubmitTransaction")', async () => {
    const bobAddress = '0xb0b'

    client.on('signAndSubmitTransaction', async (e) => {
      const tx = e.transactions[0]
      const senderAuthenticator = aptos.transaction.sign({
        signer: alice,
        transaction: tx
      })
      // Try to submit the transaction
      const pendingTransaction = await aptos.transaction.submit.simple({
        transaction,
        senderAuthenticator: senderAuthenticator
      })
      // resolve
      await client.resolveSignAndSubmitTransaction({
        requestId: e.requestId,
        signedTransactions: [pendingTransaction]
      })
    })

    await smartDelay()

    const transaction = await aptos.transaction.build.simple({
      sender: alice.accountAddress,
      data: {
        function: '0x1::coin::transfer',
        typeArguments: ['0x1::aptos_coin::AptosCoin'],
        functionArguments: [bobAddress, 100]
      }
    })

    const submittedTx = await app.signAndSubmitTransaction({
      rawTransaction: transaction.rawTransaction
    })
    // Verify the transaction was signed
    if (submittedTx.status !== UserResponseStatus.APPROVED) {
      throw new Error('Transaction was not approved')
    }
    // Verify the transaction was submitted
    expect(submittedTx.args.hash).toBeDefined()
  })
  test('#getPendingRequests()', async () => {
    client.removeAllListeners()
    const bobAddress = '0xb0b'
    const transaction = await aptos.transaction.build.simple({
      sender: alice.accountAddress,
      data: {
        function: '0x1::coin::transfer',
        typeArguments: ['0x1::aptos_coin::AptosCoin'],
        functionArguments: [bobAddress, 100]
      }
    })
    app.signAndSubmitTransaction({
      rawTransaction: transaction.rawTransaction
    })
    app.signAndSubmitTransaction({
      rawTransaction: transaction.rawTransaction
    })
    await smartDelay(500)
    const requests = await client.getPendingRequests()
    expect(requests.length).toBe(2)
    expect(requests[0].type).toBe(ContentType.SignTransactions)
    expect(requests[1].type).toBe(ContentType.SignTransactions)
    const payload1 = requests[0] as SignTransactionsAptosRequest
    expect(payload1.transactions.length).toBe(1)
  })
  test('#on("appDisconnected")', async () => {
    const disconnecFn = vi.fn()
    client.on('appDisconnected', async () => {
      disconnecFn()
    })
    app.base.ws.terminate()
    app.base.ws.close()
    await smartDelay()
    expect(disconnecFn).toHaveBeenCalledOnce()
  })
})
