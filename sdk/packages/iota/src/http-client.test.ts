import { Ed25519Keypair } from '@iota/iota-sdk/keypairs/ed25519'
import { Transaction } from '@iota/iota-sdk/transactions'
import { fromB64, toB64 } from '@iota/iota-sdk/utils'
import { verifyTransactionSignature } from '@iota/iota-sdk/verify'
import { WalletAccount } from '@iota/wallet-standard'
import { Connect, ContentType, getRandomId } from '@nightlylabs/nightly-connect-base'
import { hexToBytes } from '@noble/hashes/utils'
import { assert, beforeAll, beforeEach, describe, expect, test } from 'vitest'
import { TEST_RELAY_ENDPOINT, smartDelay } from '../../../commonTestUtils'
import { AppIota } from './app'
import { HttpClientIota } from './http-client'
import { TEST_APP_INITIALIZE } from './testUtils'
import { IOTA_NETWORK, signTransactionBlock } from './utils'

// Edit an assertion and save to see HMR in action
const ALICE_PRIVE_KEY = '4aa55c99d633c646b8dc423eed56e0fc39bdbca6ac6d8c53cc6e4decda27d970'
const alice_keypair = Ed25519Keypair.fromSecretKey(hexToBytes(ALICE_PRIVE_KEY))

const RECEIVER_SUI_ADDRESS = '0x19b78fbdf0f8fdb942abd67b8628ca80079aeb786cec0235d65af9b65019b59f'
const aliceWalletAccount: WalletAccount = {
  address: alice_keypair.getPublicKey().toIotaAddress(),
  publicKey: alice_keypair.getPublicKey().toRawBytes(),
  chains: ['iota:testnet'],
  features: ['iota:signAndExecuteTransactionBlock'],
  label: ''
}
describe('IOTA http-client tests', () => {
  let app: AppIota
  let client: HttpClientIota
  const clientId = getRandomId()

  beforeAll(async () => {
    app = await AppIota.build(TEST_APP_INITIALIZE)
    expect(app).toBeDefined()
    assert(app.sessionId !== '')
    client = new HttpClientIota({ url: TEST_RELAY_ENDPOINT, clientId })
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
  test('#resolveSignTransaction()', async () => {
    const tx = new Transaction()
    const coin = tx.splitCoins(tx.gas, [tx.pure.u64(100)])
    tx.transferObjects([coin], tx.pure.address(RECEIVER_SUI_ADDRESS))
    tx.setSenderIfNotSet(RECEIVER_SUI_ADDRESS)

    const promiseSignTransaction = app.signTransactionBlock({
      transaction: tx,
      account: aliceWalletAccount,
      chain: 'iota:testnet'
    })
    await smartDelay()
    // Query for request
    const pendingRequest = (await client.getPendingRequests({ sessionId: app.sessionId }))[0]
    if (pendingRequest.type !== ContentType.SignTransactions) {
      throw new Error('Wrong content type')
    }
    const pendingTx = pendingRequest.transactions[0].transaction
    const { signature, transactionBlockBytes } = await signTransactionBlock(
      Transaction.from(pendingTx),
      alice_keypair
    )

    await client.resolveSignTransaction({
      requestId: pendingRequest.requestId,
      sessionId: app.sessionId,
      signedTransactions: [
        {
          bytes: toB64(transactionBlockBytes),
          signature: signature
        }
      ]
    })

    await smartDelay()
    const signedTx = await promiseSignTransaction

    try {
      // Will throw if invalid
      await verifyTransactionSignature(fromB64(signedTx.bytes), signedTx.signature)
    } catch (error) {
      assert(false, 'Transaction block is invalid')
    }
  })
})
