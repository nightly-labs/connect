export enum ContentType {
  SignMessages = 'SignMessages',
  SignTransactions = 'SignTransactions',
  ChangeNetwork = 'ChangeNetwork',
  Custom = 'Custom'
}

export interface MessageToSign {
  message: string
  metadata?: string
}
export interface NetworkToChange {
  name?: string
  id: string
  url?: string
}
export interface SignMessagesContent {
  type: ContentType.SignMessages
  messages: MessageToSign[]
}
export interface TransactionToSign {
  transaction: string
  metadata?: string
}

export interface ChangeNetworkContent {
  type: ContentType.ChangeNetwork
  newNetwork: NetworkToChange
}
export interface SignTransactionsContent {
  type: ContentType.SignTransactions
  transactions: TransactionToSign[]
}

export interface CustomContent {
  type: ContentType.Custom
  content?: string
}
export type RequestInternal =
  | SignMessagesContent
  | SignTransactionsContent
  | ChangeNetworkContent
  | CustomContent

export interface RequestContent {
  requestId: string
  content: RequestInternal
}
