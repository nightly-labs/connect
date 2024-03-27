import { assert, describe, expect, test } from 'vitest'
import { NightlyCloud } from './app'
import { HttpRegisterWithPasswordRequest } from '../../../bindings/HttpRegisterWithPasswordRequest'
import { addUserToTeam, basicTeamSetup, createUser, randomEmail } from './testUtils'
import { HttpLoginRequest } from '../../../bindings/HttpLoginRequest'
import { HttpRegisterNewTeamRequest } from '../../../bindings/HttpRegisterNewTeamRequest'
import { HttpRegisterNewAppRequest } from '../../../bindings/HttpRegisterNewAppRequest'
import { HttpInviteUserToTeamRequest } from '../../../bindings/HttpInviteUserToTeamRequest'
import { HttpRemoveUserFromTeamRequest } from '../../../bindings/HttpRemoveUserFromTeamRequest'
import { HttpCancelUserTeamInviteRequest } from '../../../bindings/HttpCancelUserTeamInviteRequest'

const TEST_ENDPOINT = 'http://127.0.0.1:6969/cloud'

describe('Base Client tests', () => {
  const cloudClient: NightlyCloud = new NightlyCloud({
    url: TEST_ENDPOINT
  })

  test('#registerWithPassword()', async () => {
    const email = randomEmail() + '@gmail.com'

    const registerPayload = {
      email,
      password: 'Password123'
    } as HttpRegisterWithPasswordRequest

    const response = await (await cloudClient.registerWithPassword(registerPayload)).userId

    assert(response.length > 0)
  })

  test('#loginWithPassword()', async () => {
    const email = randomEmail() + '@gmail.com'
    const password = 'Password123'

    const registerPayload = {
      email,
      password
    } as HttpRegisterWithPasswordRequest

    const registerResponse = await await cloudClient.registerWithPassword(registerPayload)

    const loginPayload = {
      email,
      password,
      enforceIp: false
    } as HttpLoginRequest

    const loginResponse = await cloudClient.loginWithPassword(loginPayload)

    assert(registerResponse.userId === loginResponse.userId)
  })

  test('#registerNewTeam()', async () => {
    // create user
    await createUser(cloudClient)

    // create team
    const registerTeamPayload = {
      personal: false,
      teamName: 'Test_Team'
    } as HttpRegisterNewTeamRequest

    const response = await cloudClient.registerNewTeam(registerTeamPayload)

    assert(response.teamId.length > 0)
  })

  test('#registerNewApp()', async () => {
    // create user
    await createUser(cloudClient)

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

    const response = await cloudClient.registerNewApp(registerAppPayload)

    assert(response.appId.length > 0)
  })

  test('#getUserTeamInvites()', async () => {
    // create user
    await createUser(cloudClient)

    // create basic team setup
    const { teamId, appId } = await basicTeamSetup(cloudClient)

    // register new user
    const newClient = new NightlyCloud({
      url: TEST_ENDPOINT
    })
    const { userId, email } = await createUser(newClient)

    const invitePayload = {
      teamId: teamId,
      userEmail: email
    } as HttpInviteUserToTeamRequest

    // Use team admin client to invite new user
    await cloudClient.inviteUserToTeam(invitePayload)

    // Get use team invites by new user
    const response = await newClient.getUserTeamInvites()

    assert(response.teamInvites.length > 0)
    assert(response.teamInvites[0].teamName === 'Test_Team')
    assert(response.teamInvites[0].userEmail === email)
  })

  test('#getTeamUserInvites()', async () => {
    // create user
    await createUser(cloudClient)

    // create basic team setup
    const { teamId, appId } = await basicTeamSetup(cloudClient)

    // register new user
    const newClient = new NightlyCloud({
      url: TEST_ENDPOINT
    })
    const { userId, email } = await createUser(newClient)

    const invitePayload = {
      teamId: teamId,
      userEmail: email
    } as HttpInviteUserToTeamRequest

    // Use team admin client to invite new user
    await cloudClient.inviteUserToTeam(invitePayload)

    // Get team invites by team admin
    const payload = {
      teamId: teamId
    } as HttpInviteUserToTeamRequest

    const response = await cloudClient.getTeamUserInvites(payload)

    assert(response.teamInvites.length > 0)
    assert(response.teamInvites[0].teamName === 'Test_Team')
    assert(response.teamInvites[0].userEmail === email)
  })

  test('#acceptTeamInvite()', async () => {
    // create user
    await createUser(cloudClient)

    // create basic team setup
    const { teamId, appId } = await basicTeamSetup(cloudClient)

    // register new user
    const newClient = new NightlyCloud({
      url: TEST_ENDPOINT
    })
    const { userId, email } = await createUser(newClient)

    const invitePayload = {
      teamId: teamId,
      userEmail: email
    } as HttpInviteUserToTeamRequest

    // Use team admin client to invite new user
    await cloudClient.inviteUserToTeam(invitePayload)

    // Get team invites by invited user
    const payload = {
      teamId: teamId
    } as HttpInviteUserToTeamRequest

    const response = await cloudClient.getTeamUserInvites(payload)

    assert(response.teamInvites.length > 0)
    assert(response.teamInvites[0].teamName === 'Test_Team')
    assert(response.teamInvites[0].userEmail === email)

    // Accept team invite
    const acceptPayload = {
      teamId: teamId
    } as HttpInviteUserToTeamRequest

    await newClient.acceptTeamInvite(acceptPayload)

    // Get team invites by invited user
    const secondResponse = await cloudClient.getTeamUserInvites(payload)

    assert(secondResponse.teamInvites.length === 0)
  })

  test('#getUserJoinedTeams()', async () => {
    // create user
    await createUser(cloudClient)

    // create basic team setup
    const { teamId, appId } = await basicTeamSetup(cloudClient)

    // register new user
    const newClient = new NightlyCloud({
      url: TEST_ENDPOINT
    })
    const { userId, email } = await createUser(newClient)

    // Get user joined team
    const response = await newClient.getUserJoinedTeams()

    expect(Object.keys(response.teams)).toHaveLength(0)
    expect(Object.keys(response.teamsApps)).toHaveLength(0)
    expect(Object.keys(response.userPrivileges)).toHaveLength(0)

    // Add user to team
    await addUserToTeam(cloudClient, newClient, teamId, email)

    // Get user joined team
    const secondResponse = await newClient.getUserJoinedTeams()

    expect(Object.keys(secondResponse.teams)).toHaveLength(1)
    expect(Object.keys(secondResponse.teamsApps)).toHaveLength(1)
    expect(Object.keys(secondResponse.userPrivileges)).toHaveLength(1)
  })

  test('#removeUserFromTeam()', async () => {
    // create user
    await createUser(cloudClient)

    // create basic team setup
    const { teamId, appId } = await basicTeamSetup(cloudClient)

    // register new user
    const newClient = new NightlyCloud({
      url: TEST_ENDPOINT
    })
    const { userId, email } = await createUser(newClient)

    // Get user joined team
    const response = await newClient.getUserJoinedTeams()

    expect(Object.keys(response.teams)).toHaveLength(0)
    expect(Object.keys(response.teamsApps)).toHaveLength(0)
    expect(Object.keys(response.userPrivileges)).toHaveLength(0)

    // Add user to team
    await addUserToTeam(cloudClient, newClient, teamId, email)

    // Get user joined team
    const secondResponse = await newClient.getUserJoinedTeams()

    expect(Object.keys(secondResponse.teams)).toHaveLength(1)
    expect(Object.keys(secondResponse.teamsApps)).toHaveLength(1)
    expect(Object.keys(secondResponse.userPrivileges)).toHaveLength(1)

    const removePayload = {
      teamId: teamId,
      userEmail: email
    } as HttpRemoveUserFromTeamRequest

    // Remove user from team
    await cloudClient.removeUserFromTeam(removePayload)

    // Get user joined team
    const thirdResponse = await newClient.getUserJoinedTeams()

    expect(Object.keys(thirdResponse.teams)).toHaveLength(0)
    expect(Object.keys(thirdResponse.teamsApps)).toHaveLength(0)
    expect(Object.keys(thirdResponse.userPrivileges)).toHaveLength(0)
  })

  test('#cancelTeamUserInvite()', async () => {
    // create user
    await createUser(cloudClient)

    // create basic team setup
    const { teamId, appId } = await basicTeamSetup(cloudClient)

    // register new user
    const newClient = new NightlyCloud({
      url: TEST_ENDPOINT
    })
    const { userId, email } = await createUser(newClient)

    const invitePayload = {
      teamId: teamId,
      userEmail: email
    } as HttpInviteUserToTeamRequest

    // Use team admin client to invite new user
    await cloudClient.inviteUserToTeam(invitePayload)

    // Get team invites by team admin
    const payload = {
      teamId: teamId
    } as HttpInviteUserToTeamRequest

    const response = await cloudClient.getTeamUserInvites(payload)

    assert(response.teamInvites.length > 0)
    assert(response.teamInvites[0].teamName === 'Test_Team')
    assert(response.teamInvites[0].userEmail === email)

    // Cancel team invite
    const cancelPayload = {
      teamId: teamId,
      userEmail: email
    } as HttpInviteUserToTeamRequest

    await cloudClient.cancelTeamUserInvite(cancelPayload)

    // Get team invites by team admin
    const secondResponse = await cloudClient.getTeamUserInvites(payload)

    assert(secondResponse.teamInvites.length === 0)
  })

  test('#cancelUserTeamInvite()', async () => {
    // create user
    await createUser(cloudClient)

    // create basic team setup
    const { teamId, appId } = await basicTeamSetup(cloudClient)

    // register new user
    const newClient = new NightlyCloud({
      url: TEST_ENDPOINT
    })
    const { userId, email } = await createUser(newClient)

    const invitePayload = {
      teamId: teamId,
      userEmail: email
    } as HttpInviteUserToTeamRequest

    // Use team admin client to invite new user
    await cloudClient.inviteUserToTeam(invitePayload)

    // Get team invites by new user
    const response = await newClient.getUserTeamInvites()

    assert(response.teamInvites.length > 0)
    assert(response.teamInvites[0].teamName === 'Test_Team')
    assert(response.teamInvites[0].userEmail === email)

    // Cancel team invite by invited user
    const cancelPayload = {
      teamId: teamId
    } as HttpCancelUserTeamInviteRequest

    await newClient.cancelUserTeamInvite(cancelPayload)

    // Get team invites by new user
    const secondResponse = await newClient.getUserTeamInvites()

    assert(secondResponse.teamInvites.length === 0)
  })
})
