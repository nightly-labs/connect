import { ContentType, MessageToSign, TransactionToSign } from '@nightlylabs/nightly-connect-base'

export interface SignTransactionsIotaRequest {
  type: ContentType.SignTransactions
  requestId: string
  transactions: Array<TransactionToSign>
  sessionId: string
}
export interface SignMessagesIotaRequest {
  type: ContentType.SignMessages
  requestId: string
  messages: Array<MessageToSign>
  sessionId: string
}
export interface CustomIotaRequest {
  type: ContentType.Custom
  requestId: string
  content?: string
  sessionId: string
}
export type IotaRequest = SignTransactionsIotaRequest | SignMessagesIotaRequest | CustomIotaRequest
