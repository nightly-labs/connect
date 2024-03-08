import { AppBaseInitialize, ContentType, RequestContent } from '@nightlylabs/nightly-connect-base'
import {
  CustomSolanaRequest,
  SignMessagesSolanaRequest,
  SignTransactionsSolanaRequest,
  SolanaRequest
} from './requestTypes'
import { VersionedTransaction } from '@solana/web3.js'

export type AppSolanaInitialize = Omit<AppBaseInitialize, 'network'>

export const SOLANA_NETWORK = 'Solana'

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
