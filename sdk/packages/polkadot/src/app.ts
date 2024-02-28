import {
  AppBaseInitialize,
  BaseApp,
  DeeplinkConnect,
  getWalletsMetadata
} from '@nightlylabs/nightly-connect-base'
import { InjectedAccount, InjectedExtension } from '@polkadot/extension-inject/types'
import { EventEmitter } from 'eventemitter3'
import { UserDisconnectedEvent } from '../../../bindings/UserDisconnectedEvent'
import { WalletMetadata } from '../../../bindings/WalletMetadata'
import { Accounts } from './Accounts'
import { Signer } from './Signer'

export type AppPolkadotInitialize = AppBaseInitialize
interface PolkadotAppEvents {
  userConnected: (e: InjectedAccount[]) => void
  userDisconnected: (e: UserDisconnectedEvent) => void
  serverDisconnected: () => void
}
export class AppPolkadot extends EventEmitter<PolkadotAppEvents> implements InjectedExtension {
  sessionId: string
  initData: AppPolkadotInitialize
  // Polkadot specific
  name = 'Nightly Connect'
  version = '0.0.1'
  accounts: Accounts
  // metadata?: InjectedMetadata
  // provider?: InjectedProvider
  signer: Signer
  constructor(base: BaseApp, initData: AppPolkadotInitialize) {
    super()
    this.initData = initData
    base.on('userConnected', (e) => {
      if (e.metadata) {
        const accounts = JSON.parse(e.metadata) as InjectedAccount[]
        this.accounts.updateActiveAccounts(accounts)
        this.emit('userConnected', accounts)
      } else {
        const accounts = e.publicKeys.map((pk) => ({ address: pk })) as InjectedAccount[]
        this.accounts.updateActiveAccounts(accounts)
        this.emit('userConnected', accounts)
      }
    })
    base.on('userDisconnected', (e) => {
      this.emit('userDisconnected', e)
    })
    base.on('serverDisconnected', async () => {
      // We need this because of power saving mode on mobile
      await this.tryReconnect()
    })
    this.accounts = new Accounts()
    if (base.hasBeenRestored && base.connectedPublicKeys) {
      // If the base has been restored, we can get the accounts from the metadata
      // Polkadot specific
      if (base.clientMetadata) {
        const accounts = JSON.parse(base.clientMetadata) as InjectedAccount[]
        this.accounts.updateActiveAccounts(accounts)
      } else {
        // Fall back to the public keys
        const accounts = base.connectedPublicKeys.map((pk) => ({
          address: pk
        })) as InjectedAccount[]
        this.accounts.updateActiveAccounts(accounts)
      }
    }

    this.signer = new Signer(base)
    this.sessionId = base.sessionId
  }
  private tryReconnect = async () => {
    try {
      const base = await BaseApp.build({ ...this.initData })
      // On reconnect, if the base has not been restored, emit serverDisconnected
      if (!base.hasBeenRestored) {
        this.emit('serverDisconnected')
        return
      }
      // If user was connected, emit userConnected
      if (base.connectedPublicKeys.length > 0) {
        if (base.clientMetadata) {
          const accounts = JSON.parse(base.clientMetadata) as InjectedAccount[]
          this.accounts.updateActiveAccounts(accounts)
          this.emit('userConnected', accounts)
        } else {
          const accounts = base.connectedPublicKeys.map((pk) => ({
            address: pk
          })) as InjectedAccount[]
          this.accounts.updateActiveAccounts(accounts)
          this.emit('userConnected', accounts)
        }
      }
      base.on('userConnected', (e) => {
        if (e.metadata) {
          const accounts = JSON.parse(e.metadata) as InjectedAccount[]
          this.accounts.updateActiveAccounts(accounts)
          this.emit('userConnected', accounts)
        } else {
          const accounts = e.publicKeys.map((pk) => ({ address: pk })) as InjectedAccount[]
          this.accounts.updateActiveAccounts(accounts)
          this.emit('userConnected', accounts)
        }
      })
      base.on('userDisconnected', (e) => {
        this.emit('userDisconnected', e)
      })
      base.on('serverDisconnected', async () => {
        await this.tryReconnect()
      })
      // If there is a deeplink, reconnect to it
      if (this.signer.base.deeplink) {
        base.connectDeeplink(this.signer.base.deeplink)
      }
      this.signer.base = base
      return
    } catch (_) {
      this.emit('serverDisconnected')
    }
  }
  public hasBeenRestored = () => {
    return this.signer.base.hasBeenRestored
  }

  public static getWalletsMetadata = async (url?: string): Promise<WalletMetadata[]> => {
    return getWalletsMetadata(url, 'polkadot')
  }

  public static build = async (initData: AppPolkadotInitialize): Promise<AppPolkadot> => {
    const base = await BaseApp.build(initData)
    return new AppPolkadot(base, initData)
  }

  connectDeeplink = async (data: DeeplinkConnect) => {
    this.signer.base.connectDeeplink(data)
  }
}
