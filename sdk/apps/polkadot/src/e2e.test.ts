import { ContentType } from '@nightlylabs/nightly-connect-base'
import { ApiPromise, WsProvider } from '@polkadot/api'
import { Keyring } from '@polkadot/keyring'
import { SignerPayloadJSON, SignerPayloadRaw } from '@polkadot/types/types'
import { u8aToHex } from '@polkadot/util'
import { cryptoWaitReady, decodeAddress, signatureVerify } from '@polkadot/util-crypto'
import { assert, beforeAll, beforeEach, describe, expect, test } from 'vitest'
import { AppPolkadot } from './app'
import { ClientPolkadot, Connect } from './client'
import { TEST_APP_INITIALIZE } from './testUtils'
import { SignTransactionsPolkadotRequest } from './requestTypes'
import { TypeRegistry } from '@polkadot/types'
import { smartDelay, TEST_RELAY_ENDPOINT } from '../../../commonTestUtils'

// Edit an assertion and save to see HMR in action
const alice_keypair = new Keyring()
alice_keypair.setSS58Format(42)
const aliceKeyringPair = alice_keypair.createFromUri('//Alice')
const RECEIVER = '5CFRopxy991HCJj1HYtUQjaaBMw9iRLE9jxPndBsgdCjeJj5'
describe('Base Client tests', () => {
  let app: AppPolkadot
  let client: ClientPolkadot
  let provider: WsProvider
  let polkadotApi: ApiPromise

  beforeAll(async () => {
    await cryptoWaitReady()
    app = await AppPolkadot.build(TEST_APP_INITIALIZE)
    expect(app).toBeDefined()
    assert(app.sessionId !== '')
    client = await ClientPolkadot.create({ url: TEST_RELAY_ENDPOINT }, TEST_APP_INITIALIZE.network)
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
    const msg: Connect = {
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

  test('#on("signTransactions")', async () => {
    const payload = polkadotApi.tx.balances.transfer(RECEIVER, 50000000)

    let payloadToSign: string | Uint8Array = ''

    client.on('signTransactions', async (e) => {
      // Asset network
      expect(e.network).toBe(TEST_APP_INITIALIZE.network)
      // resolve
      const payload = e.transactions[0] as SignerPayloadRaw | SignerPayloadJSON
      let signature: `0x${string}` = '0x'
      if (typeof payload === 'object') {
        if ('data' in payload) {
          payloadToSign = payload.data
          signature = u8aToHex(aliceKeyringPair.sign(payload.data, { withType: true }))
        } else {
          const registry = new TypeRegistry()
          registry.setSignedExtensions(payload.signedExtensions)
          const extrinsicPayload = registry.createType('ExtrinsicPayload', payload, {
            version: payload.version
          })
          payloadToSign = extrinsicPayload.toU8a({ method: true })
          const signedPayload = extrinsicPayload.sign(aliceKeyringPair)
          signature = signedPayload.signature
        }
      }
      // TODO seems like signature is 65 bytes long, but it should be 64
      // console.log('signature', u8aToHex(signature.slice(1, 64)))
      await client.resolveSignTransaction({
        requestId: e.requestId,
        // TODO Not sure what id here means
        signedTransactions: [{ signature: signature, id: new Date().getTime() }]
      })
    })
    await smartDelay()
    const signed = await payload.signAsync(RECEIVER, { signer: app.signer })
    const verify = signatureVerify(
      payloadToSign,
      signed.signature,
      u8aToHex(decodeAddress(aliceKeyringPair.address))
    )

    expect(verify.isValid).toBeTruthy()
  })
  test('#getPendingRequests()', async () => {
    client.removeListener('signTransactions')
    const payload = polkadotApi.tx.balances.transfer(RECEIVER, 50000000)
    payload.signAsync(RECEIVER, { signer: app.signer })
    payload.signAsync(RECEIVER, { signer: app.signer })
    await smartDelay(500)
    await smartDelay(500)
    const requests = await client.getPendingRequests()
    expect(requests.length).toBe(2)
    expect(requests[0].type).toBe(ContentType.SignTransactions)
    expect(requests[1].type).toBe(ContentType.SignTransactions)
    const payload1 = requests[0] as SignTransactionsPolkadotRequest
    expect(payload1.network).toBe(TEST_APP_INITIALIZE.network)
    expect(payload1.transactions.length).toBe(1)
    expect(payload1.transactions[0].address).toBe(RECEIVER)
  })
})
