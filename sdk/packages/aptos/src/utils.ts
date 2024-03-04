import { AppBaseInitialize, ContentType, RequestContent } from '@nightlylabs/nightly-connect-base'
import {
  AptosRequest,
  CustomAptosRequest,
  SignMessagesAptosRequest,
  SignTransactionsAptosRequest
} from './requestTypes'
import {
  AccountAddress,
  AccountAuthenticator,
  AnyRawTransaction,
  Deserializer,
  Ed25519PublicKey,
  PendingTransactionResponse,
  PublicKey,
  RawTransaction,
  Serializer
} from '@aptos-labs/ts-sdk'
import { AccountInfo, NetworkInfo } from '@aptos-labs/wallet-standard'

export type AppAptosInitialize = Omit<AppBaseInitialize, 'network'>

export const APTOS_NETWORK = 'Aptos'

// Hack to serialize bigints to strings
function replacer(key, value) {
  if (typeof value === 'bigint') {
    return {
      __type: 'bigint',
      __value: value.toString()
    }
  } else {
    return value
  }
}
// Hack to serialize bigints to strings
function reviver(key, value) {
  if (value && value.__type == 'bigint') {
    return BigInt(value.__value)
  }
  return value
}

export const serializeObject = (obj: object): string => {
  return JSON.stringify(obj, replacer)
}

export const deserializeObject = (s: string): any => {
  return JSON.parse(s, reviver)
}

interface SerializedTx {
  feePayerAddress?: string
  secondarySignerAddresses?: string
  rawTransaction: string
}
export const serializeAptosTx = (tx: AnyRawTransaction): string => {
  let feePayerAddress: string | undefined = undefined
  if (tx.feePayerAddress) {
    const serializer = new Serializer()
    serializer.serialize(tx.feePayerAddress)
    feePayerAddress = Buffer.from(serializer.toUint8Array()).toString('hex')
  }
  let secondarySignerAddresses: string | undefined = undefined
  if (tx.secondarySignerAddresses) {
    const serializer = new Serializer()
    for (const address of tx.secondarySignerAddresses) {
      serializer.serialize(address)
    }
    secondarySignerAddresses = Buffer.from(serializer.toUint8Array()).toString('hex')
  }
  let rawTransaction: string | undefined = undefined
  if (tx.rawTransaction) {
    const serializer = new Serializer()
    serializer.serialize(tx.rawTransaction)
    rawTransaction = Buffer.from(serializer.toUint8Array()).toString('hex')
  }
  const obj = {
    feePayerAddress,
    secondarySignerAddresses,
    rawTransaction
  } as SerializedTx
  return JSON.stringify(obj, replacer)
}

export const deserializeAptosTx = (s: string): AnyRawTransaction => {
  const obj = JSON.parse(s, reviver) as SerializedTx
  let feePayerAddress: AccountAddress | undefined = undefined
  if (obj.feePayerAddress) {
    feePayerAddress = AccountAddress.deserialize(
      new Deserializer(Buffer.from(obj.feePayerAddress, 'hex'))
    )
  }
  let secondarySignerAddresses: AccountAddress[] | undefined = undefined
  if (obj.secondarySignerAddresses) {
    const deserializer = new Deserializer(Buffer.from(obj.secondarySignerAddresses, 'hex'))
    secondarySignerAddresses = []
    // eslint-disable-next-line no-constant-condition
    while (true) {
      try {
        secondarySignerAddresses.push(AccountAddress.deserialize(deserializer))
      } catch (error) {
        break
      }
    }
  }
  const rawTransaction: RawTransaction = RawTransaction.deserialize(
    new Deserializer(Buffer.from(obj.rawTransaction, 'hex'))
  )
  return {
    feePayerAddress,
    secondarySignerAddresses,
    rawTransaction
  } as AnyRawTransaction
}
export const serializeAccountAuthenticator = (
  accountAuthenticator: AccountAuthenticator
): string => {
  const serializer = new Serializer()
  serializer.serialize(accountAuthenticator)
  return Buffer.from(serializer.toUint8Array()).toString('hex')
}
export const deserializeAccountAuthenticator = (s: string): AccountAuthenticator => {
  const deserializer = new Deserializer(Buffer.from(s, 'hex'))
  return AccountAuthenticator.deserialize(deserializer)
}
export const serializePendingTransactionResponse = (
  pendingTransactionResponse: PendingTransactionResponse
): string => {
  return serializeObject(pendingTransactionResponse)
}

export const deserializePendingTransactionResponse = (s: string): PendingTransactionResponse => {
  return deserializeObject(s)
}
interface SerializedConnectData {
  address: string
  publicKey: string
  networkInfo: NetworkInfo
  ansName?: string
}
export const serializeConnectData = (
  accountAuthenticator: AccountInfo,
  networkInfo: NetworkInfo
): string => {
  const serializerAddress = new Serializer()
  serializerAddress.serialize(accountAuthenticator.address)
  const address = Buffer.from(serializerAddress.toUint8Array()).toString('hex')
  const serializerPublicKey = new Serializer()
  // TODO support other public key types
  if (accountAuthenticator.publicKey instanceof Ed25519PublicKey) {
    serializerPublicKey.serialize(accountAuthenticator.publicKey)
  } else {
    // We don't support other public key types
    throw new Error('Unsupported public key type')
  }
  const publicKey = Buffer.from(serializerPublicKey.toUint8Array()).toString('hex')
  const obj: SerializedConnectData = {
    address,
    publicKey,
    ansName: accountAuthenticator.ansName,
    networkInfo: networkInfo
  }
  return serializeObject(obj)
}
export const deserializeConnectData = (
  s: string
): { accountInfo: AccountInfo; networkInfo: NetworkInfo } => {
  const obj = deserializeObject(s)
  const deserializerAddress = new Deserializer(Buffer.from(obj.address, 'hex'))
  const address = AccountAddress.deserialize(deserializerAddress)
  const deserializerPublicKey = new Deserializer(Buffer.from(obj.publicKey, 'hex'))
  const publicKey = Ed25519PublicKey.deserialize(deserializerPublicKey)
  return {
    accountInfo: {
      address,
      publicKey,
      ansName: obj.ansName
    },
    networkInfo: obj.networkInfo
  }
}
export const parseRequest = (request: RequestContent, sessionId: string): AptosRequest => {
  switch (request.content.type) {
    case ContentType.SignTransactions: {
      if (request.content.transactions.length === 0) {
        return {
          type: ContentType.SignTransactions,
          requestId: request.requestId,
          sessionId: sessionId,
          transactions: [],
          execute: true
        }
      }
      const signTransactionsRequest: SignTransactionsAptosRequest = {
        type: ContentType.SignTransactions,
        requestId: request.requestId,
        sessionId: sessionId,
        transactions: request.content.transactions.map((tx) => deserializeAptosTx(tx.transaction)),
        execute: request.content.transactions[0].metadata
          ? JSON.parse(request.content.transactions[0].metadata).execute
          : true
      }
      return signTransactionsRequest
    }
    case ContentType.SignMessages: {
      const signMessagesRequest: SignMessagesAptosRequest = {
        type: ContentType.SignMessages,
        requestId: request.requestId,
        sessionId: sessionId,
        messages: request.content.messages.map((tx) => deserializeObject(tx.message))
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
