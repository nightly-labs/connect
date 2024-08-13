import { assert, beforeAll, describe, expect, test } from 'vitest'
import { TEST_RELAY_ENDPOINT, smartDelay } from '../../../commonTestUtils'
import { NightlyAnalytics } from './app'
import { NightlyCloud } from '@nightlylabs/nightly-cloud'
import { BaseApp } from '@nightlylabs/nightly-connect-base'
import {
  createUser,
  randomDomainName,
  randomEmail,
  randomOrigin,
  setupAnalytics,
  setupTestTeam,
  verifyDomain
} from './test_utils'
import {
  AppConnectEvent,
  AppDisconnectEvent,
  ChangeNetworkEvent,
  ChangeNetworkResolveEvent,
  ChangeWalletEvent,
  ChangeWalletResolveEvent,
  ClientConnectEvent,
  ClientConnectResolveEvent,
  ClientDisconnectEvent,
  RequestFail,
  SignAndSendTransactionEvent,
  SignAndSendTransactionResolveEvent,
  SignMessageEvent,
  SignMessageResolveEvent,
  SignTransactionEvent,
  SignTransactionResolveEvent
} from '../../../bindings'

const TEST_CLOUD_ENDPOINT = 'http://127.0.0.1:6969/cloud'
const TEST_ENDPOINT = 'http://127.0.0.1:6969/cloud/public/events'

