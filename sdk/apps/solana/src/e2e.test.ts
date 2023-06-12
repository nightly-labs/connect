import { Keypair, LAMPORTS_PER_SOL, SystemProgram, Transaction } from '@solana/web3.js'
import { Connect, RELAY_ENDPOINT, smartDelay } from '@nightlylabs/nightly-connect-base'
import { sha256 } from 'js-sha256'
import nacl from 'tweetnacl'
import { assert, beforeAll, beforeEach, describe, expect, test } from 'vitest'
import { AppSolana } from './app'
import { ClientSolana } from './client'
import { SOLANA_NETWORK, TEST_APP_INITIALIZE } from './utils'

// Edit an assertion and save to see HMR in action
const alice_keypair = Keypair.generate()
describe('Base Client tests', () => {
  let app: AppSolana
  let client: ClientSolana
  beforeAll(async () => {
    app = await AppSolana.build(TEST_APP_INITIALIZE)
    expect(app).toBeDefined()
    assert(app.sessionId !== '')
    client = await ClientSolana.create({ url: RELAY_ENDPOINT })
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
    assert(info.network === SOLANA_NETWORK)
    // assert(info.version === testAppBaseInitialize.version)
  })
  test('#connect()', async () => {
    const msg: Connect = {
      publicKeys: ['1', '2'],
      sessionId: app.sessionId
    }
    await client.connect(msg)
  })
  test('#on("signTransactions")', async () => {
    const RECEIVER = Keypair.generate()
    const ix = SystemProgram.transfer({
      fromPubkey: alice_keypair.publicKey,
      lamports: LAMPORTS_PER_SOL,
      toPubkey: RECEIVER.publicKey
    })
    const tx = new Transaction().add(ix)
    tx.feePayer = alice_keypair.publicKey
    tx.recentBlockhash = 'E6wypnGQkndknX5Urd5yXV8yxAkbHwD5MJ1aaNKWZBd5'

    client.on('signTransactions', async (e) => {
      const tx = e.transactions[0]
      tx.sign([alice_keypair])
      // resolve
      await client.resolveSignTransaction({
        requestId: e.requestId,
        signedTransactions: [tx]
      })
    })

    await smartDelay()
    const signed = await app.signTransaction(tx)
    // Transform to Transaction cuz idk how to verify VersionedTransaction
    const signed_transaction = Transaction.from(signed.serialize())
    assert(signed_transaction.verifySignatures())
  })
  test('#on("signMessages")', async () => {
    const msgToSign = 'Hello World'
    client.on('signMessages', async (e) => {
      const msg = e.messages[0].message
      const encoded = Uint8Array.from(sha256.array(msg))
      const signature = nacl.sign.detached(encoded, alice_keypair.secretKey)
      // resolve
      await client.resolveSignMessage({
        requestId: e.responseId,
        signature: signature
      })
    })
    await smartDelay()
    const signature = await app.signMessage(msgToSign)
    const verified = nacl.sign.detached.verify(
      Uint8Array.from(sha256.array(msgToSign)),
      signature,
      alice_keypair.publicKey.toBuffer()
    )
    assert(verified)
  })
})
