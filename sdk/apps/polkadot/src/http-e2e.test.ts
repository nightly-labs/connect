import {
  ContentType,
  RELAY_ENDPOINT,
  getRandomId,
  smartDelay
} from '@nightlylabs/nightly-connect-base'
import { assert, beforeAll, beforeEach, describe, expect, test } from 'vitest'
import { AppPolkadot } from './app'
import { TEST_APP_INITIALIZE } from './utils'

import { ApiPromise, WsProvider } from '@polkadot/api'
import Keyring from '@polkadot/keyring'
import { SignerPayloadRaw } from '@polkadot/types/types'
import { u8aToHex } from '@polkadot/util'
import { decodeAddress, signatureVerify } from '@polkadot/util-crypto'
import { HttpClientPolkadot, HttpConnect } from './http-client'

// Edit an assertion and save to see HMR in action
const alice_keypair = new Keyring()
alice_keypair.setSS58Format(42)
const aliceKeyringPair = alice_keypair.createFromUri('//Alice')
const RECEIVER = '5CFRopxy991HCJj1HYtUQjaaBMw9iRLE9jxPndBsgdCjeJj5'
describe('Base Client tests', () => {
  let app: AppPolkadot
  let client: HttpClientPolkadot
  let provider: WsProvider
  let polkadotApi: ApiPromise
  const clientId = getRandomId()
  beforeAll(async () => {
    app = await AppPolkadot.build(TEST_APP_INITIALIZE)
    expect(app).toBeDefined()
    assert(app.sessionId !== '')
    client = new HttpClientPolkadot({ url: RELAY_ENDPOINT, clientId })
    provider = new WsProvider('wss://ws.test.azero.dev/')
    polkadotApi = await ApiPromise.create({
      provider
    })
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
    assert(info.network === TEST_APP_INITIALIZE.network)
    // assert(info.version === testAppBaseInitialize.version)
  })
  test('#connect()', async () => {
    const msg: HttpConnect = {
      publicKeys: [aliceKeyringPair.address],
      sessionId: app.sessionId,
      walletsMetadata: [
        {
          address: aliceKeyringPair.address,
          name: 'Alice',
          type: 'ed25519'
        }
      ]
    }
    await client.connect(msg)
  })
  test('#resolveSignTransaction()', async () => {
    const payload = polkadotApi.tx.balances.transfer(RECEIVER, 50000000)

    const promiseSignTransaction = payload.signAsync(RECEIVER, { signer: app.signer })
    await smartDelay(500)
    // Query for request
    const pendingRequest = (await client.getPendingRequests({ sessionId: app.sessionId }))[0]
    if (pendingRequest.content.type !== ContentType.SignTransactions) {
      throw new Error('Wrong content type')
    }
    const transactionToSign = JSON.parse(
      pendingRequest.content.transactions[0].transaction
    ) as SignerPayloadRaw
    const signature = aliceKeyringPair.sign(transactionToSign.data, { withType: true })

    await client.resolveSignTransaction({
      requestId: pendingRequest.requestId,
      sessionId: app.sessionId,
      signedTransactions: [{ signature: u8aToHex(signature), id: new Date().getTime() }],
      network: TEST_APP_INITIALIZE.network
    })
    const signed = await promiseSignTransaction
    const verify = signatureVerify(
      transactionToSign.data,
      signed.signature,
      u8aToHex(decodeAddress(aliceKeyringPair.address))
    )

    expect(verify.isValid).toBeTruthy()
  })
})
