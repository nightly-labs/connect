import { Connect, ContentType, getRandomId } from '@nightlylabs/nightly-connect-base'
import { Keypair, LAMPORTS_PER_SOL, SystemProgram, Transaction } from '@solana/web3.js'
import { assert, beforeAll, beforeEach, describe, expect, test } from 'vitest'
import { smartDelay, TEST_RELAY_ENDPOINT } from '../../../commonTestUtils'
import { AppSolana } from './app'
import { SolanaChangeNetworkInput } from './client'
import { HttpClientSolana } from './http-client'
import { ChangeNetworkSolanaRequest } from './requestTypes'
import { TEST_APP_INITIALIZE } from './testUtils'
import { SOLANA_NETWORK } from './utils'

// Edit an assertion and save to see HMR in action
const alice_keypair = Keypair.generate()
describe('Base Client tests', () => {
  let app: AppSolana
  let client: HttpClientSolana
  const clientId = getRandomId()
  beforeAll(async () => {
    app = await AppSolana.build(TEST_APP_INITIALIZE)
    expect(app).toBeDefined()
    assert(app.sessionId !== '')
    client = new HttpClientSolana({ url: TEST_RELAY_ENDPOINT, clientId })
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
  test('#resolveSignTransaction()', async () => {
    const RECEIVER = Keypair.generate()
    const ix = SystemProgram.transfer({
      fromPubkey: alice_keypair.publicKey,
      lamports: LAMPORTS_PER_SOL,
      toPubkey: RECEIVER.publicKey
    })
    const tx = new Transaction().add(ix)
    tx.feePayer = alice_keypair.publicKey
    tx.recentBlockhash = 'E6wypnGQkndknX5Urd5yXV8yxAkbHwD5MJ1aaNKWZBd5'
    const promiseSignTransaction = app.signTransaction(tx)
    await smartDelay()
    // Query for request
    const pendingRequest = (await client.getPendingRequests({ sessionId: app.sessionId }))[0]
    if (pendingRequest.type !== ContentType.SignTransactions) {
      throw new Error('Wrong content type')
    }
    const txToSign = pendingRequest.transactions[0]
    txToSign.sign([alice_keypair])
    await client.resolveSignTransaction({
      requestId: pendingRequest.requestId,
      sessionId: app.sessionId,
      signedTransactions: [txToSign]
    })

    await smartDelay()
    const signed = await promiseSignTransaction
    // Transform to Transaction cuz idk how to verify VersionedTransaction
    const signed_transaction = Transaction.from(signed.serialize())
    assert(signed_transaction.verifySignatures())
  })
  test('#resolveChangeNetwork()', async () => {
    const newNetwork: SolanaChangeNetworkInput = {
      genesisHash: '5eykt4UsFv8P8NJdTREpY1vzqKqZKvdpKuc147dw2N9d',
      url: 'https://solana-mainnet.rpc.extrnode.com/85c27167-63a1-4fa3-9971-fc1df7b132dc'
    }

    const _changedNetwork = app.changeNetwork(newNetwork)
    await smartDelay()

    const pendingRequest = (
      await client.getPendingRequests({ sessionId: app.sessionId })
    )[0] as ChangeNetworkSolanaRequest
    expect(pendingRequest.type).toBe(ContentType.ChangeNetwork)
    expect(pendingRequest.newNetwork.genesisHash).toBe(
      '5eykt4UsFv8P8NJdTREpY1vzqKqZKvdpKuc147dw2N9d'
    )
    expect(pendingRequest.newNetwork.url).toBe(
      'https://solana-mainnet.rpc.extrnode.com/85c27167-63a1-4fa3-9971-fc1df7b132dc'
    )

    const payload = pendingRequest.newNetwork

    await client.resolveChangeNetwork({
      requestId: pendingRequest.requestId,
      sessionId: app.sessionId,
      newNetwork: payload
    })

    await smartDelay()
    const isSuccess = (await _changedNetwork).success
    assert(isSuccess)
  })
  test('#rejectRequest()', async () => {
    try {
      const newNetwork: SolanaChangeNetworkInput = {
        genesisHash: '5eykt4UsFv8P8NJdTREpY1vzqKqZKvdpKuc147dw2N9d',
        url: 'https://solana-mainnet.rpc.extrnode.com/85c27167-63a1-4fa3-9971-fc1df7b132dc'
      }

      // eslint-disable-next-line no-async-promise-executor
      const promiseChangeNetwork = new Promise<void>(async (resolve) => {
        expect(() => app.changeNetwork(newNetwork)).rejects.toThrow('test-error')
        resolve()
      })
      await smartDelay()

      const pendingRequest = (
        await client.getPendingRequests({ sessionId: app.sessionId })
      )[0] as ChangeNetworkSolanaRequest

      await client.rejectRequest({
        requestId: pendingRequest.requestId,
        sessionId: app.sessionId,
        reason: 'test-error'
      })
      await smartDelay()
      await promiseChangeNetwork
    } catch (error) {
      console.log(error)
    }
  })
})
