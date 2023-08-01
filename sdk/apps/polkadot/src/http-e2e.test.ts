// import { assert, beforeAll, beforeEach, describe, expect, test } from 'vitest'
// import { AppPolkadot } from './app'
// import { POLKADOT_NETWORK, TEST_APP_INITIALIZE } from './utils'
// import {
//   Connect,
//   getRandomId,
//   RELAY_ENDPOINT,
//   smartDelay,
//   ContentType
// } from '@nightlylabs/nightly-connect-base'

// import { HttpClientPolkadot } from './http-client'
// import Keyring from '@polkadot/keyring'
// import { SignerPayloadJSON, SignerPayloadRaw } from '@polkadot/types/types'
// import { stringToU8a, u8aToHex } from '@polkadot/util'
// import { signatureVerify } from '@polkadot/util-crypto'
// import { TypeRegistry } from '@polkadot/types'

// // Edit an assertion and save to see HMR in action
// const alice_keypair = new Keyring()
// const aliceKayringPair = alice_keypair.createFromUri('//Alice')
// describe('Base Client tests', () => {
//   let app: AppPolkadot
//   let client: HttpClientPolkadot
//   const clientId = getRandomId()
//   beforeAll(async () => {
//     app = await AppPolkadot.build(TEST_APP_INITIALIZE)
//     expect(app).toBeDefined()
//     assert(app.sessionId !== '')
//     client = new HttpClientPolkadot({ url: RELAY_ENDPOINT, clientId })
//   })
//   beforeEach(async () => {
//     await smartDelay()
//   })
//   test('#getInfo()', async () => {
//     const info = await client.getInfo(app.sessionId)
//     expect(info).toBeDefined()
//     assert(info.appMetadata.additionalInfo === TEST_APP_INITIALIZE.appMetadata.additionalInfo)
//     assert(info.appMetadata.description === TEST_APP_INITIALIZE.appMetadata.description)
//     assert(info.appMetadata.icon === TEST_APP_INITIALIZE.appMetadata.icon)
//     assert(info.appMetadata.name === TEST_APP_INITIALIZE.appMetadata.name)
//     assert(info.network === POLKADOT_NETWORK)
//     // assert(info.version === testAppBaseInitialize.version)
//   })
//   test('#connect()', async () => {
//     const msg: Connect = {
//       publicKeys: ['1', '2'],
//       sessionId: app.sessionId
//     }
//     await client.connect(msg)
//   })
//   test('#resolveSignTransaction() using signRaw', async () => {
//     const RECEIVER = new Keyring().createFromUri('//Receiver')

//     const messageBytes = stringToU8a('LOVE NIGHTLY')
//     const payload: SignerPayloadRaw = {
//       type: 'bytes',
//       data: u8aToHex(messageBytes),
//       address: RECEIVER.address
//     }

//     const promiseSignTransaction = app.signRaw(payload)
//     await smartDelay()
//     // Query for request
//     const pendingRequest = (await client.getPendingRequests({ sessionId: app.sessionId }))[0]
//     if (pendingRequest.content.type !== ContentType.SignTransactions) {
//       throw new Error('Wrong content type')
//     }
//     // Wonder if this step should be done by the client)
//     const signedBytes = aliceKayringPair.sign(
//       JSON.parse(pendingRequest.content.transactions[0].transaction).data
//     )
//     const signature = u8aToHex(signedBytes)

//     await client.resolveSignTransaction({
//       requestId: pendingRequest.requestId,
//       sessionId: app.sessionId,
//       signedTransactions: [{ signature: signature, id: new Date().getTime() }]
//     })

//     await smartDelay()
//     const signed = await promiseSignTransaction
//     // Transform to Transaction cuz idk how to verify VersionedTransaction
//     const isSignatureValid = signatureVerify(
//       payload.data,
//       signed.signature,
//       aliceKayringPair.address
//     )
//     assert(isSignatureValid.isValid)
//   })

//   test('#resolveSignTransaction() using singPayload', async () => {
//     const payloadToSign: SignerPayloadJSON = {
//       address: aliceKayringPair.address,
//       blockHash: '0xe1b1dda72998846487e4d858909d4f9a6bbd6e338e4588e5d809de16b1317b80',
//       blockNumber: '0x00000393',
//       era: '0x3601',
//       genesisHash: '0x242a54b35e1aad38f37b884eddeb71f6f9931b02fac27bf52dfb62ef754e5e62',
//       method: '0x040105fa8eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a4882380100',
//       nonce: '0x0000000000000000',
//       signedExtensions: [
//         'CheckSpecVersion',
//         'CheckTxVersion',
//         'CheckGenesis',
//         'CheckMortality',
//         'CheckNonce',
//         'CheckWeight',
//         'ChargeTransactionPayment'
//       ],
//       specVersion: '0x00000026',
//       tip: '0x00000000000000000000000000000000',
//       transactionVersion: '0x00000005',
//       version: 4
//     }
//     const registry = new TypeRegistry()

//     const signatureExpected = registry
//       .createType('ExtrinsicPayload', payloadToSign, { version: payloadToSign.version })
//       .sign(aliceKayringPair)

//     const promiseSignTransaction = app.signPayload(payloadToSign)
//     await smartDelay()
//     // Query for request
//     const pendingRequest = (await client.getPendingRequests({ sessionId: app.sessionId }))[0]
//     if (pendingRequest.content.type !== ContentType.SignTransactions) {
//       throw new Error('Wrong content type')
//     }
//     // Wonder if this step should be done by the client)

//     const payload = JSON.parse(pendingRequest.content.transactions[0].transaction)
//     const signature = registry
//       .createType('ExtrinsicPayload', payload, { version: payload.version })
//       .sign(aliceKayringPair)

//     await client.resolveSignTransaction({
//       requestId: pendingRequest.requestId,
//       sessionId: app.sessionId,
//       signedTransactions: [{ signature: signature.signature, id: new Date().getTime() }]
//     })

//     await smartDelay()
//     const signed = await promiseSignTransaction
//     // Transform to Transaction cuz idk how to verify VersionedTransaction
//     expect(signed.signature).toEqual(signatureExpected.signature)
//   })
// })
