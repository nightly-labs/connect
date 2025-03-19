import { IotaClient } from '@iota/iota-sdk/client'
import { messageWithIntent, toSerializedSignature } from '@iota/iota-sdk/cryptography'
import { type Ed25519Keypair } from '@iota/iota-sdk/keypairs/ed25519'
import { type Transaction } from '@iota/iota-sdk/transactions'
import { AppBaseInitialize, ContentType, RequestContent } from '@nightlylabs/nightly-connect-base'
import { blake2b } from '@noble/hashes/blake2b'
import {
  CustomIotaRequest,
  IotaRequest,
  SignMessagesIotaRequest,
  SignTransactionsIotaRequest
} from './requestTypes'

export type AppIotaInitialize = Omit<AppBaseInitialize, 'network'>

export const IOTA_NETWORK = 'IOTA'

type SerializedSignature = string

export type SignedTransaction = {
  bytes: string
  signature: SerializedSignature
}

export type SignedMessage = {
  messageBytes: string
  signature: SerializedSignature
}

const suiConnection = new IotaClient({ url: 'https://api.testnet.iota.cafe/' })
export const signTransactionBlock = async (tx: Transaction, account: Ed25519Keypair) => {
  const transactionBlockBytes = await tx.build({
    client: suiConnection,
    onlyTransactionKind: true
  })

  const intentMessage = messageWithIntent('TransactionData', transactionBlockBytes)
  const digest = blake2b(intentMessage, { dkLen: 32 })
  const signatureArray = await account.sign(digest)
  const signature = toSerializedSignature({
    signature: signatureArray,
    signatureScheme: 'ED25519',
    publicKey: account.getPublicKey()
  })
  return { transactionBlockBytes, signature }
}

export const parseRequest = (request: RequestContent, sessionId: string): IotaRequest => {
  switch (request.content.type) {
    case ContentType.SignTransactions: {
      const signTransactionsRequest: SignTransactionsIotaRequest = {
        type: ContentType.SignTransactions,
        requestId: request.requestId,
        sessionId: sessionId,
        transactions: request.content.transactions
      }
      return signTransactionsRequest
    }
    case ContentType.SignMessages: {
      const signMessagesRequest: SignMessagesIotaRequest = {
        type: ContentType.SignMessages,
        requestId: request.requestId,
        sessionId: sessionId,
        messages: request.content.messages
      }
      return signMessagesRequest
    }
    case ContentType.Custom: {
      const customRequest: CustomIotaRequest = {
        type: ContentType.Custom,
        content: request.content.content,
        requestId: request.requestId,
        sessionId: sessionId
      }
      return customRequest
    }
  }
}
