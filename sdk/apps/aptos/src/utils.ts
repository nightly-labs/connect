import { AppBaseInitialize, ContentType, RequestContent } from '@nightlylabs/nightly-connect-base'
import {
  CustomAptosRequest,
  SignMessagesAptosRequest,
  SignTransactionsAptosRequest,
  AptosRequest
} from './requestTypes'
import { SignMessageResponse } from '@aptos-labs/wallet-adapter-core'

export type AppAptosInitialize = Omit<AppBaseInitialize, 'network'>

export const APTOS_NETWORK = 'Aptos'

export const parseRequest = (request: RequestContent, sessionId: string): AptosRequest => {
  switch (request.content.type) {
    case ContentType.SignTransactions: {
      const signTransactionsRequest: SignTransactionsAptosRequest = {
        type: ContentType.SignTransactions,
        requestId: request.requestId,
        sessionId: sessionId,
        transactions: request.content.transactions.map((tx) => {
          return {
            options: tx.metadata ? JSON.parse(tx.metadata) : { submit: true },
            transaction: JSON.parse(tx.transaction)
          }
        })
      }
      return signTransactionsRequest
    }
    case ContentType.SignMessages: {
      const signMessagesRequest: SignMessagesAptosRequest = {
        type: ContentType.SignMessages,
        requestId: request.requestId,
        sessionId: sessionId,
        messages: request.content.messages.map((msg) => JSON.parse(msg.message))
      }
      return signMessagesRequest
    }
    case ContentType.Custom: {
      const customRequest: CustomAptosRequest = {
        type: ContentType.Custom,
        content: request.content.content,
        requestId: request.requestId,
        sessionId: sessionId
      }
      return customRequest
    }
  }
}

export const serializeSignMessageResponse = (response: SignMessageResponse) => {
  const data = {
    ...response,
    bitmap: response.bitmap ? Buffer.from(response.bitmap).toString('hex') : undefined
  }
  return JSON.stringify(data)
}
export const deserializeSignMessageResponse = (data: string): SignMessageResponse => {
  const response = JSON.parse(data)
  const bitmap = response.bitmap ? Buffer.from(response.bitmap, 'hex') : undefined
  return {
    ...response,
    bitmap
  }
}

import { sha256 } from 'js-sha256'
import base58 from 'bs58'

export class AptosPublicKey {
  private readonly hexString: string

  static default() {
    return new AptosPublicKey('0'.repeat(64))
  }

  address() {
    const hash = sha256.create()
    hash.update(Buffer.from(this.asPureHex(), 'hex'))
    hash.update('\x00')
    return '0x' + hash.hex()
  }

  asUint8Array() {
    return new Uint8Array(Buffer.from(this.asPureHex(), 'hex'))
  }
  static fromBase58(base58string: string) {
    const bytes = Buffer.from(base58.decode(base58string))
    const hexString = bytes.toString('hex')
    return new AptosPublicKey(hexString)
  }
  asString() {
    return this.hexString
  }

  asPureHex() {
    return this.hexString.substr(2)
  }

  constructor(hexString: string) {
    if (hexString.startsWith('0x')) {
      this.hexString = hexString
    } else {
      this.hexString = `0x${hexString}`
    }
  }
}
