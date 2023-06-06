import { assert, beforeAll, beforeEach, describe, expect, test, vi } from 'vitest'
import { AppSui } from './app'
import { SUI_NETWORK, TEST_APP_INITIALIZE, sleep } from './utils'
import { Connect, getRandomId } from 'base'
import { HttpClientSui } from './http-client'

import {
  Connection,
  Ed25519Keypair,
  IntentScope,
  JsonRpcProvider,
  messageWithIntent,
  toB64,
  toSerializedSignature,
  TransactionBlock,
  verifyMessage
} from '@mysten/sui.js'
import { blake2b } from '@noble/hashes/blake2b'
import { hexToBytes } from '@noble/hashes/utils'
import { WalletAccount } from '@mysten/wallet-standard'
import { ContentType } from 'base/src/content'
// Edit an assertion and save to see HMR in action
const ALICE_PRIVE_KEY = '4aa55c99d633c646b8dc423eed56e0fc39bdbca6ac6d8c53cc6e4decda27d970'
const alice_keypair = Ed25519Keypair.fromSecretKey(hexToBytes(ALICE_PRIVE_KEY))

const RECEIVER_SUI_ADDRESS = '0x19b78fbdf0f8fdb942abd67b8628ca80079aeb786cec0235d65af9b65019b59f'
const suiConnection = new JsonRpcProvider(
  new Connection({ fullnode: 'https://fullnode.testnet.sui.io/' })
)
const aliceWalletAccount: WalletAccount = {
  address: alice_keypair.getPublicKey().toSuiAddress(),
  publicKey: alice_keypair.getPublicKey().toBytes(),
  chains: ['sui:testnet'],
  features: ['sui:signAndExecuteTransactionBlock'],
  label: ''
}
describe('Base Client tests', () => {
  let app: AppSui
  let client: HttpClientSui
  const clientId = getRandomId()

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
    client = new HttpClientSui({ url: 'http://localhost:6969', clientId })
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
  test('#resolveSignTransaction()', async () => {
    const tx = new TransactionBlock()
    const coin = tx.splitCoins(tx.gas, [tx.pure(100)])
    tx.transferObjects([coin], tx.pure(RECEIVER_SUI_ADDRESS))
    tx.setSenderIfNotSet(RECEIVER_SUI_ADDRESS)

    const promiseSignTransaction = app.signTransactionBlock({
      transactionBlock: tx,
      account: aliceWalletAccount,
      chain: 'sui:testnet'
    })
    await sleep(100)
    // Query for request
    const pendingRequest = (await client.getPendingRequests({ sessionId: app.sessionId }))[0]
    if (pendingRequest.content.type !== ContentType.SignTransactions) {
      throw new Error('Wrong content type')
    }
    const pendingTx = pendingRequest.content.transactions[0].transaction
    const { signature, transactionBlockBytes } = await signTransactionBlock(
      TransactionBlock.from(pendingTx)
    )

    await client.resolveSignTransaction({
      requestId: pendingRequest.requestId,
      sessionId: app.sessionId,
      signedTransactions: [
        {
          transactionBlockBytes: toB64(transactionBlockBytes),
          signature: signature
        }
      ]
    })

    sleep(100)
    const signedTx = await promiseSignTransaction

    const isValid = await verifyMessage(
      signedTx.transactionBlockBytes,
      signedTx.signature,
      IntentScope.TransactionData
    )
    expect(isValid).toBeTruthy()
  })
})
