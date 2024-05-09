import {
  HttpInviteUserToTeamRequest,
  HttpLoginRequest,
  HttpRegisterNewAppRequest,
  HttpRegisterNewTeamRequest,
  HttpRegisterWithPasswordStartRequest
} from '../../../bindings'
import { NightlyCloud } from './app'

export async function createUser(
  cloudClient: NightlyCloud
): Promise<{ userId: string; email: string }> {
  const email = randomEmail() + '@gmail.com'
  const password = 'Password123'

  const registerPayload = {
    email,
    password
  } as HttpRegisterWithPasswordStartRequest

  await cloudClient.registerWithPasswordStart(registerPayload)
  await cloudClient.registerWithPasswordFinish({ code: '123456', email })

  const loginPayload = {
    email,
    password,
    enforceIp: false
  } as HttpLoginRequest

  const userId = (await cloudClient.loginWithPassword(loginPayload)).userId.toString()

  return { userId, email }
}

export async function addUserToTeam(
  adminClient: NightlyCloud,
  userClient: NightlyCloud,
  teamId: string,
  userEmail: string
): Promise<void> {
  const invitePayload = {
    teamId: teamId,
    userEmail: userEmail
  } as HttpInviteUserToTeamRequest

  // Use team admin client to invite new user
  await adminClient.inviteUserToTeam(invitePayload)

  // Accept team invite
  const acceptPayload = {
    teamId: teamId
  } as HttpInviteUserToTeamRequest

  await userClient.acceptTeamInvite(acceptPayload)
}

export async function basicTeamSetup(
  cloudClient: NightlyCloud
): Promise<{ teamId: string; appId: string }> {
  // create team
  const registerTeamPayload = {
    personal: false,
    teamName: 'Test_Team'
  } as HttpRegisterNewTeamRequest

  const teamId = (await cloudClient.registerNewTeam(registerTeamPayload)).teamId

  // create app
  const registerAppPayload = {
    teamId: teamId,
    appName: 'Test_App',
    ackPublicKeys: [],
    whitelistedDomains: []
  } as HttpRegisterNewAppRequest

  const appId = (await cloudClient.registerNewApp(registerAppPayload)).appId

  // Return both teamId and appId in an object
  return { teamId, appId }
}

export function randomEmail(): string {
  return Math.random().toString(36).substring(7)
}
