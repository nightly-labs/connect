import {
  AppBaseInitialize,
  ContentType,
  RELAY_ENDPOINT,
  RequestContent
} from '@nightlylabs/nightly-connect-base'
import {
  CustomSolanaRequest,
  SignMessagesSolanaRequest,
  SignTransactionsSolanaRequest,
  SolanaRequest
} from './requestTypes'
import { VersionedTransaction } from '@solana/web3.js'

export type AppSolanaInitialize = Omit<AppBaseInitialize, 'network'>

export const SOLANA_NETWORK = 'Solana'

export const TEST_APP_INITIALIZE: AppSolanaInitialize = {
  appMetadata: {
    additionalInfo: 'test-solana-additional-info',
    description: 'test-solana-app-description',
    icon: 'test-solana-app-icon',
    name: 'test-solana-app-name'
  },
  persistent: false,
  persistentSessionId: undefined,
  timeout: undefined,
  url: RELAY_ENDPOINT
}
export function sleep(ms: number) {
  return new Promise((resolve) => setTimeout(resolve, ms))
}

export const parseRequest = (request: RequestContent, sessionId: string): SolanaRequest => {
  switch (request.content.type) {
    case ContentType.SignTransactions: {
      const signTransactionsRequest: SignTransactionsSolanaRequest = {
        type: ContentType.SignTransactions,
        requestId: request.requestId,
        sessionId: sessionId,
        transactions: request.content.transactions.map((tx) =>
          VersionedTransaction.deserialize(Buffer.from(tx.transaction, 'hex'))
        )
      }
      return signTransactionsRequest
    }
    case ContentType.SignMessages: {
      const signMessagesRequest: SignMessagesSolanaRequest = {
        type: ContentType.SignMessages,
        requestId: request.requestId,
        sessionId: sessionId,
        messages: request.content.messages
      }
      return signMessagesRequest
    }
    case ContentType.Custom: {
      const customRequest: CustomSolanaRequest = {
        type: ContentType.Custom,
        content: request.content.content,
        requestId: request.requestId,
        sessionId: sessionId
      }
      return customRequest
    }
  }
}
