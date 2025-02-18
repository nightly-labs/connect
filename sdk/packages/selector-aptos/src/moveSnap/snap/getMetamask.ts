import { MetaMaskInpageProvider } from '@metamask/providers'
import { defaultSnapOrigin } from '../config'
import { GetSnapsResponse, Snap } from '../types'
import { getRequest } from './getInvoke'

export const getReadyStatus = async (provider: MetaMaskInpageProvider) => {
  const request = getRequest(provider)

  let isFlask = false
  let installedSnap: Snap | null = null

  /**
   * Detect if the version of MetaMask is Flask.
   */
  const detectFlask = async () => {
    const clientVersion = await request({
      method: 'web3_clientVersion'
    })

    const isFlaskDetected = (clientVersion as string[])?.includes('flask')

    isFlask = isFlaskDetected
  }

  /**
   * Get the Snap informations from MetaMask.
   */
  const getSnap = async () => {
    const snaps = (await request({
      method: 'wallet_getSnaps'
    })) as GetSnapsResponse

    installedSnap = snaps[defaultSnapOrigin] ?? null
  }

  await detectFlask()
  await getSnap()

  return { isFlask, installedSnap }
}
