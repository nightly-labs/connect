import { assert, beforeAll, beforeEach, describe, expect, test, vi } from 'vitest'
import { AppSui } from './app'
import { ClientSui } from './client'
import { SUI_NETWORK, TEST_APP_INITIALIZE, sleep } from './utils'
import { Connect } from 'base'
import { sha256 } from 'js-sha256'
import nacl from 'tweetnacl'
import {
  Connection,
  Ed25519Keypair,
  fromB64,
  IntentScope,
  JsonRpcProvider,
  messageWithIntent,
  toB64,
  toSerializedSignature,
  TransactionBlock,
  verifyMessage
} from '@mysten/sui.js'
import { blake2b } from '@noble/hashes/blake2b'
import { fetch } from 'cross-fetch'
global.fetch = fetch
// Edit an assertion and save to see HMR in action
const alice_keypair = Ed25519Keypair.generate()
describe('Base Client tests', () => {
  let app: AppSui
  let client: ClientSui
  beforeAll(async () => {
    app = await AppSui.build(TEST_APP_INITIALIZE)
    expect(app).toBeDefined()
    assert(app.sessionId !== '')
    client = await ClientSui.create({ wsUrl: 'ws://localhost:6969' })
  })
  beforeEach(async () => {
    await sleep(5)
  })
  test('#getInfo()', async () => {
    const info = await client.getInfo(app.sessionId)
    expect(info).toBeDefined()
    assert(info.appMetadata.additionalInfo === TEST_APP_INITIALIZE.appMetadata.additionalInfo)
    assert(info.appMetadata.description === TEST_APP_INITIALIZE.appMetadata.description)
    assert(info.appMetadata.icon === TEST_APP_INITIALIZE.appMetadata.icon)
    assert(info.appMetadata.name === TEST_APP_INITIALIZE.appMetadata.name)
    assert(info.network === SUI_NETWORK)
    // assert(info.version === testAppBaseInitialize.version)
  })
  test('#connect()', async () => {
    const msg: Connect = {
      publicKeys: ['1', '2'],
      sessionId: app.sessionId
    }
    await client.connect(msg)
  })
  test('#on("signTransactions")', async () => {
    const RECEIVER = Ed25519Keypair.generate()
    const RECEIVER_SUI_ADDRESS = RECEIVER.getPublicKey().toSuiAddress()
    const suiConnection = new JsonRpcProvider(
      new Connection({ fullnode: 'https://fullnode.testnet.sui.io/' })
    )
    const tx = new TransactionBlock()

    client.on('signTransactions', async (e) => {
      const tx = e.transactions[0]
      const coin = tx.splitCoins(tx.gas, [tx.pure(100)])
      tx.transferObjects([coin], tx.pure(RECEIVER_SUI_ADDRESS))

      tx.setSenderIfNotSet(RECEIVER_SUI_ADDRESS)
      const transactionBlockBytes = await tx.build({
        provider: suiConnection,
        onlyTransactionKind: true
      })
      const intentMessage = messageWithIntent(IntentScope.TransactionData, transactionBlockBytes)
      const digest = blake2b(intentMessage, { dkLen: 32 })
      const signature = alice_keypair.signData(digest)

      // resolve
      await client.resolveSignTransaction({
        responseId: e.requestId,
        signedTransactions: [
          {
            transactionBlockBytes: toB64(transactionBlockBytes),
            signature: toSerializedSignature({
              signature,
              signatureScheme: 'ED25519',
              pubKey: alice_keypair.getPublicKey()
            })
          }
        ]
      })
    })
    // // sleep(100)
    await sleep(0)
    const signedTx = await app.signTransaction(tx)
    const isValid = await verifyMessage(
      signedTx.transactionBlockBytes,
      signedTx.signature,
      IntentScope.TransactionData
    )
    expect(isValid).toBeTruthy()
  })
  test('#on("signMessages")', async () => {
    const msgToSign = 'Hello World'
    client.on('signMessages', async (e) => {
      const msg = e.messages[0].message
      const msgTo64 = toB64(new TextEncoder().encode(msg))
      const intentMessage = messageWithIntent(IntentScope.PersonalMessage, fromB64(msgTo64))
      const digest = blake2b(intentMessage, { dkLen: 32 })
      const signature = alice_keypair.signData(digest)
      const signedMessage = {
        messageBytes: msg,
        signature: toSerializedSignature({
          signature,
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
    await sleep(0)
    const signedMessage = await app.signMessage(msgToSign)
    const signData = new TextEncoder().encode(msgToSign)
    const isValid = await verifyMessage(
      signData,
      signedMessage.signature,
      IntentScope.PersonalMessage
    )
    expect(isValid).toBe(true)
  })
})
