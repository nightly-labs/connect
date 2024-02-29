import { ContentType, RequestContent } from '@nightlylabs/nightly-connect-base'
import { SignerPayloadRaw, SignerPayloadJSON } from '@polkadot/types/types'
import {
  SignTransactionsPolkadotRequest,
  SignMessagesPolkadotRequest,
  CustomPolkadotRequest,
  PolkadotRequest
} from './requestTypes'

export const parseRequest = (request: RequestContent, sessionId: string): PolkadotRequest => {
  switch (request.content.type) {
    case ContentType.SignTransactions: {
      const signTransactionsRequest: SignTransactionsPolkadotRequest = {
        type: ContentType.SignTransactions,
        requestId: request.requestId,
        sessionId: sessionId,
        transactions: request.content.transactions.map(
          (tx) => JSON.parse(tx.transaction) as SignerPayloadRaw | SignerPayloadJSON
        ),
        network: request.content.transactions[0]?.metadata
          ? JSON.parse(request.content.transactions[0]?.metadata).network
          : undefined
      }
      return signTransactionsRequest
    }
    case ContentType.SignMessages: {
      const signMessagesRequest: SignMessagesPolkadotRequest = {
        type: ContentType.SignMessages,
        requestId: request.requestId,
        sessionId: sessionId,
        messages: request.content.messages,
        network: request.content.messages[0]?.metadata
          ? JSON.parse(request.content.messages[0]?.metadata).network
          : undefined
      }
      return signMessagesRequest
    }
    case ContentType.Custom: {
      const customRequest: CustomPolkadotRequest = {
        type: ContentType.Custom,
        content: request.content.content,
        requestId: request.requestId,
        sessionId: sessionId
      }
      return customRequest
    }
  }
}
