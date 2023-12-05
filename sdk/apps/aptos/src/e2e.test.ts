import {
  AccountInfo,
  SignMessagePayload,
  SignMessageResponse
} from '@aptos-labs/wallet-adapter-core'
import {
  AptosAccount,
  CoinClient,
  FaucetClient,
  Network,
  TransactionBuilderEd25519,
  TxnBuilderTypes,
  Types
} from 'aptos'
import { assert, beforeAll, beforeEach, describe, expect, test, vi } from 'vitest'
import { TEST_RELAY_ENDPOINT, smartDelay } from '../../../commonTestUtils'
import { AppAptos } from './app'
import { AptosConnect, ClientAptos } from './client'
import { FAUCET_URL, NODE_URL, TEST_APP_INITIALIZE } from './testUtils'
import { APTOS_NETWORK } from './utils'

const aptosClient = new FaucetClient(NODE_URL, FAUCET_URL) // <:!:section_1
const alice = new AptosAccount()
const aliceAccountInfo: AccountInfo = {
  address: alice.address().toString(),
  publicKey: alice.pubKey().toString()
}
const olive = new AptosAccount()
const oliveAccountInfo: AccountInfo = {
  address: olive.address().toString(),
  publicKey: olive.pubKey().toString()
}
const coinClient = new CoinClient(aptosClient)
describe('Base Client tests', () => {
  let app: AppAptos
  let client: ClientAptos
  beforeAll(async () => {
    await aptosClient.fundAccount(alice.address(), 10 ** 9)
    app = await AppAptos.build(TEST_APP_INITIALIZE)
    expect(app).toBeDefined()
    assert(app.sessionId !== '')
    client = await ClientAptos.create({ url: TEST_RELAY_ENDPOINT })
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
      networkInfo: { name: Network.MAINNET }
    }
    await client.connect(msg)
    await smartDelay()
    expect(fn).toHaveBeenCalledWith(aliceAccountInfo)
    expect(fnNetworkInfo).toHaveBeenCalledWith({ name: Network.MAINNET })
  })
  test('#on("signTransactions")', async () => {
    client.on('signTransactions', async (e) => {
      const tx = await aptosClient.generateTransaction(
        alice.address(),
        e.transactions[0].transaction as Types.EntryFunctionPayload
      )
      const txnBuilder = new TransactionBuilderEd25519(
        (signingMessage: TxnBuilderTypes.SigningMessage) => {
          const sigHexStr = alice.signBuffer(signingMessage)
          return new TxnBuilderTypes.Ed25519Signature(sigHexStr.toUint8Array())
        },
        alice.pubKey().toUint8Array()
      )
      const signedTxn = txnBuilder.sign(tx)
      const hash = await aptosClient.submitTransaction(signedTxn)

      await client.resolveSignTransaction({
        requestId: e.requestId,
        transactionHashes: [{ hash: hash.hash }]
      })
    })
    smartDelay()
    const tx = {
      type: 'entry_function_payload',
      arguments: [olive.address().toString(), 1000],
      function: '0x1::coin::transfer',
      type_arguments: ['0x1::aptos_coin::AptosCoin']
    }
    const signedtx = await app.signAndSubmitTransaction(tx)
    assert(signedtx.hash.length === 66)
  })
  test('#on("signMessages")', async () => {
    const msgToSign: SignMessagePayload = {
      message: 'I love Nightly',
      nonce: '1234',
      address: true
    }
    const signMsgResponse: SignMessageResponse = {
      fullMessage: 'I love Nightly',
      message: 'I love Nightly',
      nonce: '1234',
      prefix: 'APTOS',
      signature: 'signature',
      bitmap: undefined
    }
    client.on('signMessages', async (e) => {
      await client.resolveSignMessage({
        requestId: e.requestId,
        response: signMsgResponse
      })
    })
    await smartDelay()
    const signedMsg = await app.signMessage(msgToSign)
    assert(JSON.stringify(signedMsg) === JSON.stringify(signMsgResponse))
  })
  // test('#getPendingRequests()', async () => {
  //   client.removeListener('signTransactions')
  //   const RECEIVER = Keypair.generate()
  //   const ix = SystemProgram.transfer({
  //     fromPubkey: alice_keypair.publicKey,
  //     lamports: LAMPORTS_PER_SOL,
  //     toPubkey: RECEIVER.publicKey
  //   })
  //   const tx = new Transaction().add(ix)
  //   tx.feePayer = alice_keypair.publicKey
  //   tx.recentBlockhash = 'E6wypnGQkndknX5Urd5yXV8yxAkbHwD5MJ1aaNKWZBd5'
  //   app.signTransaction(tx)
  //   app.signTransaction(tx)
  //   await smartDelay(500)
  //   const requests = await client.getPendingRequests()
  //   expect(requests.length).toBe(2)
  //   expect(requests[0].type).toBe(ContentType.SignTransactions)
  //   expect(requests[1].type).toBe(ContentType.SignTransactions)
  //   const payload1 = requests[0] as SignTransactionsSolanaRequest
  //   expect(payload1.transactions.length).toBe(1)
  // })
})
