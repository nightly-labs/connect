import { MetaMaskInpageProvider, RequestArguments } from '@metamask/providers'
import { defaultSnapOrigin } from './config'
import { Snap } from './types'

export type Request = (params: RequestArguments) => Promise<unknown | null>

export type InvokeSnapParams = {
  method: string
  params?: Record<string, unknown>
}

export const getRequest = (provider: MetaMaskInpageProvider) => {
  /**
   * `provider.request` wrapper.
   *
   * @param params - The request params.
   * @param params.method - The method to call.
   * @param params.params - The method params.
   * @returns The result of the request.
   */
  const request: Request = async ({ method, params }) => {
    console.log(method, params, 'request')
    try {
      const data =
        (await provider?.request({
          method,
          params
        } as RequestArguments)) ?? null

      console.log(data, 'data')

      return data
    } catch (requestError) {
      console.log(requestError, 'req error')
      return null
    }
  }

  return request
}

export const getInvokeSnap = (provider: MetaMaskInpageProvider, snapId = defaultSnapOrigin) => {
  const request = getRequest(provider)

  /**
   * Invoke the requested Snap method.
   *
   * @param params - The invoke params.
   * @param params.method - The method name.
   * @param params.params - The method params.
   * @returns The Snap response.
   */
  const invokeSnap = async ({ method, params }: InvokeSnapParams) =>
    request({
      method: 'wallet_invokeSnap',
      params: {
        snapId,
        request: params ? { method, params } : { method }
      }
    })

  return invokeSnap
}

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

export const getFlaskStatus = async (provider: MetaMaskInpageProvider) => {
  const request = getRequest(provider)

  let isFlask = false

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

  await detectFlask()

  return isFlask
}

export const isLocalSnap = (snapId: string) => snapId.startsWith('local:')

export const shouldDisplayReconnectButton = (installedSnap: Snap | null) =>
  installedSnap && isLocalSnap(installedSnap?.id)
