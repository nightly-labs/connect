import { MetaMaskInpageProvider } from '@metamask/providers'
import { defaultSnapOrigin } from '../config'
import { Snap } from '../types'
import { getRequest } from './getInvoke'

export const getRequestSnaps = (
  provider: MetaMaskInpageProvider,
  snapId = defaultSnapOrigin,
  version?: string
) => {
  const request = getRequest(provider)
  let installedSnap: Snap | null = null

  /**
   * Request the Snap.
   */
  const requestSnap = async () => {
    const snaps = (await request({
      method: 'wallet_requestSnaps',
      params: {
        [snapId]: version ? { version } : {}
      }
    })) as Record<string, Snap>
    installedSnap = snaps?.[snapId] ?? null
  }

  return { installedSnap, requestSnap }
}
