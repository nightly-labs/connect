export enum ContentType {
  SignMessages = 'SignMessages',
  SignTransactions = 'SignTransactions',
  Custom = 'Custom'
}

export interface MessageToSign {
  message: string
  metadata?: string
}
export interface SignMessagesContent {
  type: ContentType.SignMessages
  messages: MessageToSign[]
}
export interface TransactionToSign {
  transaction: string
  metadata?: string
}
export interface SignTransactionsContent {
  type: ContentType.SignTransactions
  transactions: TransactionToSign[]
}
export interface CustomContent {
  type: ContentType.Custom
  content?: string
}
export type RequestInternal = SignMessagesContent | SignTransactionsContent | CustomContent
export interface RequestContent {
  requestId: string
  content: RequestInternal
}
