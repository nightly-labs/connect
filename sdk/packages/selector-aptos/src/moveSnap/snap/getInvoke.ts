import { MetaMaskInpageProvider, RequestArguments } from '@metamask/providers'
import { defaultSnapOrigin } from '../config'

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
    const data =
      (await provider?.request({
        method,
        params
      } as RequestArguments)) ?? null

    return data
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
