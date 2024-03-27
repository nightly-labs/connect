import { HttpCloudEndpoint } from '../../../bindings/HttpCloudEndpoint'
import { HttpLoginRequest } from '../../../bindings/HttpLoginRequest'
import { HttpLoginResponse } from '../../../bindings/HttpLoginResponse'
import { HttpLoginWithGoogleRequest } from '../../../bindings/HttpLoginWithGoogleRequest'
import { HttpLoginWithGoogleResponse } from '../../../bindings/HttpLoginWithGoogleResponse'
import { HttpRegisterNewAppRequest } from '../../../bindings/HttpRegisterNewAppRequest'
import { HttpRegisterNewAppResponse } from '../../../bindings/HttpRegisterNewAppResponse'
import { HttpRegisterNewTeamRequest } from '../../../bindings/HttpRegisterNewTeamRequest'
import { HttpRegisterNewTeamResponse } from '../../../bindings/HttpRegisterNewTeamResponse'
import { HttpRegisterWithPasswordRequest } from '../../../bindings/HttpRegisterWithPasswordRequest'
import { HttpRegisterWithPasswordResponse } from '../../../bindings/HttpRegisterWithPasswordResponse'
import { DEFAULT_CLOUD_URL, EndpointType, Method } from './utils'

export interface NightlyCloudParams {
  url?: string
}
// SDK for sending cloud messages
export class NightlyCloud {
  url: string = DEFAULT_CLOUD_URL
  authToken: string | undefined = undefined
  refreshToken: string | undefined = undefined

  public constructor(params: NightlyCloudParams) {
    this.url = params.url ?? DEFAULT_CLOUD_URL
  }

  send = async (
    request: object,
    method = Method.POST,
    endpoint: HttpCloudEndpoint,
    endpointType: EndpointType
  ): Promise<any> => {
    const URL = this.url + endpointType + endpoint

    const header = {
      Accept: 'application/json',
      'Content-Type': 'application/json'
    }
    const headerAuth = {
      Accept: 'application/json',
      'Content-Type': 'application/json',
      Authorization: 'Bearer ' + this.authToken
    }
    const headers = endpointType === EndpointType.Private ? headerAuth : header

    try {
      const response: Response = await fetch(URL, {
        body: JSON.stringify(request),
        method: method,
        headers: headers
      })
      if (response.status !== 200) {
        let msg = await response.text()
        throw new Error(msg)
      }
      return await response.json()
    } catch (e) {
      let error = e as any
      throw new Error(error)
    }
  }

  registerWithPassword = async (
    request: HttpRegisterWithPasswordRequest
  ): Promise<HttpRegisterWithPasswordResponse> => {
    const response = (await this.send(
      request,
      Method.POST,
      '/register_with_password',
      EndpointType.Public
    )) as HttpRegisterWithPasswordResponse

    return response
  }

  loginWithPassword = async (request: HttpLoginRequest): Promise<HttpLoginResponse> => {
    const response = (await this.send(
      request,
      Method.POST,
      '/login_with_password',
      EndpointType.Public
    )) as HttpLoginResponse

    this.authToken = response.authToken
    this.refreshToken = response.refreshToken

    return response
  }

  loginWithGoogle = async (
    request: HttpLoginWithGoogleRequest
  ): Promise<HttpLoginWithGoogleResponse> => {
    const response = (await this.send(
      request,
      Method.POST,
      '/login_with_password',
      EndpointType.Public
    )) as HttpLoginWithGoogleResponse

    this.authToken = response.authToken
    this.refreshToken = response.refreshToken

    return response
  }

  registerNewTeam = async (
    request: HttpRegisterNewTeamRequest
  ): Promise<HttpRegisterNewTeamResponse> => {
    const response = (await this.send(
      request,
      Method.POST,
      '/register_new_team',
      EndpointType.Private
    )) as HttpRegisterNewTeamResponse

    return response
  }

  registerNewApp = async (
    request: HttpRegisterNewAppRequest
  ): Promise<HttpRegisterNewAppResponse> => {
    const response = (await this.send(
      request,
      Method.POST,
      '/register_new_app',
      EndpointType.Private
    )) as HttpRegisterNewAppResponse

    return response
  }
}
