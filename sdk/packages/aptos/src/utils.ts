import {
  AccountAddress,
  AccountAuthenticator,
  AnyRawTransaction,
  Deserializer,
  PendingTransactionResponse,
  RawTransaction,
  Serializer
} from '@aptos-labs/ts-sdk'
import { AccountInfo, NetworkInfo } from '@aptos-labs/wallet-standard'
import { AppBaseInitialize, ContentType, RequestContent } from '@nightlylabs/nightly-connect-base'
import {
  AptosRequest,
  CustomAptosRequest,
  SignMessagesAptosRequest,
  SignTransactionsAptosRequest
} from './requestTypes'

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
  accountInfo: string
  networkInfo: NetworkInfo
}
export const serializeConnectData = (
  accountInfo: AccountInfo,
  networkInfo: NetworkInfo
): string => {
  const serializerAccountInfo = new Serializer()
  serializerAccountInfo.serialize(accountInfo)

  const obj: SerializedConnectData = {
    accountInfo: Buffer.from(serializerAccountInfo.toUint8Array()).toString('hex'),
    networkInfo: networkInfo
  }
  return serializeObject(obj)
}
export const deserializeConnectData = (
  s: string
): { accountInfo: AccountInfo; networkInfo: NetworkInfo } => {
  const obj = deserializeObject(s)
  const deserializerAccountInfo = new Deserializer(Buffer.from(obj.accountInfo, 'hex'))
  const accountInfo = AccountInfo.deserialize(deserializerAccountInfo)
  return {
    accountInfo: accountInfo,
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
    case ContentType.ChangeNetwork: {
      const changeNetworkRequest: CustomAptosRequest = {
        type: ContentType.Custom,
        content: request.content.newNetwork.chainId.toString(),
        requestId: request.requestId,
        sessionId: sessionId
      }
      return changeNetworkRequest
    }
  }
}
