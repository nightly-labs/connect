import type { SignMessagePayload, Types } from '@aptos-labs/wallet-adapter-core'
import type { ContentType } from '@nightlylabs/nightly-connect-base'

export interface SignTransactionsAptosRequest {
  type: ContentType.SignTransactions
  requestId: string
  transactions: Array<{
    transaction: Types.TransactionPayload
    options: unknown & { submit: true }
  }>
  sessionId: string
}
export interface SignMessagesAptosRequest {
  type: ContentType.SignMessages
  requestId: string
  messages: Array<SignMessagePayload>
  sessionId: string
}
export interface CustomAptosRequest {
  type: ContentType.Custom
  requestId: string
  content?: string
  sessionId: string
}
export type AptosRequest =
  | SignTransactionsAptosRequest
  | SignMessagesAptosRequest
  | CustomAptosRequest
