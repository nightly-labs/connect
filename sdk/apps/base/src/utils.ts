import { v4 as uuidv4 } from 'uuid'
import { getStorage, ILocalStorage } from 'isomorphic-localstorage'
import { WalletMetadata } from '../../../bindings/WalletMetadata'
import { fetch } from 'cross-fetch'

export const getRandomId = () => uuidv4()

export const RELAY_ENDPOINT = 'https://nc2.nightly.app'

export const getWalletsMetadata = async (
  url?: string,
  network?: string
): Promise<WalletMetadata[]> => {
  const endpoint = url ?? RELAY_ENDPOINT + '/get_wallets_metadata'
  const result = (await (
    await fetch(endpoint, {
      method: 'GET',
      headers: {
        Accept: 'application/json',
        'Content-Type': 'application/json'
      }
    })
  ).json()) as WalletMetadata[]
  if (network) {
    return result.filter((walletMetadata) => walletMetadata.chains.includes(network))
  } else {
    return result
  }
}

let _localStorage: ILocalStorage | null = null
export const getLocalStorage = () => {
  if (_localStorage === null) {
    _localStorage = getStorage('./.nightly-connect-session')
  }

  return _localStorage
}

export const getSessionIdLocalStorageKey = (network: string) =>
  'NIGHTLY_CONNECT_SESSION_ID_' + network
