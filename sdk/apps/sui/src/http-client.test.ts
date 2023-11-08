import { assert, beforeAll, beforeEach, describe, expect, test } from 'vitest'
import { AppSui } from './app'
import { signTransactionBlock, SUI_NETWORK } from './utils'
import { TEST_APP_INITIALIZE } from './testUtils'
import { Connect, getRandomId, ContentType } from '@nightlylabs/nightly-connect-base'
import { HttpClientSui } from './http-client'
import { fromB64, toB64 } from '@mysten/sui.js/utils'
import { TransactionBlock } from '@mysten/sui.js/transactions'
import { Ed25519Keypair } from '@mysten/sui.js/keypairs/ed25519'
import { verifyTransactionBlock } from '@mysten/sui.js/verify'
import { hexToBytes } from '@noble/hashes/utils'
import { WalletAccount } from '@mysten/wallet-standard'
import { smartDelay, TEST_RELAY_ENDPOINT } from '../../../commonTestUtils'

// Edit an assertion and save to see HMR in action
const ALICE_PRIVE_KEY = '4aa55c99d633c646b8dc423eed56e0fc39bdbca6ac6d8c53cc6e4decda27d970'
const alice_keypair = Ed25519Keypair.fromSecretKey(hexToBytes(ALICE_PRIVE_KEY))

const RECEIVER_SUI_ADDRESS = '0x19b78fbdf0f8fdb942abd67b8628ca80079aeb786cec0235d65af9b65019b59f'
const aliceWalletAccount: WalletAccount = {
  address: alice_keypair.getPublicKey().toSuiAddress(),
  publicKey: alice_keypair.getPublicKey().toBytes(),
  chains: ['sui:testnet'],
  features: ['sui:signAndExecuteTransactionBlock'],
  label: ''
}
describe('SUI http-client tests', () => {
  let app: AppSui
  let client: HttpClientSui
  const clientId = getRandomId()

  beforeAll(async () => {
    app = await AppSui.build(TEST_APP_INITIALIZE)
    expect(app).toBeDefined()
    assert(app.sessionId !== '')
    client = new HttpClientSui({ url: TEST_RELAY_ENDPOINT, clientId })
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
    await smartDelay()
    // Query for request
    const pendingRequest = (await client.getPendingRequests({ sessionId: app.sessionId }))[0]
    if (pendingRequest.type !== ContentType.SignTransactions) {
      throw new Error('Wrong content type')
    }
    const pendingTx = pendingRequest.transactions[0].transaction
    const { signature, transactionBlockBytes } = await signTransactionBlock(
      TransactionBlock.from(pendingTx),
      alice_keypair
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

    await smartDelay()
    const signedTx = await promiseSignTransaction

    try {
      // Will throw if invalid
      await verifyTransactionBlock(fromB64(signedTx.transactionBlockBytes), signedTx.signature)
    } catch (error) {
      assert(false, 'Transaction block is invalid')
    }
  })
})
