import { AnyRawTransaction, Network, Serializer } from '@aptos-labs/ts-sdk'
import { Snap } from './types'
import { NetworkInfo } from '@aptos-labs/wallet-standard'

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

export const NETWORK_MAP: Record<string, NetworkInfo> = {
  '1': {
    chainId: 1,
    name: Network.MAINNET,
    url: 'https://fullnode.mainnet.aptoslabs.com/v1'
  },
  '2': {
    chainId: 2,
    name: Network.TESTNET,
    url: 'https://fullnode.testnet.aptoslabs.com/v1'
  },
  '174': {
    chainId: 174,
    name: Network.DEVNET,
    url: 'https://fullnode.devnet.aptoslabs.com/v1'
  },
  '177': {
    chainId: 177,
    name: Network.CUSTOM,
    url: 'https://aptos.testnet.porto.movementlabs.xyz/v1'
  },
  '250': {
    chainId: 250,
    name: Network.CUSTOM,
    url: 'https://aptos.testnet.bardock.movementlabs.xyz/v1'
  },
  '126': {
    chainId: 126,
    name: Network.CUSTOM,
    url: 'https://mainnet.movementnetwork.xyz/v1'
  }
}
