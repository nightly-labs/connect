import { NightlyCloud } from '@nightlylabs/nightly-cloud'
import {
  HttpLoginRequest,
  HttpRegisterNewTeamRequest,
  HttpRegisterNewAppRequest,
  HttpRegisterWithPasswordStartRequest,
  HttpVerifyDomainStartRequest
} from '../../../bindings'
import { NightlyAnalytics } from './app'
import { fetch } from 'cross-fetch'

export async function createUser(
  cloudClient: NightlyCloud
): Promise<{ userId: string; email: string }> {
  const email = randomEmail() + '@gmail.com'
  const password = 'Password123'

  const registerPayload = {
    email,
    password,
    device: 'device',
    browser: 'browser'
  } as HttpRegisterWithPasswordStartRequest

  await cloudClient.registerWithPasswordStart(registerPayload)

  const verifyResponse = await cloudClient.verifyCode({
    email,
    code: '123456',
    action: 'registerPassword'
  })

  await cloudClient.registerWithPasswordFinish({
    authCode: verifyResponse.verificationCode,
    email,
    newPassword: password
  })

  const loginPayload = {
    email,
    password,
    enforceIp: false
  } as HttpLoginRequest

  const userId = (await cloudClient.loginWithPassword(loginPayload)).userId.toString()

  return { userId, email }
}

export async function setupTestTeam(
  cloudClient: NightlyCloud
): Promise<{ teamId: string; appId: string }> {
  // create user
  await createUser(cloudClient)

  // create basic team setup
  return await basicTeamSetup(cloudClient)
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
    whitelistedDomains: ['localhost']
  } as HttpRegisterNewAppRequest

  const appId = (await cloudClient.registerNewApp(registerAppPayload)).appId

  // Return both teamId and appId in an object
  return { teamId, appId }
}

export function randomEmail(): string {
  return Math.random().toString(36).substring(7)
}

export function setupAnalytics(
  origin: string,
  network: string,
  endpoint: string,
  appId: string,
  sessionId: string
): NightlyAnalytics {
  const analytics = new NightlyAnalytics({
    sessionId: sessionId,
    network: network,
    endpoint: endpoint,
    appId: appId
  })

  // Override sendEvent within the setup
  analytics.sendEvent = async function (request, method = 'POST') {
    return await fetch(this.endpoint, {
      body: JSON.stringify(request),
      method: method,
      headers: {
        Accept: 'application/json',
        'Content-Type': 'application/json',
        Origin: origin
      }
    })
  }

  return analytics
}

export async function verifyDomain(cloudClient: NightlyCloud, appId: string, domainName: string) {
  await cloudClient.verifyDomainStart({ appId, domainName } as HttpVerifyDomainStartRequest)
  await cloudClient.verifyDomainFinish({ appId, domainName })
}

export function randomDomainName(): string {
  return randomEmail() + '.com'
}

export function randomOrigin(): { origin: string; domainName: string } {
  const domainName = randomDomainName()
  return { origin: 'https://' + domainName, domainName }
}
