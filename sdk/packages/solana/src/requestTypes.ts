import { ContentType, MessageToSign } from '@nightlylabs/nightly-connect-base'
import { VersionedTransaction } from '@solana/web3.js'

export interface SignTransactionsSolanaRequest {
  type: ContentType.SignTransactions
  requestId: string
  transactions: Array<VersionedTransaction>
  sessionId: string
}
export interface SignMessagesSolanaRequest {
  type: ContentType.SignMessages
  requestId: string
  messages: Array<MessageToSign>
  sessionId: string
}
export interface CustomSolanaRequest {
  type: ContentType.Custom
  requestId: string
  content?: string
  sessionId: string
}
export type SolanaRequest =
  | SignTransactionsSolanaRequest
  | SignMessagesSolanaRequest
  | CustomSolanaRequest
