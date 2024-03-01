import { AnyRawTransaction } from '@aptos-labs/ts-sdk'
import { AptosSignMessageInput } from '@aptos-labs/wallet-standard'
import { ContentType } from '@nightlylabs/nightly-connect-base'

export interface SignTransactionsAptosRequest {
  type: ContentType.SignTransactions
  requestId: string
  transactions: Array<AnyRawTransaction>
  execute: boolean
  sessionId: string
}
export interface SignMessagesAptosRequest {
  type: ContentType.SignMessages
  requestId: string
  messages: Array<AptosSignMessageInput>
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
