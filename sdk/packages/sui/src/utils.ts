import { SuiClient } from '@mysten/sui/client'
import { messageWithIntent, toSerializedSignature } from '@mysten/sui/cryptography'
import { type Ed25519Keypair } from '@mysten/sui/keypairs/ed25519'
import { type Transaction } from '@mysten/sui/transactions'
import { AppBaseInitialize, ContentType, RequestContent } from '@nightlylabs/nightly-connect-base'
import { blake2b } from '@noble/hashes/blake2b'
import {
  CustomSuiRequest,
  SignMessagesSuiRequest,
  SignTransactionsSuiRequest,
  SuiRequest
} from './requestTypes'

export type AppSuiInitialize = Omit<AppBaseInitialize, 'network'>

export const SUI_NETWORK = 'Sui'

type SerializedSignature = string

export type SignedTransaction = {
  transactionBlockBytes: string
  signature: SerializedSignature
}

export type SignedMessage = {
  messageBytes: string
  signature: SerializedSignature
}

const suiConnection = new SuiClient({ url: 'https://fullnode.testnet.sui.io/' })
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

export const parseRequest = (request: RequestContent, sessionId: string): SuiRequest => {
  switch (request.content.type) {
    case ContentType.SignTransactions: {
      const signTransactionsRequest: SignTransactionsSuiRequest = {
        type: ContentType.SignTransactions,
        requestId: request.requestId,
        sessionId: sessionId,
        transactions: request.content.transactions
      }
      return signTransactionsRequest
    }
    case ContentType.SignMessages: {
      const signMessagesRequest: SignMessagesSuiRequest = {
        type: ContentType.SignMessages,
        requestId: request.requestId,
        sessionId: sessionId,
        messages: request.content.messages
      }
      return signMessagesRequest
    }
    case ContentType.Custom: {
      const customRequest: CustomSuiRequest = {
        type: ContentType.Custom,
        content: request.content.content,
        requestId: request.requestId,
        sessionId: sessionId
      }
      return customRequest
    }
  }
}
