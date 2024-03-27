import { HttpAcceptTeamInviteRequest } from '../../../bindings/HttpAcceptTeamInviteRequest'
import { HttpAcceptTeamInviteResponse } from '../../../bindings/HttpAcceptTeamInviteResponse'
import { HttpCancelTeamUserInviteRequest } from '../../../bindings/HttpCancelTeamUserInviteRequest'
import { HttpCancelTeamUserInviteResponse } from '../../../bindings/HttpCancelTeamUserInviteResponse'
import { HttpCancelUserTeamInviteRequest } from '../../../bindings/HttpCancelUserTeamInviteRequest'
import { HttpCancelUserTeamInviteResponse } from '../../../bindings/HttpCancelUserTeamInviteResponse'
import { HttpCloudEndpoint } from '../../../bindings/HttpCloudEndpoint'
import { HttpGetTeamUserInvitesRequest } from '../../../bindings/HttpGetTeamUserInvitesRequest'
import { HttpGetTeamUserInvitesResponse } from '../../../bindings/HttpGetTeamUserInvitesResponse'
import { HttpGetUserJoinedTeamsResponse } from '../../../bindings/HttpGetUserJoinedTeamsResponse'
import { HttpGetUserTeamInvitesResponse } from '../../../bindings/HttpGetUserTeamInvitesResponse'
import { HttpInviteUserToTeamRequest } from '../../../bindings/HttpInviteUserToTeamRequest'
import { HttpInviteUserToTeamResponse } from '../../../bindings/HttpInviteUserToTeamResponse'
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
import { HttpRemoveUserFromTeamRequest } from '../../../bindings/HttpRemoveUserFromTeamRequest'
import { HttpRemoveUserFromTeamResponse } from '../../../bindings/HttpRemoveUserFromTeamResponse'
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

  sendPostJson = async (
    endpoint: HttpCloudEndpoint,
    endpointType: EndpointType,
    request: object
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
        method: Method.POST,
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

  sendGetJson = async (
    endpoint: HttpCloudEndpoint,
    endpointType: EndpointType,
    message?: { [key: string]: any }
  ): Promise<any> => {
    let URL = this.url + endpointType + endpoint + '?'

    if (message) {
      for (const key of Object.keys(message)) {
        if (Array.isArray(message[key])) {
          message[key].forEach((value: any) => {
            URL += key + '=' + value + '&'
          })
        } else if (message[key]) {
          // @ts-ignore
          URL += key + '=' + message[key] + '&'
        }
      }
      // remove last &
      URL = URL.slice(0, -1)
    }

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
        method: Method.GET,
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
    const response = (await this.sendPostJson(
      '/register_with_password',
      EndpointType.Public,
      request
    )) as HttpRegisterWithPasswordResponse

    return response
  }

  loginWithPassword = async (request: HttpLoginRequest): Promise<HttpLoginResponse> => {
    const response = (await this.sendPostJson(
      '/login_with_password',
      EndpointType.Public,
      request
    )) as HttpLoginResponse

    this.authToken = response.authToken
    this.refreshToken = response.refreshToken

    return response
  }

  loginWithGoogle = async (
    request: HttpLoginWithGoogleRequest
  ): Promise<HttpLoginWithGoogleResponse> => {
    const response = (await this.sendPostJson(
      '/login_with_google',
      EndpointType.Public,
      request
    )) as HttpLoginWithGoogleResponse

    this.authToken = response.authToken
    this.refreshToken = response.refreshToken

    return response
  }

  registerNewTeam = async (
    request: HttpRegisterNewTeamRequest
  ): Promise<HttpRegisterNewTeamResponse> => {
    const response = (await this.sendPostJson(
      '/register_new_team',
      EndpointType.Private,
      request
    )) as HttpRegisterNewTeamResponse

    return response
  }

  registerNewApp = async (
    request: HttpRegisterNewAppRequest
  ): Promise<HttpRegisterNewAppResponse> => {
    const response = (await this.sendPostJson(
      '/register_new_app',
      EndpointType.Private,
      request
    )) as HttpRegisterNewAppResponse

    return response
  }

  inviteUserToTeam = async (
    request: HttpInviteUserToTeamRequest
  ): Promise<HttpInviteUserToTeamResponse> => {
    const response = (await this.sendPostJson(
      '/invite_user_to_team',
      EndpointType.Private,
      request
    )) as HttpInviteUserToTeamResponse

    return response
  }

  acceptTeamInvite = async (
    request: HttpAcceptTeamInviteRequest
  ): Promise<HttpAcceptTeamInviteResponse> => {
    const response = (await this.sendPostJson(
      '/accept_team_invite',
      EndpointType.Private,
      request
    )) as HttpAcceptTeamInviteResponse

    return response
  }

  removeUserFromTeam = async (
    request: HttpRemoveUserFromTeamRequest
  ): Promise<HttpRemoveUserFromTeamResponse> => {
    const response = (await this.sendPostJson(
      '/remove_user_from_team',
      EndpointType.Private,
      request
    )) as HttpAcceptTeamInviteResponse

    return response
  }

  cancelTeamUserInvite = async (
    request: HttpCancelTeamUserInviteRequest
  ): Promise<HttpCancelTeamUserInviteResponse> => {
    const response = (await this.sendPostJson(
      '/cancel_team_user_invite',
      EndpointType.Private,
      request
    )) as HttpCancelTeamUserInviteResponse

    return response
  }

  cancelUserTeamInvite = async (
    request: HttpCancelUserTeamInviteRequest
  ): Promise<HttpCancelUserTeamInviteResponse> => {
    const response = (await this.sendPostJson(
      '/cancel_user_team_invite',
      EndpointType.Private,
      request
    )) as HttpCancelTeamUserInviteResponse

    return response
  }

  ////////////////////////// GETTERS

  getUserTeamInvites = async (): Promise<HttpGetUserTeamInvitesResponse> => {
    const response = (await this.sendGetJson(
      '/get_user_team_invites',
      EndpointType.Private
    )) as HttpGetUserTeamInvitesResponse

    return response
  }

  getTeamUserInvites = async (
    request: HttpGetTeamUserInvitesRequest
  ): Promise<HttpGetUserTeamInvitesResponse> => {
    const response = (await this.sendGetJson(
      '/get_team_user_invites',
      EndpointType.Private,
      request
    )) as HttpGetTeamUserInvitesResponse

    return response
  }

  getUserJoinedTeams = async (): Promise<HttpGetUserJoinedTeamsResponse> => {
    const response = (await this.sendGetJson(
      '/get_user_joined_teams',
      EndpointType.Private
    )) as HttpGetUserJoinedTeamsResponse

    return response
  }
}
