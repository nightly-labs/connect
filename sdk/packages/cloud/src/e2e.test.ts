import { assert, describe, expect, test } from 'vitest'
import { NightlyCloud } from './app'
import { addUserToTeam, basicTeamSetup, createUser, randomEmail } from './testUtils'
import {
  HttpCancelUserTeamInviteRequest,
  HttpGetAppEventsRequest,
  HttpInviteUserToTeamRequest,
  HttpLeaveTeamRequest,
  HttpLoginRequest,
  HttpRegisterNewAppRequest,
  HttpRegisterNewTeamRequest,
  HttpRegisterWithPasswordStartRequest,
  HttpRemoveUserFromTeamRequest
} from '../../../bindings'

const TEST_ENDPOINT = 'http://127.0.0.1:6969/cloud'

describe.concurrent(
  'Base Client tests',
  () => {
    test('#registerWithPassword()', async () => {
      const cloudClient: NightlyCloud = new NightlyCloud({
        url: TEST_ENDPOINT
      })

      const email = randomEmail() + '@gmail.com'

      const registerPayload = {
        email,
        password: 'Password123'
      } as HttpRegisterWithPasswordStartRequest

      await cloudClient.registerWithPasswordStart(registerPayload)
    })

    test('#loginWithPassword()', async () => {
      const cloudClient: NightlyCloud = new NightlyCloud({
        url: TEST_ENDPOINT
      })
      const email = randomEmail() + '@gmail.com'
      const password = 'Password123'

      const registerPayload = {
        email,
        password
      } as HttpRegisterWithPasswordStartRequest

      await cloudClient.registerWithPasswordStart(registerPayload)

      const authCode = await cloudClient.verifyCode({
        email,
        code: '123456',
        action: 'registerPassword'
      })

      await cloudClient.registerWithPasswordFinish({
        authCode: authCode.verificationCode,
        email,
        newPassword: password
      })

      const loginPayload = {
        email,
        password,
        enforceIp: false
      } as HttpLoginRequest

      const loginResponse = await cloudClient.loginWithPassword(loginPayload)

      assert(loginResponse.userId.length > 0)
    })

    test('#refreshToken()', async () => {
      const cloudClient: NightlyCloud = new NightlyCloud({
        url: TEST_ENDPOINT
      })
      const email = randomEmail() + '@gmail.com'
      const password = 'Password123'

      const registerPayload = {
        email,
        password
      } as HttpRegisterWithPasswordStartRequest

      await cloudClient.registerWithPasswordStart(registerPayload)

      const authCode = await cloudClient.verifyCode({
        email,
        code: '123456',
        action: 'registerPassword'
      })

      await cloudClient.registerWithPasswordFinish({
        authCode: authCode.verificationCode,
        email,
        newPassword: password
      })

      const loginPayload = {
        email,
        password,
        enforceIp: false
      } as HttpLoginRequest

      const loginResponse = await cloudClient.loginWithPassword(loginPayload)

      assert(loginResponse.userId.length > 0)

      // Save current token
      const currentToken = cloudClient.authToken

      // Refresh token
      const refreshToken = await cloudClient.refreshAuthToken()
      assert(refreshToken.authToken.length > 0)

      // Check if token is different
      assert(currentToken !== refreshToken.authToken)

      // Check if token is valid
      const response = await cloudClient.getUserMetadata()
      assert(response.userId.length > 0)
      assert(response.email === email)
    })

    test('#resetPassword()', async () => {
      const cloudClient: NightlyCloud = new NightlyCloud({
        url: TEST_ENDPOINT
      })
      // create user
      const { userId, email } = await createUser(cloudClient)

      // Send reset password request
      const newPassword = 'NewPassword123124123'
      await cloudClient.resetPasswordStart({ email })

      const authCode = await cloudClient.verifyCode({
        email,
        code: '123456',
        action: 'resetPassword'
      })

      // Finish reset password, the authCode doesn't matter
      await cloudClient.resetPasswordFinish({
        authCode: authCode.verificationCode,
        email,
        newPassword: newPassword
      })

      // Login once again with new password
      const loginPayload = {
        email,
        password: newPassword,
        enforceIp: false
      } as HttpLoginRequest

      const loginResponse = await cloudClient.loginWithPassword(loginPayload)

      assert(loginResponse.userId == userId)
    })

    test('#registerNewTeam()', async () => {
      const cloudClient: NightlyCloud = new NightlyCloud({
        url: TEST_ENDPOINT
      })
      // create user
      await createUser(cloudClient)

      // create team
      const registerTeamPayload = {
        personal: false,
        teamName: 'Test_Team'
      } as HttpRegisterNewTeamRequest

      const response = await cloudClient.registerNewTeam(registerTeamPayload)

      assert(response.teamId.length > 0)
    }, 10000)

    test('#registerNewApp()', async () => {
      const cloudClient: NightlyCloud = new NightlyCloud({
        url: TEST_ENDPOINT
      })
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
      const cloudClient: NightlyCloud = new NightlyCloud({
        url: TEST_ENDPOINT
      })
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
      const cloudClient: NightlyCloud = new NightlyCloud({
        url: TEST_ENDPOINT
      })
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
      const cloudClient: NightlyCloud = new NightlyCloud({
        url: TEST_ENDPOINT
      })
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
      const cloudClient: NightlyCloud = new NightlyCloud({
        url: TEST_ENDPOINT
      })
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
      const cloudClient: NightlyCloud = new NightlyCloud({
        url: TEST_ENDPOINT
      })
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

    test('#leaveTheTeam()', async () => {
      const cloudClient: NightlyCloud = new NightlyCloud({
        url: TEST_ENDPOINT
      })
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

      const leavePayload = {
        teamId: teamId
      } as HttpLeaveTeamRequest

      // Remove user from team
      await newClient.leaveTeam(leavePayload)

      // Get user joined team
      const thirdResponse = await newClient.getUserJoinedTeams()

      expect(Object.keys(thirdResponse.teams)).toHaveLength(0)
      expect(Object.keys(thirdResponse.teamsApps)).toHaveLength(0)
      expect(Object.keys(thirdResponse.userPrivileges)).toHaveLength(0)
    })

    test('#cancelTeamUserInvite()', async () => {
      const cloudClient: NightlyCloud = new NightlyCloud({
        url: TEST_ENDPOINT
      })
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
      const cloudClient: NightlyCloud = new NightlyCloud({
        url: TEST_ENDPOINT
      })
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

    test('#getAppEvents()', async () => {
      const cloudClient: NightlyCloud = new NightlyCloud({
        url: TEST_ENDPOINT
      })
      // create user
      await createUser(cloudClient)

      // create basic team setup
      const { teamId, appId } = await basicTeamSetup(cloudClient)

      // Get app events, should be empty
      const payload = {
        appId: appId
      } as HttpGetAppEventsRequest

      const response = await cloudClient.getAppEvents(payload)

      expect(response.events).toHaveLength(0)
    })

    test('#changeUserPrivileges()', async () => {
      const cloudClient: NightlyCloud = new NightlyCloud({
        url: TEST_ENDPOINT
      })
      // create user
      const { userId: adminUserId, email: adminEmail } = await createUser(cloudClient)

      // create basic team setup
      const { teamId, appId } = await basicTeamSetup(cloudClient)

      // register new user
      const newClient = new NightlyCloud({
        url: TEST_ENDPOINT
      })
      const { userId, email } = await createUser(newClient)

      // Add user to team
      await addUserToTeam(cloudClient, newClient, teamId, email)

      // Check user privileges
      const firstResponse = await cloudClient.getTeamUsersPrivileges({ teamId })

      assert(firstResponse.usersPrivileges.length === 2)
      assert(firstResponse.usersPrivileges[0].userEmail === adminEmail)
      assert(firstResponse.usersPrivileges[0].appId === appId)
      assert(firstResponse.usersPrivileges[0].privilege === 'Admin')
      assert(firstResponse.usersPrivileges[1].userEmail === email)
      assert(firstResponse.usersPrivileges[1].appId === appId)
      assert(firstResponse.usersPrivileges[1].privilege === 'Read')

      await cloudClient.changeUserPrivileges({
        teamId: teamId,
        privilegesChanges: [{ appId, userEmail: email, newPrivilegeLevel: 'edit' }]
      })

      // Get privileges
      const secondResponse = await cloudClient.getTeamUsersPrivileges({ teamId })

      assert(secondResponse.usersPrivileges.length === 2)
      assert(secondResponse.usersPrivileges[0].userEmail === adminEmail)
      assert(secondResponse.usersPrivileges[0].appId === appId)
      assert(secondResponse.usersPrivileges[0].privilege === 'Admin')
      assert(secondResponse.usersPrivileges[1].userEmail === email)
      assert(secondResponse.usersPrivileges[1].appId === appId)
      assert(secondResponse.usersPrivileges[1].privilege === 'Edit')
    })

    test('#getUserMetadata()', async () => {
      const cloudClient: NightlyCloud = new NightlyCloud({
        url: TEST_ENDPOINT
      })
      // create user
      const { userId, email } = await createUser(cloudClient)

      // Get user metadata
      const response = await cloudClient.getUserMetadata()

      assert(response.userId === userId)
      assert(response.email === email)
      assert(response.passwordSet === true)
      assert(response.passkeyIds.length === 0)
    })

    test('#getTeamMetadata()', async () => {
      const cloudClient: NightlyCloud = new NightlyCloud({
        url: TEST_ENDPOINT
      })
      // create user
      const { userId: adminUserId, email: adminEmail } = await createUser(cloudClient)

      // create basic team setup
      const { teamId, appId } = await basicTeamSetup(cloudClient)

      // register new user
      const newClient = new NightlyCloud({
        url: TEST_ENDPOINT
      })
      const { userId, email } = await createUser(newClient)

      // Add user to team
      await addUserToTeam(cloudClient, newClient, teamId, email)

      // Get team metadata
      const response = await cloudClient.getTeamMetadata({ teamId })

      assert(response.teamMetadata.teamId === teamId)
      assert(response.teamMetadata.teamName === 'Test_Team')
      assert(response.teamMetadata.creatorEmail === adminEmail)
      assert(response.teamMetadata.personalTeam === false)

      assert(response.teamApps.length === 1)
      assert(response.teamApps[0].appId === appId)
      assert(response.teamApps[0].appName === 'Test_App')
      assert(response.teamApps[0].ackPublicKeys.length === 0)
      assert(response.teamApps[0].whitelistedDomains.length === 0)

      assert(response.teamMembers.length === 2)
      assert(response.teamMembers.find((member) => member === adminEmail))
      assert(response.teamMembers.find((member) => member === email))
    })
  },
  {
    timeout: 10000
  }
)
