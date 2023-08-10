// import LocalStorage from 'isomorphic-localstorage'
import { HttpEndpoint } from '../../../bindings/HttpEndpoint'
import { HttpGetSessionInfoRequest } from '../../../bindings/HttpGetSessionInfoRequest'
import { HttpGetSessionInfoResponse } from '../../../bindings/HttpGetSessionInfoResponse'
import { HttpConnectSessionRequest } from '../../../bindings/HttpConnectSessionRequest'
import { HttpConnectSessionResponse } from '../../../bindings/HttpConnectSessionResponse'
import { HttpDropSessionsRequest } from '../../../bindings/HttpDropSessionsRequest'
import { HttpDropSessionsResponse } from '../../../bindings/HttpDropSessionsResponse'
import { HttpGetSessionsRequest } from '../../../bindings/HttpGetSessionsRequest'
import { HttpGetSessionsResponse } from '../../../bindings/HttpGetSessionsResponse'
import { HttpResolveRequestRequest } from '../../../bindings/HttpResolveRequestRequest'
import { HttpResolveRequestResponse } from '../../../bindings/HttpResolveRequestResponse'
import { HttpGetPendingRequestsRequest } from '../../../bindings/HttpGetPendingRequestsRequest'
import { HttpGetPendingRequestsResponse } from '../../../bindings/HttpGetPendingRequestsResponse'
import { HttpGetPendingRequestRequest } from '../../../bindings/HttpGetPendingRequestRequest'
import { HttpGetPendingRequestResponse } from '../../../bindings/HttpGetPendingRequestResponse'
import { fetch } from 'cross-fetch'
import { getRandomId } from './utils'
import {
  CustomResponseContent,
  RejectResponseContent,
  ResponseContentType,
  SignMessagesResponseContent,
  SignTransactionsResponseContent,
  SignedMessage,
  SignedTransaction
} from './responseContent'
import { RequestContent } from './content'

export interface HttpBaseClientInitialize {
  clientId?: string
  url?: string
  timeout?: number
}

export class HttpBaseClient {
  timeout: number
  clientId: string
  url: string
  public constructor({ clientId, timeout, url }: HttpBaseClientInitialize) {
    this.timeout = timeout || 10000
    this.clientId = clientId || getRandomId()
    this.url = url || 'https://nc2.nightly.app'
  }

  send = async (request: object, endpoint: HttpEndpoint, method = 'POST'): Promise<any> => {
    return await (
      await fetch(this.url + endpoint, {
        body: JSON.stringify(request),
        method: method,
        headers: {
          Accept: 'application/json',
          'Content-Type': 'application/json'
        }
      })
    ).json()
  }
  getInfo = async (sessionId: string) => {
    const request: HttpGetSessionInfoRequest = {
      sessionId
    }
    const response = (await this.send(request, '/get_session_info')) as HttpGetSessionInfoResponse
    return response
  }
  connect = async (
    request: Omit<HttpConnectSessionRequest, 'clientId'> | HttpConnectSessionRequest
  ) => {
    const payload: HttpConnectSessionRequest = {
      clientId: this.clientId,
      ...request
    }
    return (await this.send(payload, '/connect_session')) as HttpConnectSessionResponse
  }
  drop = async (request: Omit<HttpDropSessionsRequest, 'clientId'> | HttpDropSessionsRequest) => {
    const payload: HttpDropSessionsRequest = {
      clientId: this.clientId,
      ...request
    }
    return (await this.send(payload, '/drop_sessions')) as HttpDropSessionsResponse
  }
  getSessions = async (
    request: Omit<HttpGetSessionsRequest, 'clientId'> | HttpGetSessionsRequest
  ) => {
    const payload: HttpGetSessionsRequest = {
      clientId: this.clientId,
      ...request
    }
    return (await this.send(payload, '/get_sessions')) as HttpGetSessionsResponse
  }
  resolveRequest = async (
    request: Omit<HttpResolveRequestRequest, 'clientId'> | HttpResolveRequestRequest
  ) => {
    const payload: HttpResolveRequestRequest = {
      clientId: this.clientId,
      ...request
    }
    return (await this.send(payload, '/resolve_request')) as HttpResolveRequestResponse
  }
  getPendingRequests = async (
    request: Omit<HttpGetPendingRequestsRequest, 'clientId'> | HttpGetPendingRequestsRequest
  ) => {
    const payload: HttpGetPendingRequestsRequest = {
      clientId: this.clientId,
      ...request
    }
    const response = (await this.send(
      payload,
      '/get_pending_requests'
    )) as HttpGetPendingRequestsResponse
    const requests = response.pendingRequests.map((request) => {
      return {
        requestId: request.requestId,
        content: JSON.parse(request.content)
      } as unknown as RequestContent
    })
    return requests
  }
  getPendingRequest = async (
    request: Omit<HttpGetPendingRequestRequest, 'clientId'> | HttpGetPendingRequestRequest
  ) => {
    const payload: HttpGetPendingRequestRequest = {
      clientId: this.clientId,
      ...request
    }
    const response = (await this.send(
      payload,
      '/get_pending_request'
    )) as HttpGetPendingRequestResponse

    return {
      requestId: response.request.requestId,
      content: JSON.parse(response.request.content)
    } as unknown as RequestContent
  }
  resolveSignTransactions = async ({
    requestId,
    sessionId,
    signedTransactions,
    clientId
  }: HttpResolveSignTransactions) => {
    const client = clientId || this.clientId
    const content: SignTransactionsResponseContent = {
      transactions: signedTransactions,
      type: ResponseContentType.SignTransactions
    }
    await this.resolveRequest({
      requestId,
      clientId: client,
      sessionId,
      content: JSON.stringify(content)
    })
  }
  resolveSignMessages = async ({
    requestId,
    sessionId,
    signedMessages,
    clientId
  }: HttpResolveSignMessages) => {
    const client = clientId || this.clientId
    const content: SignMessagesResponseContent = {
      messages: signedMessages,
      type: ResponseContentType.SignMessages
    }
    await this.resolveRequest({
      requestId,
      clientId: client,
      sessionId,
      content: JSON.stringify(content)
    })
  }
  resolveCustom = async ({ requestId, sessionId, content, clientId }: HttpResolveCustom) => {
    const client = clientId || this.clientId
    const resolveContent: CustomResponseContent = {
      content: content,
      type: ResponseContentType.Custom
    }
    await this.resolveRequest({
      requestId,
      clientId: client,
      sessionId,
      content: JSON.stringify(resolveContent)
    })
  }
  reject = async ({ requestId, sessionId, reason, clientId }: HttpReject) => {
    const client = clientId || this.clientId
    const content: RejectResponseContent = {
      reason: reason,
      type: ResponseContentType.Reject
    }
    await this.resolveRequest({
      requestId,
      clientId: client,
      sessionId,
      content: JSON.stringify(content)
    })
  }
}
export interface HttpPendingRequest {
  requestId: string
  content: RequestContent
}
export interface HttpResolveSignTransactions {
  requestId: string
  sessionId: string
  signedTransactions: SignedTransaction[]
  clientId?: string
}
export interface HttpResolveSignMessages {
  requestId: string
  sessionId: string
  signedMessages: SignedMessage[]
  clientId?: string
}
export interface HttpResolveCustom {
  requestId: string
  sessionId: string
  content?: string
  clientId?: string
}
export interface HttpReject {
  requestId: string
  sessionId: string
  reason?: string
  clientId?: string
}
