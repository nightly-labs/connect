import { assert, describe, test } from 'vitest'
import { NightlyCloud } from './app'
import { HttpRegisterWithPasswordRequest } from '../../../bindings/HttpRegisterWithPasswordRequest'
import { createUser, randomEmail } from './testUtils'
import { HttpLoginRequest } from '../../../bindings/HttpLoginRequest'
import { HttpRegisterNewTeamRequest } from '../../../bindings/HttpRegisterNewTeamRequest'
import { HttpRegisterNewAppRequest } from '../../../bindings/HttpRegisterNewAppRequest'

const TEST_ENDPOINT = 'http://127.0.0.1:6969/cloud'

describe('Base Client tests', () => {
  const cloudClient: NightlyCloud = new NightlyCloud({
    url: TEST_ENDPOINT
  })

  test('#registerWithPassword()', async () => {
    const email = randomEmail() + '@gmail.com'

    let registerPayload = {
      email,
      password: 'Password123'
    } as HttpRegisterWithPasswordRequest

    let response = await (await cloudClient.registerWithPassword(registerPayload)).userId

    assert(response.length > 0)
  })

  test('#loginWithPassword()', async () => {
    const email = randomEmail() + '@gmail.com'
    const password = 'Password123'

    let registerPayload = {
      email,
      password
    } as HttpRegisterWithPasswordRequest

    let registerResponse = await await cloudClient.registerWithPassword(registerPayload)

    let loginPayload = {
      email,
      password,
      enforceIp: false
    } as HttpLoginRequest

    let loginResponse = await cloudClient.loginWithPassword(loginPayload)

    assert(registerResponse.userId === loginResponse.userId)
  })

  test('#registerNewTeam()', async () => {
    // create user
    await createUser(cloudClient)

    // create team
    let registerTeamPayload = {
      personal: false,
      teamName: 'Test_Team'
    } as HttpRegisterNewTeamRequest

    let response = await cloudClient.registerNewTeam(registerTeamPayload)

    assert(response.teamId.length > 0)
  })

  test('#registerNewApp()', async () => {
    // create user
    await createUser(cloudClient)

    // create team
    let registerTeamPayload = {
      personal: false,
      teamName: 'Test_Team'
    } as HttpRegisterNewTeamRequest

    let teamId = (await cloudClient.registerNewTeam(registerTeamPayload)).teamId

    // create app
    let registerAppPayload = {
      teamId: teamId,
      appName: 'Test_App',
      ackPublicKeys: [],
      whitelistedDomains: []
    } as HttpRegisterNewAppRequest

    let response = await cloudClient.registerNewApp(registerAppPayload)

    assert(response.appId.length > 0)
  })
})
