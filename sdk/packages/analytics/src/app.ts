import { AppConnectEvent } from '../../../bindings/AppConnectEvent'
import { HttpNightlyConnectCloudEvent } from '../../../bindings/HttpNightlyConnectCloudEvent'
import { DEFAULT_ANALYTICS_URL } from './utils'

export interface NightlyAnalyticsParams {
  sessionId: string
  network: string
  appId?: string // We should generate if not provided
  endpoint?: string
}
// SDK for sending analytics
export class NightlyAnalytics {
  sessionId: string
  network: string
  endpoint: string = DEFAULT_ANALYTICS_URL // Endpoint for sending analytics
  appId?: string
  public constructor(params: NightlyAnalyticsParams) {
    this.sessionId = params.sessionId
    this.network = params.network
    this.endpoint = params.endpoint ?? DEFAULT_ANALYTICS_URL
    this.appId = params.appId
  }
  sendEvent = async (request: HttpNightlyConnectCloudEvent, method = 'POST') => {
    // We don't need response
    return await await fetch(this.endpoint, {
      body: JSON.stringify(request),
      method: method,
      headers: {
        Accept: 'application/json',
        'Content-Type': 'application/json'
      }
    })
  }

  appConnected = async (event: AppConnectEvent) => {
    return await this.sendEvent({
      appId: this.appId ?? event.sessionId,
      event: { AppConnect: event }
    })
  }
}
