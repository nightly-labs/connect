import { parseSerializedSignature, toSerializedSignature } from '@iota/iota-sdk/cryptography'
import { Ed25519Keypair } from '@iota/iota-sdk/keypairs/ed25519'
import { Transaction } from '@iota/iota-sdk/transactions'
import { fromB64, toB64 } from '@iota/iota-sdk/utils'
import { verifyPersonalMessageSignature, verifyTransactionSignature } from '@iota/iota-sdk/verify'
import { WalletAccount } from '@iota/wallet-standard'
import { Connect, ContentType } from '@nightlylabs/nightly-connect-base'
import { hexToBytes } from '@noble/hashes/utils'
import { fetch } from 'cross-fetch'
import { assert, beforeAll, beforeEach, describe, expect, test, vi } from 'vitest'
import { TEST_RELAY_ENDPOINT, smartDelay } from '../../../commonTestUtils'
import { AppIota } from './app'
import { ClientIota } from './client'
import { SignTransactionsIotaRequest } from './requestTypes'
import { TEST_APP_INITIALIZE } from './testUtils'
import { IOTA_NETWORK, signTransactionBlock } from './utils'

global.fetch = fetch

const ALICE_PRIVE_KEY = '4aa55c99d633c646b8dc423eed56e0fc39bdbca6ac6d8c53cc6e4decda27d970'
const alice_keypair = Ed25519Keypair.fromSecretKey(hexToBytes(ALICE_PRIVE_KEY))
const aliceWalletAccount: WalletAccount = {
  address: alice_keypair.getPublicKey().toIotaAddress(),
  publicKey: alice_keypair.getPublicKey().toRawBytes(),
  chains: ['iota:testnet'],
  features: ['iota:signAndExecuteTransactionBlock'],
  label: ''
}

// Wallet 4 from test seed
const RECEIVER_IOTA_ADDRESS = '0x62248df36a0f520bac63a54301079eb62b45c0c3374211a53fa0f57de5d8c415'

describe('SUI client tests', () => {
  let app: AppIota
  let client: ClientIota

  beforeAll(async () => {
    app = await AppIota.build(TEST_APP_INITIALIZE)
    expect(app).toBeDefined()
    assert(app.sessionId !== '')
    client = await ClientIota.create({ url: TEST_RELAY_ENDPOINT })
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
    assert(info.network === IOTA_NETWORK)
  })
  test('#connect()', async () => {
    const msg: Connect = {
      publicKeys: ['1', '2'],
      sessionId: app.sessionId
    }
    await client.connect(msg)
  })
  test('#on("signTransactions")', async () => {
    const tx = new Transaction()
    const coin = tx.splitCoins(tx.gas, [tx.pure.u64(100)])
    tx.transferObjects([coin], tx.pure.address(RECEIVER_IOTA_ADDRESS))
    tx.setSenderIfNotSet(RECEIVER_IOTA_ADDRESS)
    client.on('signTransactions', async (e) => {
      const tx = e.transactions[0].transaction
      const { signature, transactionBlockBytes } = await signTransactionBlock(
        Transaction.from(tx),
        alice_keypair
      )
      // resolve
      await client.resolveSignTransaction({
        responseId: e.requestId,
        signedTransactions: [
          {
            transactionBlockBytes: toB64(transactionBlockBytes),
            signature: signature
          }
        ]
      })
    })

    await smartDelay()

    const signedTx = await app.signTransactionBlock({
      transaction: tx,
      account: aliceWalletAccount,
      chain: 'sui:testnet'
    })

    try {
      // Will throw if invalid
      await verifyTransactionSignature(fromB64(signedTx.transactionBlockBytes), signedTx.signature)
    } catch (error) {
      assert(false, 'Transaction block is invalid')
    }
  })
  test('#on("signMessages")', async () => {
    const msgToSign = 'I love Nightly'
    client.on('signMessages', async (e) => {
      const msg = e.messages[0].message
      const msgTo64 = new TextEncoder().encode(msg)
      const { signature } = await alice_keypair.signPersonalMessage(msgTo64)
      await verifyPersonalMessageSignature(msgTo64, signature)
      const signedMessage = {
        messageBytes: msg,
        signature: toSerializedSignature({
          signature: fromB64(signature),
          signatureScheme: 'ED25519',
          publicKey: alice_keypair.getPublicKey()
        })
      }
      // resolve
      await client.resolveSignMessage({
        responseId: e.responseId,
        signature: signedMessage
      })
    })
    await smartDelay()
    const signedMessage = await app.signMessage({
      message: new TextEncoder().encode(msgToSign),
      account: aliceWalletAccount
    })
    try {
      // We need to deserialize the signature
      const parsedSignature = parseSerializedSignature(signedMessage.signature)
      // Will throw if invalid
      await verifyPersonalMessageSignature(
        new TextEncoder().encode(msgToSign),
        toB64(parsedSignature.signature!)
      )
    } catch (error) {
      console.log(error)
      assert(false, 'Message is invalid')
    }
  })
  test('#on("signAndExecuteSignTransaction")', async () => {
    client.removeListener('signTransactions')
    const tx = new Transaction()
    const coin = tx.splitCoins(tx.gas, [tx.pure.u64(100)])
    tx.transferObjects([coin], tx.pure.address(RECEIVER_IOTA_ADDRESS))
    tx.setSenderIfNotSet(RECEIVER_IOTA_ADDRESS)
    const exampleDigest = "I'm a digest"
    client.on('signTransactions', async (e) => {
      const metadata = e.transactions[0].metadata
        ? JSON.parse(e.transactions[0].metadata)
        : undefined
      assert.ok(metadata?.execute === true)
      // Send TX and resolve with digest
      await client.resolveSignTransaction({
        responseId: e.requestId,
        signedTransactions: [{ digest: exampleDigest, confirmedLocalExecution: true }]
      })
    })

    await smartDelay()
    const response = await app.signAndExecuteTransactionBlock({
      transaction: tx,
      account: aliceWalletAccount,
      chain: 'iota:testnet'
    })
    assert.equal(response.digest, exampleDigest)
  })
  test('#getPendingRequests()', async () => {
    client.removeListener('signTransactions')
    const tx = new Transaction()
    const coin = tx.splitCoins(tx.gas, [tx.pure.u64(100)])
    tx.transferObjects([coin], tx.pure.address(RECEIVER_IOTA_ADDRESS))
    tx.setSenderIfNotSet(RECEIVER_IOTA_ADDRESS)
    app.signTransactionBlock({
      transaction: tx,
      account: aliceWalletAccount,
      chain: 'iota:testnet'
    })
    app.signTransactionBlock({
      transaction: tx,
      account: aliceWalletAccount,
      chain: 'iota:testnet'
    })
    await smartDelay(500)
    const requests = await client.getPendingRequests()
    expect(requests.length).toBe(2)
    expect(requests[0].type).toBe(ContentType.SignTransactions)
    expect(requests[1].type).toBe(ContentType.SignTransactions)
    const payload1 = requests[0] as SignTransactionsIotaRequest
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
