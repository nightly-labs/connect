import { Connect, ContentType } from '@nightlylabs/nightly-connect-base'
import { assert, beforeAll, beforeEach, describe, expect, test, vi } from 'vitest'
import { AppSui } from './app'
import { ClientSui } from './client'
import { signTransactionBlock, SUI_NETWORK } from './utils'
import { TEST_APP_INITIALIZE } from './testUtils'
import { fromB64, toB64 } from '@mysten/sui.js/utils'
import { TransactionBlock } from '@mysten/sui.js/transactions'
import { Ed25519Keypair } from '@mysten/sui.js/keypairs/ed25519'
import { verifyPersonalMessage, verifyTransactionBlock } from '@mysten/sui.js/verify'
import { parseSerializedSignature, toSerializedSignature } from '@mysten/sui.js/cryptography'
import { fetch } from 'cross-fetch'
import { WalletAccount } from '@mysten/wallet-standard'
import { hexToBytes } from '@noble/hashes/utils'
import { SignTransactionsSuiRequest } from './requestTypes'
import { smartDelay, TEST_RELAY_ENDPOINT } from '../../../commonTestUtils'

global.fetch = fetch

const ALICE_PRIVE_KEY = '4aa55c99d633c646b8dc423eed56e0fc39bdbca6ac6d8c53cc6e4decda27d970'
const alice_keypair = Ed25519Keypair.fromSecretKey(hexToBytes(ALICE_PRIVE_KEY))
const aliceWalletAccount: WalletAccount = {
  address: alice_keypair.getPublicKey().toSuiAddress(),
  publicKey: alice_keypair.getPublicKey().toBytes(),
  chains: ['sui:testnet'],
  features: ['sui:signAndExecuteTransactionBlock'],
  label: ''
}

// Wallet 10 from test seed
const RECEIVER_SUI_ADDRESS = '0x19b78fbdf0f8fdb942abd67b8628ca80079aeb786cec0235d65af9b65019b59f'

describe('SUI client tests', () => {
  let app: AppSui
  let client: ClientSui

  beforeAll(async () => {
    app = await AppSui.build(TEST_APP_INITIALIZE)
    expect(app).toBeDefined()
    assert(app.sessionId !== '')
    client = await ClientSui.create({ url: TEST_RELAY_ENDPOINT })
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
    assert(info.network === SUI_NETWORK)
  })
  test('#connect()', async () => {
    const msg: Connect = {
      publicKeys: ['1', '2'],
      sessionId: app.sessionId
    }
    await client.connect(msg)
  })
  test('#on("signTransactions")', async () => {
    const tx = new TransactionBlock()
    const coin = tx.splitCoins(tx.gas, [tx.pure(100)])
    tx.transferObjects([coin], tx.pure(RECEIVER_SUI_ADDRESS))
    tx.setSenderIfNotSet(RECEIVER_SUI_ADDRESS)
    client.on('signTransactions', async (e) => {
      const tx = e.transactions[0].transaction
      const { signature, transactionBlockBytes } = await signTransactionBlock(
        TransactionBlock.from(tx),
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
      transactionBlock: tx,
      account: aliceWalletAccount,
      chain: 'sui:testnet'
    })

    try {
      // Will throw if invalid
      await verifyTransactionBlock(fromB64(signedTx.transactionBlockBytes), signedTx.signature)
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
      await verifyPersonalMessage(msgTo64, signature)
      const signedMessage = {
        messageBytes: msg,
        signature: toSerializedSignature({
          signature: fromB64(signature),
          signatureScheme: 'ED25519',
          pubKey: alice_keypair.getPublicKey()
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
      await verifyPersonalMessage(
        new TextEncoder().encode(msgToSign),
        toB64(parsedSignature.signature!)
      )
    } catch (error) {
      assert(false, 'Message is invalid')
    }
  })
  test('#on("signAndExecuteSignTransaction")', async () => {
    client.removeListener('signTransactions')
    const tx = new TransactionBlock()
    const coin = tx.splitCoins(tx.gas, [tx.pure(100)])
    tx.transferObjects([coin], tx.pure(RECEIVER_SUI_ADDRESS))
    tx.setSenderIfNotSet(RECEIVER_SUI_ADDRESS)
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
      transactionBlock: tx,
      account: aliceWalletAccount,
      chain: 'sui:testnet'
    })
    assert.equal(response.digest, exampleDigest)
  })
  test('#getPendingRequests()', async () => {
    client.removeListener('signTransactions')
    const tx = new TransactionBlock()
    const coin = tx.splitCoins(tx.gas, [tx.pure(100)])
    tx.transferObjects([coin], tx.pure(RECEIVER_SUI_ADDRESS))
    tx.setSenderIfNotSet(RECEIVER_SUI_ADDRESS)
    app.signTransactionBlock({
      transactionBlock: tx,
      account: aliceWalletAccount,
      chain: 'sui:testnet'
    })
    app.signTransactionBlock({
      transactionBlock: tx,
      account: aliceWalletAccount,
      chain: 'sui:testnet'
    })
    await smartDelay(500)
    const requests = await client.getPendingRequests()
    expect(requests.length).toBe(2)
    expect(requests[0].type).toBe(ContentType.SignTransactions)
    expect(requests[1].type).toBe(ContentType.SignTransactions)
    const payload1 = requests[0] as SignTransactionsSuiRequest
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
