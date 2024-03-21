import { HttpRegisterWithPasswordRequest } from '../../../bindings/HttpRegisterWithPasswordRequest'
import { HttpRegisterWithPasswordResponse } from '../../../bindings/HttpRegisterWithPasswordResponse'
import { DEFAULT_CLOUD_URL } from './utils'

export interface NightlyCloudParams {
  endpoint?: string
}
// SDK for sending cloud messages
export class NightlyCloud {
  endpoint: string = DEFAULT_CLOUD_URL
  public constructor(params: NightlyCloudParams) {
    this.endpoint = params.endpoint ?? DEFAULT_CLOUD_URL
  }
  send = async (request: object, method = 'POST') => {
    // We don't need response
    return (
      await await fetch(this.endpoint, {
        body: JSON.stringify(request),
        method: method,
        headers: {
          Accept: 'application/json',
          'Content-Type': 'application/json'
        }
      })
    ).json()
  }

  registerWithPassword = async (
    event: HttpRegisterWithPasswordRequest
  ): Promise<HttpRegisterWithPasswordResponse> => {
    return await this.send(event)
  }
}
