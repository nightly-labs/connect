import { assert, beforeAll, beforeEach, describe, expect, test } from 'vitest'
import { TEST_RELAY_ENDPOINT, smartDelay } from '../../../commonTestUtils'
import { NightlyAnalytics } from './app'
import { NightlyCloud } from '@nightlylabs/nightly-cloud'
import { BaseApp } from '@nightlylabs/nightly-connect-base'
import { createUser, randomEmail, setupAnalytics, setupTestTeam, verifyDomain } from './test_utils'
import { AppConnectEvent, AppDisconnectEvent, ClientConnectResolveEvent } from '../../../bindings'

const TEST_CLOUD_ENDPOINT = 'http://127.0.0.1:6969/cloud'
const TEST_ENDPOINT = 'http://127.0.0.1:6969/cloud/public/events'

describe('Analytics client tests', () => {
  let cloudClient: NightlyCloud
  let baseApp: BaseApp
  let teamId: string
  let appId: string
  let sessionId: string

  beforeAll(async () => {
    cloudClient = new NightlyCloud({
      url: TEST_CLOUD_ENDPOINT
    })

    // Create a test team and register one app under it
    const response = await setupTestTeam(cloudClient)

    // Create a new sessions under new app id
    baseApp = await BaseApp.build({
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
    })

    sessionId = baseApp.sessionId
    teamId = response.teamId
    appId = response.appId

    await smartDelay()
  })

  test('Create mock test with app', async () => {
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
    const domain = randomEmail()
    const analytics = setupAnalytics(domain, 'Solana', TEST_ENDPOINT, appId, sessionId)
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
    const domain = randomEmail()
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
    assert(appWhitelistedDomains.length === 2)
    assert(appWhitelistedDomains.includes(domain))
    assert(appWhitelistedDomains.includes(newDomain))
  })

  test('Test verified domain removal', async () => {
    const domain = randomEmail()

    // Verify the domain
    await verifyDomain(cloudClient, appId, domain)

    // Remove the domain from the app
    await cloudClient.removeDomain({ appId, domainName: domain })

    // Verify that the domain has been removed
    const appData = await cloudClient.getUserJoinedTeams()
    const appWhitelistedDomains = appData.teamsApps[teamId][0].whitelistedDomains
    assert(appWhitelistedDomains.find((d) => d === domain) === undefined)
  })

  test('Send event during unfinished domain registration process', async () => {
    const domain = randomEmail()
    const verifyResponse = await cloudClient.verifyDomainStart({ appId, domainName: domain })
    expect(verifyResponse.code.length >= 36)

    const analytics = setupAnalytics(domain, 'Solana', TEST_ENDPOINT, appId, sessionId)
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
    const domain = randomEmail()
    await verifyDomain(cloudClient, appId, domain)

    const analytics = setupAnalytics(domain, 'Solana', TEST_ENDPOINT, appId, sessionId)
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
    const domain = randomEmail()
    await verifyDomain(cloudClient, appId, domain)

    // Send event from origin unverified for this app id
    const differentDomain = 'different-origin' + randomEmail()
    const analyticsDifferentOrigin = setupAnalytics(
      differentDomain,
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

  describe.only('Test events', () => {
    let domain
    let analytics

    beforeAll(async () => {
      domain = randomEmail()
      await verifyDomain(cloudClient, appId, domain)
      analytics = setupAnalytics(domain, 'Solana', TEST_ENDPOINT, appId, sessionId)
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
        clientId: 'test-client-id',
        addresses: ['test-address'],
        walletName: 'test-wallet-name',
        walletType: 'test-wallet-type',
        success: false
      } as ClientConnectResolveEvent

      const response = await analytics.appDisconnected(request)
      expect(response.status).toBe(200)

      // Send event clientConnect with success set to true
      request.success = true
      const response2 = await analytics.appDisconnected(request)
      expect(response2.status).toBe(200)
    })
  })
})
