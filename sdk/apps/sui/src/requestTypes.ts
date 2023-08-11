import { ContentType, MessageToSign, TransactionToSign } from '@nightlylabs/nightly-connect-base'

export interface SignTransactionsSuiRequest {
  type: ContentType.SignTransactions
  requestId: string
  transactions: Array<TransactionToSign>
  sessionId: string
}
export interface SignMessagesSuiRequest {
  type: ContentType.SignMessages
  requestId: string
  messages: Array<MessageToSign>
  sessionId: string
}
export interface CustomSuiRequest {
  type: ContentType.Custom
  requestId: string
  content?: string
  sessionId: string
}
export type SuiRequest =
  | SignTransactionsSuiRequest
  | SignMessagesSuiRequest
  | CustomSuiRequest
