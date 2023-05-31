import { AppDisconnectedEvent } from '@bindings/AppDisconnectedEvent'
import { VersionedTransaction } from '@solana/web3.js'
import { BaseClient, ClientBaseInitialize, Connect } from 'base'
import { TypedEmitter } from 'tiny-typed-emitter'
import { SOLANA_NETWORK } from './utils'
import { SignMessagesRequest } from '@bindings/SignMessagesRequest'
export interface SignSolanaTransactionEvent {
  requestId: string
  transactions: Array<VersionedTransaction>
  metadata?: string
}
export type SignSolanaMessageEvent = SignMessagesRequest
export interface ClientSolanaEvents {
  signTransactions: (e: SignSolanaTransactionEvent) => void
  signMessages: (e: SignSolanaMessageEvent) => void
  appDisconnected: (e: AppDisconnectedEvent) => void
}
export class ClientSolana extends TypedEmitter<ClientSolanaEvents> {
  baseClient: BaseClient
  sessionId: string | undefined = undefined
  private constructor(baseClient: BaseClient) {
    super()
    baseClient.on('signTransactions', (e) => {
      const event: SignSolanaTransactionEvent = {
        requestId: e.responseId,
        transactions: e.transactions.map((tx) => {
          return VersionedTransaction.deserialize(Buffer.from(tx.transaction, 'hex'))
        }),
        metadata: e.metadata
      }
      this.emit('signTransactions', event)
    })
    baseClient.on('signMessages', (e) => {
      this.emit('signMessages', e)
    })
    baseClient.on('appDisconnected', (e) => {
      this.emit('appDisconnected', e)
    })
    this.baseClient = baseClient
  }
  public static build = async (sessionId: string, initData: ClientBaseInitialize) => {
    const baseClient = await BaseClient.build(initData)
    const data = await baseClient.getInfo(sessionId)
    const client = new ClientSolana(baseClient)
    return { client, data }
  }
  public static create = async (initData: ClientBaseInitialize) => {
    const baseClient = await BaseClient.build(initData)
    const client = new ClientSolana(baseClient)
    return client
  }
  public getInfo = async (sessionId: string) => {
    const response = await this.baseClient.getInfo(sessionId)
    return response
  }
  public connect = async (connect: Connect) => {
    await this.baseClient.connect(connect)
    this.sessionId = connect.sessionId
  }
  public getPendingRequests = async () => {
    //Assert session id is defined
    if (this.sessionId === undefined) {
      throw new Error('Session id is undefined')
    }
    return await this.baseClient.getPendingRequests()
  }

  public resolveSignTransaction = async ({
    requestId,
    signedTransactions
  }: ResolveSignSolanaTransactions) => {
    const serializedTxs = signedTransactions
      .map((tx) => Buffer.from(tx.serialize()).toString('hex'))
      .map((tx) => {
        return { network: SOLANA_NETWORK, transaction: tx }
      })
    await this.baseClient.resolveSignTransactions({ requestId, signedTransactions: serializedTxs })
  }
  public resolveSignMessage = async ({ requestId, signature }: ResolveSignSolanaMessage) => {
    await this.baseClient.resolveSignMessages({
      requestId,
      signedMessages: [{ signedMessage: Buffer.from(signature).toString('hex') }]
    })
  }
  public rejectRequest = async (requestId: string, reason?: string) => {
    await this.baseClient.reject({ requestId, reason })
  }
}

export interface ResolveSignSolanaTransactions {
  requestId: string
  signedTransactions: VersionedTransaction[]
}
export interface ResolveSignSolanaMessage {
  requestId: string
  signature: Uint8Array
}
