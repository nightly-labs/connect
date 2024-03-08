import { ContentType, MessageToSign } from '@nightlylabs/nightly-connect-base'
import { SignerPayloadJSON, SignerPayloadRaw } from '@polkadot/types/types'

export interface SignTransactionsPolkadotRequest {
  type: ContentType.SignTransactions
  requestId: string
  transactions: Array<SignerPayloadRaw | SignerPayloadJSON>
  sessionId: string
  network?: string
}
export interface SignMessagesPolkadotRequest {
  type: ContentType.SignMessages
  requestId: string
  messages: Array<MessageToSign>
  sessionId: string
  network?: string
}
export interface CustomPolkadotRequest {
  type: ContentType.Custom
  requestId: string
  content?: string
  sessionId: string
  network?: string
}
export type PolkadotRequest =
  | SignTransactionsPolkadotRequest
  | SignMessagesPolkadotRequest
  | CustomPolkadotRequest
