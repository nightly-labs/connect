import { Account, Aptos, Ed25519PrivateKey, Network } from '@aptos-labs/ts-sdk'
import {
  AccountInfo,
  AptosChangeNetworkInput,
  AptosSignMessageInput,
  NetworkInfo,
  UserResponseStatus
} from '@aptos-labs/wallet-standard'
import { ContentType, getRandomId } from '@nightlylabs/nightly-connect-base'
import { assert, beforeAll, beforeEach, describe, expect, test, vi } from 'vitest'
import { TEST_RELAY_ENDPOINT, smartDelay } from '../../../commonTestUtils'
import { AppAptos } from './app'
import { HttpClientAptos } from './http-client'
import {
  ChangeNetworkAptosRequest,
  SignMessagesAptosRequest,
  SignTransactionsAptosRequest
} from './requestTypes'
import { TEST_APP_INITIALIZE } from './testUtils'
import { APTOS_NETWORK, serializeConnectData } from './utils'
// Edit an assertion and save to see HMR in action
const aptos = new Aptos() // default to devnet
const alice: Account = Account.fromPrivateKey({
  privateKey: new Ed25519PrivateKey(
    '200fb003a6e97c8ff2bd8691fa48b03e7ace251aae1b9a7365ac05d7db93bdc1' // PLEASE DO NOT USE THIS KEY
  )
})

describe('Aptos http-client tests', () => {
  let app: AppAptos
  let client: HttpClientAptos
  const clientId = getRandomId()

  beforeAll(async () => {
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
  })
  test('#connect()', async () => {
    const connectFn = vi.fn()
    app.on('userConnected', (accountInfo, networkInfo) => {
      connectFn(accountInfo, networkInfo)
    })
    const accountInfo: AccountInfo = new AccountInfo({
      address: alice.accountAddress,
      publicKey: alice.publicKey,
      ansName: undefined
    })
    const networkInfo: NetworkInfo = {
      chainId: 69,
      name: Network.MAINNET,
      url: undefined
    }
    await client.connect({
      sessionId: app.sessionId,
      metadata: serializeConnectData(accountInfo, networkInfo),
      publicKeys: [accountInfo.address.toString()]
    })
    await smartDelay()
    expect(connectFn).toHaveBeenCalledOnce()
    expect(connectFn).toHaveBeenCalledWith(accountInfo, networkInfo)
  })
  test('#resolveSignTransaction()', async () => {
    const bobAddress = '0xb0b'
    const transaction = await aptos.transaction.build.simple({
      sender: alice.accountAddress,
      data: {
        function: '0x1::coin::transfer',
        typeArguments: ['0x1::aptos_coin::AptosCoin'],
        functionArguments: [bobAddress, 100]
      }
    })
    const promiseSignTransaction = app.signTransaction(transaction)
    await smartDelay()
    // Query for request
    const pendingRequest = (
      await client.getPendingRequests({ sessionId: app.sessionId })
    )[0] as SignTransactionsAptosRequest
    expect(pendingRequest.type).toBe(ContentType.SignTransactions)
    expect(pendingRequest.transactions.length).toBe(1)
    expect(pendingRequest.execute).toBe(false)
    const senderAuthenticator = aptos.transaction.sign({
      signer: alice,
      transaction: pendingRequest.transactions[0]
    })
    // resolve
    await client.resolveSignTransaction({
      requestId: pendingRequest.requestId,
      sessionId: app.sessionId,
      signedTransactions: [senderAuthenticator]
    })

    await smartDelay()
    const signedTx = await promiseSignTransaction
    if (signedTx.status !== UserResponseStatus.APPROVED) {
      throw new Error('Transaction was not approved')
    }
    // Try to submit the transaction
    const pendingTransaction = await aptos.transaction.submit.simple({
      transaction,
      senderAuthenticator: signedTx.args
    })
    // Verify the transaction was submitted
    expect(pendingTransaction.hash).toBeDefined()
  })
  test('#resolveSignAndSubmitTransaction()', async () => {
    const bobAddress = '0xb0b'
    const transaction = await aptos.transaction.build.simple({
      sender: alice.accountAddress,
      data: {
        function: '0x1::coin::transfer',
        typeArguments: ['0x1::aptos_coin::AptosCoin'],
        functionArguments: [bobAddress, 100]
      }
    })
    const promiseSignTransaction = app.signAndSubmitTransaction(transaction)
    await smartDelay()
    // Query for request
    const pendingRequest = (
      await client.getPendingRequests({ sessionId: app.sessionId })
    )[0] as SignTransactionsAptosRequest
    expect(pendingRequest.type).toBe(ContentType.SignTransactions)
    expect(pendingRequest.transactions.length).toBe(1)
    expect(pendingRequest.execute).toBe(true)
    const submittedTx = await aptos.transaction.signAndSubmitTransaction({
      signer: alice,
      transaction: pendingRequest.transactions[0]
    })
    // resolve
    await client.resolveSignAndSubmitTransaction({
      requestId: pendingRequest.requestId,
      sessionId: app.sessionId,
      signedTransactions: [submittedTx]
    })

    await smartDelay()
    const submitted = await promiseSignTransaction
    if (submitted.status !== UserResponseStatus.APPROVED) {
      throw new Error('Transaction was not approved')
    }
    // Verify the transaction was submitted
    expect(submitted.args.hash).toBeDefined()
  })
  test('#resolveSignMessage()', async () => {
    const msgToSign: AptosSignMessageInput = {
      message: 'I love Nightly',
      nonce: 'YOLO',
      address: true
    }
    const promiseSignTransaction = app.signMessage(msgToSign)
    await smartDelay()
    // Query for request
    const pendingRequest = (
      await client.getPendingRequests({ sessionId: app.sessionId })
    )[0] as SignMessagesAptosRequest
    expect(pendingRequest.type).toBe(ContentType.SignMessages)
    expect(pendingRequest.messages.length).toBe(1)
    const payload = pendingRequest.messages[0]
    const signature = alice.sign(new Buffer(payload.message).toString('hex'))
    // resolve
    await client.resolveSignMessage({
      requestId: pendingRequest.requestId,
      sessionId: app.sessionId,
      signedMessages: [
        {
          message: payload.message,
          signature: signature,
          fullMessage: payload.message,
          nonce: payload.nonce,
          prefix: 'APTOS'
        }
      ]
    })

    await smartDelay()
    const submitted = await promiseSignTransaction
    if (submitted.status !== UserResponseStatus.APPROVED) {
      throw new Error('Transaction was not approved')
    }
  })
  test('#resolveChangeNetwork()', async () => {
    const newNetwork: AptosChangeNetworkInput = {
      name: Network.MAINNET,
      chainId: 27
    }

    const _changedNetwork = app.changeNetwork(newNetwork)
    await smartDelay()

    const pendingRequest = (
      await client.getPendingRequests({ sessionId: app.sessionId })
    )[0] as ChangeNetworkAptosRequest
    expect(pendingRequest.type).toBe(ContentType.ChangeNetwork)
    expect(pendingRequest.newNetwork.chainId).toBe(27)
    expect(pendingRequest.newNetwork.name).toBe(Network.MAINNET)

    const payload = pendingRequest.newNetwork

    await client.resolveChangeNetwork({
      requestId: pendingRequest.requestId,
      sessionId: app.sessionId,
      newNetwork: payload
    })
  })
})
