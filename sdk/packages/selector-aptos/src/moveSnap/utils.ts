import { AnyRawTransaction, Serializer } from '@aptos-labs/ts-sdk'
import { Snap } from './types'

export const isLocalSnap = (snapId: string) => snapId.startsWith('local:')

export const shouldDisplayReconnectButton = (installedSnap: Snap | null) =>
  installedSnap && isLocalSnap(installedSnap?.id)

export interface IAnyRawTransactionStringified {
  rawTransaction: string
  secondarySignerAddresses?: undefined | string[]
  feePayerAddress?: string
}

export const encodeAptosTransaction = (
  aptosTx: AnyRawTransaction
): IAnyRawTransactionStringified => {
  const buffer = new Serializer()
  aptosTx.rawTransaction.serialize(buffer)
  return {
    rawTransaction: Buffer.from(buffer.toUint8Array()).toString('hex'),
    secondarySignerAddresses: aptosTx.secondarySignerAddresses?.map((e) => e.toString()),
    feePayerAddress: aptosTx.feePayerAddress?.toString()
  }
}
