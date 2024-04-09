import {
  HttpAcceptTeamInviteRequest,
  HttpAcceptTeamInviteResponse,
  HttpCancelTeamUserInviteRequest,
  HttpCancelTeamUserInviteResponse,
  HttpCancelUserTeamInviteRequest,
  HttpCancelUserTeamInviteResponse,
  HttpChangeUsersPrivilegesRequest,
  HttpChangeUsersPrivilegesResponse,
  HttpCloudEndpoint,
  HttpGetAppEventsRequest,
  HttpGetAppEventsResponse,
  HttpGetTeamMetadataResponse,
  HttpGetTeamUserInvitesRequest,
  HttpGetTeamUserInvitesResponse,
  HttpGetUserJoinedTeamsResponse,
  HttpGetUserTeamInvitesResponse,
  HttpInviteUserToTeamRequest,
  HttpInviteUserToTeamResponse,
  HttpLoginRequest,
  HttpLoginResponse,
  HttpLoginWithGoogleRequest,
  HttpLoginWithGoogleResponse,
  HttpLoginWithPasskeyFinishResponse,
  HttpLoginWithPasskeyStartRequest,
  HttpRegisterNewAppRequest,
  HttpRegisterNewAppResponse,
  HttpRegisterNewTeamRequest,
  HttpRegisterNewTeamResponse,
  HttpRegisterWithPasskeyStartRequest,
  HttpRegisterWithPasswordFinishRequest,
  HttpRegisterWithPasswordFinishResponse,
  HttpRegisterWithPasswordStartRequest,
  HttpRegisterWithPasswordStartResponse,
  HttpRemoveUserFromTeamRequest,
  HttpRemoveUserFromTeamResponse,
  HttpRemoveWhitelistedDomainRequest,
  HttpRemoveWhitelistedDomainResponse,
  HttpResetPasskeyFinishResponse,
  HttpResetPasskeyStartRequest,
  HttpResetPasswordFinishRequest,
  HttpResetPasswordFinishResponse,
  HttpResetPasswordStartRequest,
  HttpResetPasswordStartResponse,
  HttpUserMetadataResponse,
  HttpVerifyDomainFinishRequest,
  HttpVerifyDomainFinishResponse,
  HttpVerifyDomainStartRequest,
  HttpVerifyDomainStartResponse
} from '../../../bindings'
import {
  HttpDeletePasskeyRequest,
  HttpGetPasskeyChallengeResponse,
  HttpLoginWithPasskeyFinishRequest,
  HttpLoginWithPasskeyStartResponse,
  HttpRegisterWithPasskeyFinishRequest,
  HttpResetPasskeyFinishRequest,
  HttpResetPasskeyStartResponse
} from './passkeyTypes'
import { DEFAULT_CLOUD_URL, EndpointType, Method } from './utils'
import { fetch } from 'cross-fetch'

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
        const msg = await response.text()
        throw new Error(msg)
      }
      return await response.json()
    } catch (e) {
      const error = e as any
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
        const msg = await response.text()
        throw new Error(msg)
      }
      return await response.json()
    } catch (e) {
      const error = e as any
      throw new Error(error)
    }
  }

  ///////////////////////////////////////////////////// Register

  registerWithPasswordStart = async (
    request: HttpRegisterWithPasswordStartRequest
  ): Promise<HttpRegisterWithPasswordStartResponse> => {
    const response = (await this.sendPostJson(
      '/register_with_password_start',
      EndpointType.Public,
      request
    )) as HttpRegisterWithPasswordStartResponse

    return response
  }

  registerWithPasswordFinish = async (
    request: HttpRegisterWithPasswordFinishRequest
  ): Promise<HttpRegisterWithPasswordFinishResponse> => {
    const response = (await this.sendPostJson(
      '/register_with_password_finish',
      EndpointType.Public,
      request
    )) as HttpRegisterWithPasswordFinishResponse

    return response
  }

  registerWithPasskeyStart = async (
    request: HttpRegisterWithPasskeyStartRequest
  ): Promise<HttpRegisterWithPasswordStartResponse> => {
    const response = (await this.sendPostJson(
      '/register_with_passkey_start',
      EndpointType.Public,
      request
    )) as HttpRegisterWithPasswordStartResponse

    return response
  }

  registerWithPasskeyFinish = async (
    request: HttpRegisterWithPasskeyFinishRequest
  ): Promise<HttpRegisterWithPasswordFinishResponse> => {
    const response = (await this.sendPostJson(
      '/register_with_passkey_finish',
      EndpointType.Public,
      request
    )) as HttpRegisterWithPasswordFinishResponse

    return response
  }

  ///////////////////////////////////////////////////// Login

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

  loginWithPasskeyStart = async (
    request: HttpLoginWithPasskeyStartRequest
  ): Promise<HttpLoginWithPasskeyStartResponse> => {
    const response = (await this.sendPostJson(
      '/login_with_passkey_start',
      EndpointType.Public,
      request
    )) as HttpLoginWithPasskeyStartResponse

    return response
  }

  loginWithPasskeyFinish = async (
    request: HttpLoginWithPasskeyFinishRequest
  ): Promise<HttpLoginWithPasskeyFinishResponse> => {
    const response = (await this.sendPostJson(
      '/login_with_passkey_finish',
      EndpointType.Public,
      request
    )) as HttpLoginWithPasskeyFinishResponse

    this.authToken = response.authToken
    this.refreshToken = response.refreshToken

    return response
  }

  ///////////////////////////////////////////////////// Credentials

  resetPasswordStart = async (
    request: HttpResetPasswordStartRequest
  ): Promise<HttpResetPasswordStartResponse> => {
    const response = (await this.sendPostJson(
      '/reset_password_start',
      EndpointType.Public,
      request
    )) as HttpResetPasswordStartResponse

    return response
  }

  resetPasswordFinish = async (
    request: HttpResetPasswordFinishRequest
  ): Promise<HttpResetPasswordFinishResponse> => {
    const response = (await this.sendPostJson(
      '/reset_password_finish',
      EndpointType.Public,
      request
    )) as HttpResetPasswordFinishResponse

    return response
  }

  resetPasskeyStart = async (
    request: HttpResetPasskeyStartRequest
  ): Promise<HttpResetPasskeyStartResponse> => {
    const response = (await this.sendPostJson(
      '/reset_passkey_start',
      EndpointType.Public,
      request
    )) as HttpResetPasskeyStartResponse

    return response
  }

  resetPasskeyFinish = async (
    request: HttpResetPasskeyFinishRequest
  ): Promise<HttpResetPasskeyFinishResponse> => {
    const response = (await this.sendPostJson(
      '/reset_passkey_finish',
      EndpointType.Public,
      request
    )) as HttpResetPasskeyFinishResponse

    return response
  }

  deletePasskey = async (request: HttpDeletePasskeyRequest): Promise<void> => {
    await this.sendPostJson('/reset_passkey_finish', EndpointType.Public, request)
  }

  ///////////////////////////////////////////////////// Teams actions

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

  changeUserPrivileges = async (
    request: HttpChangeUsersPrivilegesRequest
  ): Promise<HttpChangeUsersPrivilegesResponse> => {
    const response = (await this.sendPostJson(
      '/change_user_privileges',
      EndpointType.Private,
      request
    )) as HttpChangeUsersPrivilegesResponse

    return response
  }

  ///////////////////////////////////////////////////// App actions

  verifyDomainStart = async (
    request: HttpVerifyDomainStartRequest
  ): Promise<HttpVerifyDomainStartResponse> => {
    const response = (await this.sendPostJson(
      '/verify_domain_start',
      EndpointType.Private,
      request
    )) as HttpVerifyDomainStartResponse

    return response
  }

  verifyDomainFinish = async (
    request: HttpVerifyDomainFinishRequest
  ): Promise<HttpVerifyDomainFinishResponse> => {
    const response = (await this.sendPostJson(
      '/verify_domain_finish',
      EndpointType.Private,
      request
    )) as HttpVerifyDomainFinishResponse

    return response
  }

  removeDomain = async (
    request: HttpRemoveWhitelistedDomainRequest
  ): Promise<HttpRemoveWhitelistedDomainResponse> => {
    const response = (await this.sendPostJson(
      '/remove_whitelisted_domain',
      EndpointType.Private,
      request
    )) as HttpRemoveWhitelistedDomainResponse

    return response
  }

  ///////////////////////////////////////////////////// Getters

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

  getAppEvents = async (request: HttpGetAppEventsRequest): Promise<HttpGetAppEventsResponse> => {
    const response = (await this.sendGetJson(
      '/get_app_events',
      EndpointType.Private,
      request
    )) as HttpGetAppEventsResponse

    return response
  }

  getPasskeyChallenge = async (): Promise<HttpGetPasskeyChallengeResponse> => {
    const response = (await this.sendGetJson(
      '/get_passkey_challenge',
      EndpointType.Private
    )) as HttpGetPasskeyChallengeResponse

    return response
  }

  getUserMetadata = async (): Promise<HttpUserMetadataResponse> => {
    const response = (await this.sendGetJson(
      '/get_user_metadata',
      EndpointType.Private
    )) as HttpUserMetadataResponse

    return response
  }

  getTeamMetadata = async (): Promise<HttpGetTeamMetadataResponse> => {
    const response = (await this.sendGetJson(
      '/get_team_metadata',
      EndpointType.Private
    )) as HttpGetTeamMetadataResponse

    return response
  }
}