describe.concurrent('Analytics client tests', () => {
  let cloudClient: NightlyCloud
  let baseApp: BaseApp
  let teamId: string
  let appId: string
  let sessionId: string

  beforeAll(async () => {
    cloudClient = new NightlyCloud({
      url: TEST_CLOUD_ENDPOINT
    })

    const { origin, domainName } = randomOrigin()

    // Create a test team and register one app under it
    const response = await setupTestTeam(cloudClient)
    await verifyDomain(cloudClient, response.appId, domainName)
    // Create a new sessions under new app id
    baseApp = await BaseApp.build(
      {
        appMetadata: {
          additionalInfo: 'test-additional-info',
          description: 'test-app-description',
          icon: 'test-app-icon',
          name: 'test-app-name'
        },
        network: 'Solana',
        persistent: false,
        persistentSessionId: undefined,
        timeout: undefined,
        url: TEST_RELAY_ENDPOINT,
        appId: response.appId
      },
      {
        origin
      }
    )

    sessionId = baseApp.sessionId
    teamId = response.teamId
    appId = response.appId

    await smartDelay()
  })

  test('Create mock test team with app', async () => {
    const tempCloudClient = new NightlyCloud({
      url: TEST_CLOUD_ENDPOINT
    })
    const setupResult = await setupTestTeam(tempCloudClient)
    expect(setupResult).toBeDefined()
    expect(setupResult.teamId).toBeDefined()
    expect(setupResult.appId).toBeDefined()
  })

  test('Send event, original client without origin', async () => {
    const analyticsOriginal = new NightlyAnalytics({
      sessionId: sessionId,
      network: 'Solana',
      endpoint: TEST_ENDPOINT,
      appId: appId
    })

    const request = {
      sessionId: analyticsOriginal.sessionId,
      deviceMetadata: {
        mobile: {
          system: 'Unknown',
          version: '15.0'
        }
      },
      language: 'en',
      timezone: 'Europe/London',
      network: analyticsOriginal.network,
      newSession: false
    } as AppConnectEvent

    const responseWithoutOrigin = await analyticsOriginal.appConnected(request)
    assert(responseWithoutOrigin.status === 403)
  })

  test('Reject send event without domain verification', async () => {
    const { origin } = randomOrigin()
    const analytics = setupAnalytics(origin, 'Solana', TEST_ENDPOINT, appId, sessionId)
    const request = {
      sessionId: analytics.sessionId,
      deviceMetadata: {
        mobile: {
          system: 'Unknown',
          version: '15.0'
        }
      },
      language: 'en',
      timezone: 'Europe/London',
      network: analytics.network,
      newSession: false
    } as AppConnectEvent

    const response = await analytics.appConnected(request)
    expect(response.status).toBe(403)
  })

  test('Test domain verification', async () => {
    const domain = randomDomainName()
    // Try to verify domain with invalid domain name
    await expect(cloudClient.verifyDomainStart({ appId, domainName: '' })).rejects.toThrow(
      'InvalidDomainName'
    )

    // Start domain verification with valid domain name
    const verifyResponse = await cloudClient.verifyDomainStart({ appId, domainName: domain })
    expect(verifyResponse.code.length >= 36)

    // Try to finish domain verification with invalid domain name
    await expect(cloudClient.verifyDomainFinish({ appId, domainName: '' })).rejects.toThrow(
      'InvalidDomainName'
    )

    // Finish domain verification with valid domain name
    await cloudClient.verifyDomainFinish({
      appId,
      domainName: domain
    })

    // Try to verify domain with the same domain name again
    await expect(cloudClient.verifyDomainStart({ appId, domainName: domain })).rejects.toThrow(
      'DomainAlreadyVerified'
    )

    // Try to finish domain verification for domain that has already been verified
    await expect(cloudClient.verifyDomainFinish({ appId, domainName: domain })).rejects.toThrow(
      'DomainAlreadyVerified'
    )

    // Try to finish domain verification for domain that has not been started
    await expect(
      cloudClient.verifyDomainFinish({ appId, domainName: randomEmail() })
    ).rejects.toThrow('DomainVerificationNotStarted')

    // Try to verify new domain without permissions to the previously created app with already one verified domain, create new cloud client
    const tempCloudClient = new NightlyCloud({
      url: TEST_CLOUD_ENDPOINT
    })
    // create user
    await createUser(tempCloudClient)

    await expect(
      tempCloudClient.verifyDomainStart({ appId, domainName: randomEmail() })
    ).rejects.toThrow('InsufficientPermissions')

    // Try to properly verify new domain with new cloud client for the app
    const newDomain = randomEmail()
    const secondVerifyResponse = await cloudClient.verifyDomainStart({
      appId,
      domainName: newDomain
    })
    expect(secondVerifyResponse.code.length === 6)

    await cloudClient.verifyDomainFinish({
      appId,
      domainName: newDomain
    })

    // Get app data and validate that 2 domains has been verified
    const appData = await cloudClient.getUserJoinedTeams()

    const appWhitelistedDomains = appData.teamsApps[teamId][0].whitelistedDomains
    assert(appWhitelistedDomains.some((d) => d.domain === domain && d.status === 'Verified'))
    assert(appWhitelistedDomains.some((d) => d.domain === newDomain && d.status === 'Verified'))
  })

  test('Test verified domain removal', async () => {
    const domain = randomDomainName()

    // Verify the domain
    await verifyDomain(cloudClient, appId, domain)

    // Remove the domain from the app
    await cloudClient.removeDomain({ appId, domainName: domain })

    // Verify that the domain has been removed
    const appData = await cloudClient.getUserJoinedTeams()
    const appWhitelistedDomains = appData.teamsApps[teamId][0].whitelistedDomains
    assert(appWhitelistedDomains.find((d) => d.domain === domain) === undefined)
  })

  test('Test cancel domain verification challenge', async () => {
    const domain = randomDomainName()

    // Start domain verification with valid domain name
    const firstVerifyResponse = await cloudClient.verifyDomainStart({ appId, domainName: domain })
    expect(firstVerifyResponse.code.length >= 36)

    // Try to start challenge again, should simply return the same code
    const secondVerifyResponse = await cloudClient.verifyDomainStart({ appId, domainName: domain })
    expect(secondVerifyResponse.code === firstVerifyResponse.code)

    // Fetch the app data and validate that the domain is in the pending state
    const appData = await cloudClient.getTeamMetadata({ teamId })
    const currentWhitelistChallenge = appData.teamApps[0].whitelistedDomains.find(
      (d) => d.domain === domain
    )

    // Check if currentWhitelistChallenge is defined
    if (currentWhitelistChallenge !== undefined) {
      assert(currentWhitelistChallenge.status === 'Pending')

      // Cancel the challenge
      await cloudClient.cancelDomainVerification({
        appId,
        domainName: currentWhitelistChallenge.domain
      })
    } else {
      throw new Error(`Domain ${domain} not found in the whitelist.`)
    }

    // Fetch the app data and validate that the domain verification has been removed
    const appDataAfter = await cloudClient.getTeamMetadata({ teamId })
    const currentWhitelistChallengeAfter = appDataAfter.teamApps[0].whitelistedDomains.find(
      (d) => d.domain === domain
    )
    assert(currentWhitelistChallengeAfter === undefined)
  })

  test('Send event during unfinished domain registration process', async () => {
    const { origin, domainName } = randomOrigin()
    const verifyResponse = await cloudClient.verifyDomainStart({ appId, domainName })
    expect(verifyResponse.code.length >= 36)

    const analytics = setupAnalytics(origin, 'Solana', TEST_ENDPOINT, appId, sessionId)
    const request = {
      sessionId: analytics.sessionId,
      deviceMetadata: {
        mobile: {
          system: 'Unknown',
          version: '15.0'
        }
      },
      language: 'en',
      timezone: 'Europe/London',
      network: analytics.network,
      newSession: false
    } as AppConnectEvent

    const response = await analytics.appConnected(request)
    expect(response.status).toBe(403)
  })

  test('Send event success', async () => {
    const { origin, domainName } = randomOrigin()
    await verifyDomain(cloudClient, appId, domainName)

    const analytics = setupAnalytics(origin, 'Solana', TEST_ENDPOINT, appId, sessionId)
    const request = {
      sessionId: analytics.sessionId,
      deviceMetadata: {
        mobile: {
          system: 'Unknown',
          version: '15.0'
        }
      },
      language: 'en',
      timezone: 'Europe/London',
      network: analytics.network,
      newSession: false
    } as AppConnectEvent

    const response = await analytics.appConnected(request)
    expect(response.status).toBe(200)
  })

  test('Reject event from different origin', async () => {
    const { domainName } = randomOrigin()
    await verifyDomain(cloudClient, appId, domainName)

    // Send event from origin unverified for this app id
    const differentOrigin = 'http://different-origin' + randomEmail() + '.com'
    const analyticsDifferentOrigin = setupAnalytics(
      differentOrigin,
      'Solana',
      TEST_ENDPOINT,
      appId,
      sessionId
    )

    const request = {
      sessionId: analyticsDifferentOrigin.sessionId,
      deviceMetadata: {
        mobile: {
          system: 'Unknown',
          version: '15.0'
        }
      },
      language: 'en',
      timezone: 'Europe/London',
      network: analyticsDifferentOrigin.network,
      newSession: false
    } as AppConnectEvent

    const response = await analyticsDifferentOrigin.appConnected(request)
    expect(response.status).toBe(403)
  })

  describe.concurrent('Test events', () => {
    let analytics: NightlyAnalytics
    const clientId: string = 'test-client-id' + randomEmail()
    const addresses: string[] = ['test-address' + randomEmail()]

    beforeAll(async () => {
      const { origin, domainName } = randomOrigin()
      await verifyDomain(cloudClient, appId, domainName)
      analytics = setupAnalytics(origin, 'Solana', TEST_ENDPOINT, appId, sessionId)
    })

    test('Event appConnected', async () => {
      // Connect to new session
      const request = {
        sessionId: analytics.sessionId,
        deviceMetadata: {
          mobile: {
            system: 'Unknown',
            version: '15.0'
          }
        },
        language: 'en',
        timezone: 'Europe/London',
        network: analytics.network,
        newSession: true
      } as AppConnectEvent

      const response = await analytics.appConnected(request)
      expect(response.status).toBe(200)

      // Reconnect to the same session
      request.newSession = false
      const response2 = await analytics.appConnected(request)
      expect(response2.status).toBe(200)
    })

    test('Event appDisconnected', async () => {
      const request = {
        sessionId: analytics.sessionId
      } as AppDisconnectEvent

      const response = await analytics.appDisconnected(request)
      expect(response.status).toBe(200)
    })

    test('Event clientConnect', async () => {
      // Send event clientConnect with success set to false
      const request = {
        sessionId: analytics.sessionId,
        walletName: 'test-wallet-name',
        walletType: 'test-wallet-type',
        clientId: clientId,
        sessionType: 'Extension'
      } as ClientConnectEvent

      const response = await analytics.clientConnect(request)
      expect(response.status).toBe(200)
    })

    test('Event clientConnectResolve', async () => {
      const request = {
        addresses: addresses,
        sessionId: analytics.sessionId,
        success: true,
        walletName: 'test-wallet-name',
        walletType: 'test-wallet-type',
        clientId: clientId
      } as ClientConnectResolveEvent

      const response = await analytics.clientConnectResolve(request)
      expect(response.status).toBe(200)
    })

    test('Event clientDisconnect', async () => {
      const request = {
        disconnectedSessionId: analytics.sessionId,
        clientId: clientId
      } as ClientDisconnectEvent

      const response = await analytics.clientDisconnect(request)
      expect(response.status).toBe(200)
    })

    test('Event signMessage', async () => {
      const request = {
        network: 'Solana',
        requestId: 'test-request-id',
        sessionId: analytics.sessionId
      } as SignMessageEvent

      const response = await analytics.signMessage(request)
      expect(response.status).toBe(200)
    })

    test('Event signMessageResolve', async () => {
      // First send signMessageResolve event with failureReason
      const request = {
        requestId: 'test-request-id',
        sessionId: analytics.sessionId,
        failureReason: 'Rejected' as RequestFail
      } as SignMessageResolveEvent

      const response = await analytics.signMessageResolve(request)
      expect(response.status).toBe(200)

      // Then send signMessageResolve event with success
      request.failureReason = undefined

      const responseSuccess = await analytics.signMessageResolve(request)
      expect(responseSuccess.status).toBe(200)
    })

    test('Event signTransaction', async () => {
      const request = {
        network: 'Solana',
        requestId: 'test-request-id',
        sessionId: analytics.sessionId
      } as SignTransactionEvent

      const response = await analytics.signTransaction(request)
      expect(response.status).toBe(200)
    })

    test('Event signTransactionResolve', async () => {
      // First send signTransactionResolve event with failureReason
      const request = {
        requestId: 'test-request-id',
        sessionId: analytics.sessionId,
        failureReason: 'Rejected' as RequestFail
      } as SignTransactionResolveEvent

      const response = await analytics.signTransactionResolve(request)
      expect(response.status).toBe(200)

      // Then send signTransactionResolve event with success
      request.failureReason = undefined
      request.txHash = 'test-tx-hash'

      const responseSuccess = await analytics.signTransactionResolve(request)
      expect(responseSuccess.status).toBe(200)
    })

    test('Event signAndSendTransaction', async () => {
      const request = {
        network: 'Solana',
        requestId: 'test-request-id',
        sessionId: analytics.sessionId
      } as SignAndSendTransactionEvent

      const response = await analytics.signAndSendTransaction(request)
      expect(response.status).toBe(200)
    })

    test('Event signAndSendTransactionResolve', async () => {
      // First send signAndSendTransactionResolve event with failureReason
      const request = {
        requestId: 'test-request-id',
        sessionId: analytics.sessionId,
        failureReason: 'Rejected' as RequestFail
      } as SignAndSendTransactionResolveEvent

      const response = await analytics.signAndSendTransactionResolve(request)
      expect(response.status).toBe(200)

      // Then send signAndSendTransactionResolve event with success
      request.failureReason = undefined
      request.txHash = 'test-tx-hash'

      const responseSuccess = await analytics.signAndSendTransactionResolve(request)
      expect(responseSuccess.status).toBe(200)
    })

    test('Event changeWallet', async () => {
      const request = {
        network: 'Solana',
        requestId: 'test-request-id',
        sessionId: analytics.sessionId,
        walletName: 'test-wallet-name',
        walletType: 'test-wallet-type',
        oldWalletAddress: 'test-old-wallet-address'
      } as ChangeWalletEvent

      const response = await analytics.changeWallet(request)
      expect(response.status).toBe(200)
    })

    test('Event changeWalletResolve', async () => {
      // First send changeWalletResolve event with failureReason
      const request = {
        requestId: 'test-request-id',
        sessionId: analytics.sessionId,
        failureReason: 'TimedOut' as RequestFail
      } as ChangeWalletResolveEvent

      const response = await analytics.changeWalletResolve(request)
      expect(response.status).toBe(200)

      // Then send changeWalletResolve event with success
      request.failureReason = undefined
      request.newWalletAddress = 'new-wallet-address'

      const responseSuccess = await analytics.changeWalletResolve(request)
      expect(responseSuccess.status).toBe(200)
    })

    test('Event changeNetwork', async () => {
      const request = {
        oldNetwork: 'Solana',
        requestId: 'test-request-id',
        sessionId: analytics.sessionId
      } as ChangeNetworkEvent

      const response = await analytics.changeNetwork(request)
      expect(response.status).toBe(200)
    })

    test('Event changeNetworkResolve', async () => {
      // First send changeNetworkResolve event with failureReason
      const request = {
        requestId: 'test-request-id',
        sessionId: analytics.sessionId,
        failureReason: 'TimedOut' as RequestFail
      } as ChangeNetworkResolveEvent

      const response = await analytics.changeNetworkResolve(request)
      expect(response.status).toBe(200)

      // Then send changeWalletResolve event with success
      request.failureReason = undefined
      request.newNetwork = 'Aptos'

      const responseSuccess = await analytics.changeNetworkResolve(request)
      expect(responseSuccess.status).toBe(200)
    })
  })
})
