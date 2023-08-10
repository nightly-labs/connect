import { ContentType, RELAY_ENDPOINT, RequestContent } from '@nightlylabs/nightly-connect-base'
import { AppPolkadotInitialize } from './app'
import { SignerPayloadRaw, SignerPayloadJSON } from '@polkadot/types/types'
import {
  SignTransactionsPolkadotRequest,
  SignMessagesPolkadotRequest,
  CustomPolkadotRequest,
  PolkadotRequest
} from './requestTypes'

export const TEST_APP_INITIALIZE: AppPolkadotInitialize = {
  appMetadata: {
    additionalInfo: 'test-polkadot-additional-info',
    description: 'test-polkadot-app-description',
    icon: 'test-polkadot-app-icon',
    name: 'test-polkadot-app-name'
  },
  network: 'POLKADOT',
  persistent: false,
  persistentSessionId: undefined,
  timeout: undefined,
  url: RELAY_ENDPOINT
}
export const sleep = (ms: number) => {
  return new Promise((resolve) => setTimeout(resolve, ms))
}

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
