import { Connect, RELAY_ENDPOINT, smartDelay } from '@nightlylabs/nightly-connect-base'
import { assert, beforeAll, beforeEach, describe, expect, test } from 'vitest'
import { AppPolkadot } from './app'
import { ClientPolkadot } from './client'
import { POLKADOT_NETWORK, TEST_APP_INITIALIZE } from './utils'
import { Keyring } from '@polkadot/keyring'
import { stringToU8a, u8aToHex } from '@polkadot/util'
import { signatureVerify } from '@polkadot/util-crypto'
import { SignerPayloadJSON, SignerPayloadRaw } from '@polkadot/types/types'
import { TypeRegistry } from '@polkadot/types'

// Edit an assertion and save to see HMR in action
const alice_keypair = new Keyring()
const aliceKayringPair = alice_keypair.createFromUri('//Alice')
describe('Base Client tests', () => {
  let app: AppPolkadot
  let client: ClientPolkadot
  beforeAll(async () => {
    app = await AppPolkadot.build(TEST_APP_INITIALIZE)
    expect(app).toBeDefined()
    assert(app.sessionId !== '')
    client = await ClientPolkadot.create({ url: RELAY_ENDPOINT })
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
    assert(info.network === POLKADOT_NETWORK)
    // assert(info.version === testAppBaseInitialize.version)
  })
  test('#connect()', async () => {
    const msg: Connect = {
      publicKeys: ['1', '2'],
      sessionId: app.sessionId
    }
    await client.connect(msg)
  })

  test('#on("signPayload")', async () => {
    const payloadToSign: SignerPayloadJSON = {
      address: aliceKayringPair.address,
      blockHash: '0xe1b1dda72998846487e4d858909d4f9a6bbd6e338e4588e5d809de16b1317b80',
      blockNumber: '0x00000393',
      era: '0x3601',
      genesisHash: '0x242a54b35e1aad38f37b884eddeb71f6f9931b02fac27bf52dfb62ef754e5e62',
      method: '0x040105fa8eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a4882380100',
      nonce: '0x0000000000000000',
      signedExtensions: [
        'CheckSpecVersion',
        'CheckTxVersion',
        'CheckGenesis',
        'CheckMortality',
        'CheckNonce',
        'CheckWeight',
        'ChargeTransactionPayment'
      ],
      specVersion: '0x00000026',
      tip: '0x00000000000000000000000000000000',
      transactionVersion: '0x00000005',
      version: 4
    }
    const registry = new TypeRegistry()

    const signatureExpected = registry
      .createType('ExtrinsicPayload', payloadToSign, { version: payloadToSign.version })
      .sign(aliceKayringPair)

    client.on('signTransactions', async (e) => {
      // resolve
      const payload = e.transactions[0]
      const signature = registry
        .createType('ExtrinsicPayload', payload, { version: payload.version })
        .sign(aliceKayringPair)

      await client.resolveSignTransaction({
        requestId: e.requestId,
        signedTransactions: [{ signature: signature.signature, id: new Date().getTime() }]
      })
    })

    await smartDelay()
    const signed = await app.signPayload(payloadToSign)

    expect(signed.signature).toEqual(signatureExpected.signature)
    client.removeListener('signTransactions')
  })

  test('#on("signRaw")', async () => {
    const messageBytes = stringToU8a('LOVE NIGHTLY')
    const payload: SignerPayloadRaw = {
      type: 'bytes',
      data: u8aToHex(messageBytes),
      address: aliceKayringPair.address
    }
    client.on('signTransactions', async (e) => {
      // resolve

      const signedBytes = aliceKayringPair.sign(e.transactions[0].data)
      const signature = u8aToHex(signedBytes)
      await client.resolveSignTransaction({
        requestId: e.requestId,
        signedTransactions: [{ signature: signature, id: new Date().getTime() }]
      })
    })

    await smartDelay()
    const signed = await app.signRaw(payload)
    const isSignatureValid = signatureVerify(
      payload.data,
      signed.signature,
      aliceKayringPair.address
    )

    assert(isSignatureValid.isValid)
  })
})
