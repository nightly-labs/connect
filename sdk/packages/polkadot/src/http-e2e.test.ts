import { ContentType, getRandomId } from '@nightlylabs/nightly-connect-base'
import { assert, beforeAll, beforeEach, describe, expect, test } from 'vitest'
import { AppPolkadot } from './app'
import { TEST_APP_INITIALIZE } from './testUtils'

import { ApiPromise, WsProvider } from '@polkadot/api'
import Keyring from '@polkadot/keyring'
import { SignerPayloadJSON, SignerPayloadRaw } from '@polkadot/types/types'
import { u8aToHex } from '@polkadot/util'
import { decodeAddress, signatureVerify } from '@polkadot/util-crypto'
import { HttpClientPolkadot, HttpConnect } from './http-client'
import { TypeRegistry } from '@polkadot/types'
import { smartDelay, TEST_RELAY_ENDPOINT } from '../../../commonTestUtils'

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
    client = new HttpClientPolkadot({ url: TEST_RELAY_ENDPOINT, clientId })
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
    await smartDelay(1000)
    // Query for request
    const pendingRequest = (await client.getPendingRequests({ sessionId: app.sessionId }))[0]
    if (pendingRequest.type !== ContentType.SignTransactions) {
      throw new Error('Wrong content type')
    }
    // Assert network
    expect(pendingRequest.network).toBe(TEST_APP_INITIALIZE.network)
    const transactionToSign = pendingRequest.transactions[0] as SignerPayloadRaw | SignerPayloadJSON
    let signature: `0x${string}` = '0x'
    let payloadToSign: string | Uint8Array = ''
    if (typeof payload === 'object') {
      if ('data' in payload) {
        const rawPayload = transactionToSign as SignerPayloadRaw
        payloadToSign = rawPayload.data
        signature = u8aToHex(aliceKeyringPair.sign(rawPayload.data, { withType: true }))
      } else if ('version' in payload) {
        const jsonPayload = transactionToSign as SignerPayloadJSON
        const registry = new TypeRegistry()
        registry.setSignedExtensions(jsonPayload.signedExtensions)
        const extrinsicPayload = registry.createType('ExtrinsicPayload', jsonPayload, {
          version: jsonPayload.version
        })
        payloadToSign = extrinsicPayload.toU8a({ method: true })
        const signedPayload = extrinsicPayload.sign(aliceKeyringPair)
        signature = signedPayload.signature
      }
    }

    await client.resolveSignTransaction({
      requestId: pendingRequest.requestId,
      sessionId: app.sessionId,
      signedTransactions: [{ signature: signature, id: new Date().getTime() }],
      network: TEST_APP_INITIALIZE.network
    })
    const signed = await promiseSignTransaction
    const verify = signatureVerify(
      payloadToSign,
      signed.signature,
      u8aToHex(decodeAddress(aliceKeyringPair.address))
    )

    expect(verify.isValid).toBeTruthy()
  })
})
