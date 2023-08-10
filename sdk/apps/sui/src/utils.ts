import {
  JsonRpcProvider,
  Connection,
  TransactionBlock,
  messageWithIntent,
  IntentScope,
  toSerializedSignature,
  Ed25519Keypair
} from '@mysten/sui.js'
import { blake2b } from '@noble/hashes/blake2b'
import { AppBaseInitialize, ContentType, RELAY_ENDPOINT, RequestContent } from '@nightlylabs/nightly-connect-base'
import { CustomSuiRequest, SignMessagesSuiRequest, SignTransactionsSuiRequest, SuiRequest } from './requestTypes'

export type AppSuiInitialize = Omit<AppBaseInitialize, 'network'>

export const SUI_NETWORK = 'Sui'

export const TEST_APP_INITIALIZE: AppSuiInitialize = {
  appMetadata: {
    additionalInfo: 'test-sui-additional-info',
    description: 'test-sui-app-description',
    icon: 'test-sui-app-icon',
    name: 'test-sui-app-name'
  },
  persistent: false,
  persistentSessionId: undefined,
  timeout: undefined,
  url: RELAY_ENDPOINT
}
export function sleep(ms: number) {
  return new Promise((resolve) => setTimeout(resolve, ms))
}

const suiConnection = new JsonRpcProvider(
  new Connection({ fullnode: 'https://fullnode.testnet.sui.io/' })
)
export const signTransactionBlock = async (tx: TransactionBlock, account: Ed25519Keypair) => {
  const transactionBlockBytes = await tx.build({
    provider: suiConnection,
    onlyTransactionKind: true
  })

  const intentMessage = messageWithIntent(IntentScope.TransactionData, transactionBlockBytes)
  const digest = blake2b(intentMessage, { dkLen: 32 })
  const signatureArray = account.signData(digest)
  const signature = toSerializedSignature({
    signature: signatureArray,
    signatureScheme: 'ED25519',
    pubKey: account.getPublicKey()
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