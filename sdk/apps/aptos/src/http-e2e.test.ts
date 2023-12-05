import { AccountInfo } from '@aptos-labs/wallet-adapter-core'
import { ContentType, getRandomId } from '@nightlylabs/nightly-connect-base'
import {
  AptosAccount,
  FaucetClient,
  Network,
  TransactionBuilderEd25519,
  TxnBuilderTypes,
  Types
} from 'aptos'
import { assert, beforeAll, beforeEach, describe, expect, test, vi } from 'vitest'
import { TEST_RELAY_ENDPOINT, smartDelay } from '../../../commonTestUtils'
import { AppAptos } from './app'
import { AptosConnect } from './client'
import { HttpClientAptos } from './http-client'
import { FAUCET_URL, NODE_URL, TEST_APP_INITIALIZE } from './testUtils'
import { APTOS_NETWORK } from './utils'

const aptosClient = new FaucetClient(NODE_URL, FAUCET_URL)
const alice = new AptosAccount()
const olive = new AptosAccount()
const aliceAccountInfo: AccountInfo = {
  address: alice.address().toString(),
  publicKey: alice.pubKey().toString()
}
describe('Base Client tests', () => {
  let app: AppAptos
  let client: HttpClientAptos
  const clientId = getRandomId()
  beforeAll(async () => {
    await aptosClient.fundAccount(alice.address(), 10 ** 9)
    app = await AppAptos.build(TEST_APP_INITIALIZE)
    expect(app).toBeDefined()
    assert(app.sessionId !== '')
    client = new HttpClientAptos({ url: TEST_RELAY_ENDPOINT, clientId })
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
    // assert(info.version === testAppBaseInitialize.version)
  })
  test('#connect()', async () => {
    const fn = vi.fn()
    const fnNetworkInfo = vi.fn()
    app.on('userConnected', (e) => {
      fn(e.accounts[0])
      fnNetworkInfo(e.networkInfo)
    })
    const msg: AptosConnect = {
      publicKeys: [aliceAccountInfo],
      sessionId: app.sessionId,
      networkInfo: {
        name: Network.MAINNET
      }
    }
    await client.connect(msg)
    await smartDelay()
    expect(fn).toHaveBeenCalledWith(aliceAccountInfo)
    expect(fnNetworkInfo).toHaveBeenCalledWith({
      name: Network.MAINNET
    })
  })
  test('#resolveSignTransaction()', async () => {
    const txnBuilder = new TransactionBuilderEd25519(
      (signingMessage: TxnBuilderTypes.SigningMessage) => {
        const sigHexStr = alice.signBuffer(signingMessage)
        return new TxnBuilderTypes.Ed25519Signature(sigHexStr.toUint8Array())
      },
      alice.pubKey().toUint8Array()
    )
    const txToSign = {
      type: 'entry_function_payload',
      arguments: [olive.address().toString(), 1000],
      function: '0x1::coin::transfer',
      type_arguments: ['0x1::aptos_coin::AptosCoin']
    }
    const promiseSignTransaction = app.signAndSubmitTransaction(txToSign)
    await smartDelay()
    // Query for request
    const pendingRequest = (await client.getPendingRequests({ sessionId: app.sessionId }))[0]
    if (pendingRequest.type !== ContentType.SignTransactions) {
      throw new Error('Wrong content type')
    }
    const tx = await aptosClient.generateTransaction(
      alice.address(),
      pendingRequest.transactions[0].transaction as Types.EntryFunctionPayload
    )
    const signedTxn = txnBuilder.sign(tx)
    const hash = await aptosClient.submitTransaction(signedTxn)

    await client.resolveSignTransaction({
      requestId: pendingRequest.requestId,
      sessionId: app.sessionId,
      transactionHashes: [{ hash: hash.hash }]
    })

    await smartDelay()
    const signed = await promiseSignTransaction
    assert(signed.hash.length === 66)
    assert(signed.hash === hash.hash)
  })
})
