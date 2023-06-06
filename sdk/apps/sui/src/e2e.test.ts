import { Connect } from 'base'
import { assert, beforeAll, beforeEach, describe, expect, test } from 'vitest'
import { AppSui } from './app'
import { ClientSui } from './client'
import { sleep, SUI_NETWORK, TEST_APP_INITIALIZE } from './utils'

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
import { WalletAccount } from '@mysten/wallet-standard'
import { hexToBytes } from '@noble/hashes/utils'
global.fetch = fetch
// Edit an assertion and save to see HMR in action
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
const suiConnection = new JsonRpcProvider(
  new Connection({ fullnode: 'https://fullnode.testnet.sui.io/' })
)
describe('Base Client tests', () => {
  let app: AppSui
  let client: ClientSui

  const signTransactionBlock = async (tx: TransactionBlock) => {
    const transactionBlockBytes = await tx.build({
      provider: suiConnection,
      onlyTransactionKind: true
    })
    const intentMessage = messageWithIntent(IntentScope.TransactionData, transactionBlockBytes)
    const digest = blake2b(intentMessage, { dkLen: 32 })
    const signatureArray = alice_keypair.signData(digest)
    const signature = toSerializedSignature({
      signature: signatureArray,
      signatureScheme: 'ED25519',
      pubKey: alice_keypair.getPublicKey()
    })
    return { transactionBlockBytes, signature }
  }

  beforeAll(async () => {
    app = await AppSui.build(TEST_APP_INITIALIZE)
    expect(app).toBeDefined()
    assert(app.sessionId !== '')
    client = await ClientSui.create({ url: 'ws://localhost:6969' })
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
    const tx = new TransactionBlock()
    const coin = tx.splitCoins(tx.gas, [tx.pure(100)])
    tx.transferObjects([coin], tx.pure(RECEIVER_SUI_ADDRESS))
    tx.setSenderIfNotSet(RECEIVER_SUI_ADDRESS)
    client.on('signTransactions', async (e) => {
      const tx = e.transactions[0]
      const { signature, transactionBlockBytes } = await signTransactionBlock(tx)
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
    // // sleep(100)
    await sleep(0)
    const signedTx = await app.signTransactionBlock({
      transactionBlock: tx,
      account: aliceWalletAccount,
      chain: 'sui:testnet'
    })

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

    const signedMessage = await app.signMessage({
      message: new TextEncoder().encode(msgToSign),
      account: aliceWalletAccount
    })
    const signData = new TextEncoder().encode(msgToSign)
    const isValid = await verifyMessage(
      signData,
      signedMessage.signature,
      IntentScope.PersonalMessage
    )
    expect(isValid).toBe(true)
  })
  // test('#on("signAndExecuteSignTransaction")', async () => {
  //   const tx = new TransactionBlock()
  //   const coin = tx.splitCoins(tx.gas, [tx.pure(100)])
  //   tx.transferObjects([coin], tx.pure(RECEIVER_SUI_ADDRESS))
  //   tx.setSenderIfNotSet(RECEIVER_SUI_ADDRESS)
  //   console.log('sign and execute')
  //   client.on('signTransactions', async (e) => {
  //     const tx = e.transactions[0]
  //     console.log('przed')
  //     const { signature, transactionBlockBytes } = await signTransactionBlock(tx)
  //     console.log('po sign')
  //     const response = await suiConnection.executeTransactionBlock({
  //       transactionBlock: transactionBlockBytes,
  //       signature
  //     })
  //     console.log(response)
  //     // resolve
  //     await client.resolveSignTransaction({
  //       responseId: e.requestId,
  //       signedTransactions: [response]
  //     })
  //   })

  //   const signedTx = await app.signAndExecuteTransactionBlock({
  //     transactionBlock: tx,
  //     account: aliceWalletAccount,
  //     chain: 'sui:testnet'
  //   })
  //   console.log(signedTx)
  // })
})
