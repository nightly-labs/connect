export enum ResponseContentType {
  SignMessages = 'SignMessagesResponse',
  SignTransactions = 'SignTransactionsResponse',
  Custom = 'CustomResponse',
  Reject = 'RejectResponse'
}

export interface SignedMessage {
  message: string
  metadata?: string
}
export interface SignMessagesResponseContent {
  type: ResponseContentType.SignMessages
  messages: SignedMessage[]
}
export interface SignedTransaction {
  transaction: string
  metadata?: string
}
export interface SignTransactionsResponseContent {
  type: ResponseContentType.SignTransactions
  transactions: SignedTransaction[]
}
export interface RejectResponseContent {
  type: ResponseContentType.Reject
  reason?: string
}
export interface CustomResponseContent {
  type: ResponseContentType.Custom
  content?: string
}
export type ResponseContent =
  | SignMessagesResponseContent
  | SignTransactionsResponseContent
  | RejectResponseContent
  | CustomResponseContent
