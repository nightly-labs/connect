/* eslint-disable @typescript-eslint/no-empty-function */
import {
  AppPolkadot,
  AppPolkadotInitialize,
  WalletMetadata
} from '@nightlylabs/nightly-connect-polkadot'
import {
  NightlyConnectSelectorModal,
  XMLOptions,
  clearRecentStandardWalletForNetwork,
  clearSessionIdForNetwork,
  getRecentStandardWalletForNetwork,
  isMobileBrowser,
  isStandardConnectedForNetwork,
  logoBase64,
  persistRecentStandardWalletForNetwork,
  persistStandardConnectForNetwork,
  persistStandardDisconnectForNetwork,
  sleep,
  triggerConnect
} from '@nightlylabs/wallet-selector-base'

import { type Signer as InjectedSigner } from '@polkadot/api/types'
import { type Injected } from '@polkadot/extension-inject/types'
import { IPolkadotWalletListItem, getPolkadotWalletsList } from './detection'
import { networkToData, SupportedNetworks } from './utils'
export type AppSelectorInitialize = Omit<AppPolkadotInitialize, 'network'> & {
  network: SupportedNetworks
}
export class NightlyConnectAdapter implements Injected {
  name = 'Nightly Connect'
  url = 'https://nightly.app'
  icon = logoBase64

  private _connecting: boolean
  private _connected: boolean

  private _app: AppPolkadot | undefined
  private _appSessionActive: boolean
  private _innerStandardAdapter: Injected | undefined
  private _modal: NightlyConnectSelectorModal | undefined

  private _appInitData: AppSelectorInitialize
  private _useEagerConnect: boolean

  private _metadataWallets: WalletMetadata[] = []
  private _walletsList: IPolkadotWalletListItem[] = []

  private _chosenMobileWalletName: string | undefined

  private _loading: boolean

  private _initOnConnect: boolean

  constructor(
    appInitData: AppSelectorInitialize,
    useEagerConnect?: boolean,
    initOnConnect = false
  ) {
    this._connecting = false
    this._connected = false
    this._appInitData = appInitData
    this._useEagerConnect = !!useEagerConnect
    this._appSessionActive = false
    this._loading = false
    this._initOnConnect = initOnConnect
  }

  get accounts() {
    // Means that we are connected via standard wallet
    if (this._innerStandardAdapter) {
      return this._innerStandardAdapter.accounts
    }
    // Fall back to Nightly Connect
    if (this._app) {
      return this._app.accounts
    }

    return {
      get: async () => [],
      subscribe: () => {
        return () => {}
      }
    }
  }
  get signer(): InjectedSigner {
    if (!this.connected) {
      throw new Error('Not connected')
    }
    // Means that we are connected via standard wallet
    if (this._innerStandardAdapter) {
      return this._innerStandardAdapter.signer
    }
    // Fall back to Nightly Connect
    if (this._app) {
      return this._app.signer
    }
    throw new Error('App not initialized')
  }
  get connecting() {
    return this._connecting
  }

  get connected() {
    return this._connected
  }
  get network() {
    return this._appInitData.network
  }
  get walletsList() {
    return this._walletsList
  }

  set walletsList(list: IPolkadotWalletListItem[]) {
    this._walletsList = list
    if (this._modal) {
      this._modal.walletsList = list
    }
  }

  public static initApp = async (
    appInitData: AppSelectorInitialize
  ): Promise<[AppPolkadot, WalletMetadata[]]> => {
    try {
      return await Promise.all([
        AppPolkadot.build(appInitData),
        AppPolkadot.getWalletsMetadata(
          `${appInitData.url ?? 'https://nc2.nightly.app'}/get_wallets_metadata`
        ).catch(() => [] as WalletMetadata[])
      ])
    } catch {
      clearSessionIdForNetwork(appInitData.network)
      return await Promise.all([
        AppPolkadot.build(appInitData),
        AppPolkadot.getWalletsMetadata(
          `${appInitData.url ?? 'https://nc2.nightly.app'}/get_wallets_metadata`
        ).catch(() => [] as WalletMetadata[])
      ])
    }
  }

  public static build = async (
    appInitData: AppSelectorInitialize,
    useEagerConnect?: boolean,
    anchorRef?: HTMLElement | null,
    uiOverrides?: {
      variablesOverride?: object
      stylesOverride?: string
      qrConfigOverride?: Partial<XMLOptions>
    }
  ) => {
    if (!useEagerConnect) {
      clearSessionIdForNetwork(appInitData.network)
    }

    const adapter = new NightlyConnectAdapter(appInitData, useEagerConnect)

    adapter.walletsList = getPolkadotWalletsList(
      [],
      getRecentStandardWalletForNetwork(adapter.network) ?? undefined
    )
    adapter._modal = new NightlyConnectSelectorModal(
      adapter.walletsList,
      appInitData.url ?? 'https://nc2.nightly.app',
      networkToData(adapter.network),
      anchorRef,
      uiOverrides?.variablesOverride,
      uiOverrides?.stylesOverride,
      uiOverrides?.qrConfigOverride
    )

    const [app, metadataWallets] = await NightlyConnectAdapter.initApp(appInitData)

    adapter._app = app
    adapter._metadataWallets = metadataWallets

    adapter.walletsList = getPolkadotWalletsList(
      metadataWallets,
      getRecentStandardWalletForNetwork(adapter.network) ?? undefined
    )

    return adapter
  }

  public static buildLazy = (
    appInitData: AppSelectorInitialize,
    useEagerConnect?: boolean,
    anchorRef?: HTMLElement | null,
    uiOverrides?: {
      variablesOverride?: object
      stylesOverride?: string
      qrConfigOverride?: Partial<XMLOptions>
    }
  ) => {
    if (!useEagerConnect) {
      clearSessionIdForNetwork(appInitData.network)
    }

    const adapter = new NightlyConnectAdapter(appInitData, useEagerConnect)

    adapter.walletsList = getPolkadotWalletsList(
      [],
      getRecentStandardWalletForNetwork(adapter.network) ?? undefined
    )
    adapter._modal = new NightlyConnectSelectorModal(
      adapter.walletsList,
      appInitData.url ?? 'https://nc2.nightly.app',
      networkToData(adapter.network),
      anchorRef,
      uiOverrides?.variablesOverride,
      uiOverrides?.stylesOverride,
      uiOverrides?.qrConfigOverride
    )

    adapter._loading = true

    NightlyConnectAdapter.initApp(appInitData)
      .then(([app, metadataWallets]) => {
        adapter._app = app
        adapter._metadataWallets = metadataWallets
        adapter.walletsList = getPolkadotWalletsList(
          metadataWallets,
          getRecentStandardWalletForNetwork(adapter.network) ?? undefined
        )

        adapter._loading = false
      })
      .catch(() => {
        adapter._loading = false
        throw new Error('Failed to initialize adapter')
      })

    return adapter
  }
  public static buildWithInitOnConnect = (
    appInitData: AppSelectorInitialize,
    useEagerConnect?: boolean,
    anchorRef?: HTMLElement | null,
    uiOverrides?: {
      variablesOverride?: object
      stylesOverride?: string
      qrConfigOverride?: Partial<XMLOptions>
    }
  ) => {
    if (!useEagerConnect) {
      clearSessionIdForNetwork(appInitData.network)
    }

    const adapter = new NightlyConnectAdapter(appInitData, useEagerConnect, true)

    adapter.walletsList = getPolkadotWalletsList(
      [],
      getRecentStandardWalletForNetwork(adapter.network) ?? undefined
    )
    adapter._modal = new NightlyConnectSelectorModal(
      adapter.walletsList,
      appInitData.url ?? 'https://nc2.nightly.app',
      networkToData(adapter.network),
      anchorRef,
      uiOverrides?.variablesOverride,
      uiOverrides?.stylesOverride,
      uiOverrides?.qrConfigOverride
    )

    return adapter
  }
  // ensureLoaded = async () => {}
  canEagerConnect = async () => {
    if (!this._useEagerConnect) {
      return false
    }

    // utility for case if somebody wants to fire connect asap, but doesn't want to show modal if e. g. there was no user connected on the device yet
    if (this._loading) {
      for (let i = 0; i < 200; i++) {
        await sleep(10)

        if (!this._loading) {
          break
        }
      }
    }

    if (this._loading) {
      false
    }

    if (this._app && this._app.hasBeenRestored() && this._app.accounts.activeAccounts.length > 0) {
      return true
    }

    if (
      getRecentStandardWalletForNetwork(this.network) !== null &&
      isStandardConnectedForNetwork(this.network)
    ) {
      return true
    }

    return false
  }

  eagerConnectDeeplink = () => {
    if (isMobileBrowser() && this._app) {
      const mobileWalletName = getRecentStandardWalletForNetwork(this.network)
      const wallet = this.walletsList.find((w) => w.name === mobileWalletName)

      if (typeof wallet === 'undefined') {
        return
      }

      if (wallet.deeplink === null) {
        return
      }

      if (wallet.deeplink.native !== null) {
        this._app.connectDeeplink({
          walletName: wallet.name,
          url: wallet.deeplink.native
        })

        return
      }

      if (wallet.deeplink.universal !== null) {
        this._app.connectDeeplink({
          walletName: wallet.name,
          url: wallet.deeplink.universal
        })
      }
    }
  }

  connectToMobileWallet = (walletName: string) => {
    if (this._modal) {
      this._modal.setStandardWalletConnectProgress(true)
    }

    const wallet = this.walletsList.find((w) => w.name === walletName)

    if (!this._app || typeof wallet === 'undefined') {
      return
    }

    if (wallet.deeplink === null) {
      return
    }

    if (wallet.deeplink.native !== null) {
      this._app.connectDeeplink({
        walletName: wallet.name,
        url: wallet.deeplink.native
      })

      this._chosenMobileWalletName = walletName

      triggerConnect(
        wallet.deeplink.native,
        this._app.sessionId,
        this._appInitData.url ?? 'https://nc2.nightly.app'
      )

      return
    }

    if (wallet.deeplink.universal !== null) {
      this._app.connectDeeplink({
        walletName: wallet.name,
        url: wallet.deeplink.universal
      })

      this._chosenMobileWalletName = walletName

      triggerConnect(
        wallet.deeplink.universal,
        this._app.sessionId,
        this._appInitData.url ?? 'https://nc2.nightly.app'
      )
      return
    }
  }

  connectToStandardWallet = async (walletName: string, onSuccess: () => void) => {
    try {
      if (this._modal) {
        this._modal.setStandardWalletConnectProgress(true)
      }
      const adapter = this.walletsList.find((w) => w.name === walletName)?.injectedWallet
      if (typeof adapter === 'undefined') {
        if (this._modal) {
          this._modal.setStandardWalletConnectProgress(false)
        }
        throw new Error('Wallet not found')
      }
      // @ts-expect-error we want to pass network to enable
      const inject = await adapter!.enable!('Nightly Connect', this.network) // TODO should we also use connect?

      // Assert that there is at least one account
      if ((await inject.accounts.get()).length <= 0) {
        throw new Error('No accounts found')
      }

      persistRecentStandardWalletForNetwork(walletName, this.network)
      persistStandardConnectForNetwork(this.network)
      this._innerStandardAdapter = {
        ...inject,
        signer: {
          ...inject.signer,
          signPayload: inject.signer.signPayload
            ? // @ts-expect-error We want to also pass network to signPayload
              (payload) => inject.signer.signPayload!(payload, this.network)
            : undefined,
          signRaw: inject.signer.signRaw
            ? // @ts-expect-error We want to also pass network to signPayload
              (payload) => inject.signer.signRaw!(payload, this.network)
            : undefined
        }
      }
      this._connected = true
      this._connecting = false
      this._modal?.closeModal()
      onSuccess()
    } catch {
      // clear recent wallet
      persistStandardDisconnectForNetwork(this.network)
      if (this._modal) {
        this._modal.setStandardWalletConnectProgress(false)
      }
    }
  }

  connect = async () =>
    new Promise<void>((resolve, reject) => {
      const innerConnect = async () => {
        try {
          if (this.connected || this.connecting) {
            resolve()
            return
          }

          if (this._initOnConnect) {
            this._connecting = true

            if (!this._app) {
              try {
                const [app, metadataWallets] = await NightlyConnectAdapter.initApp(
                  this._appInitData
                )

                this._app = app
                this._metadataWallets = metadataWallets

                this.walletsList = getPolkadotWalletsList(
                  metadataWallets,
                  getRecentStandardWalletForNetwork(this.network) ?? undefined
                )
              } catch (e) {
                this._connecting = false
                if (!this._app) {
                  throw new Error('Wallet not ready')
                }
                throw e
              }
            }
          } else {
            if (this._loading) {
              // we do it to ensure proper connect flow in case if adapter is lazily built, but e. g. polkadot wallets selector uses its own eager connect
              for (let i = 0; i < 200; i++) {
                await sleep(10)

                if (!this._loading) {
                  break
                }
              }

              if (this._loading) {
                throw new Error('Wallet not ready')
              }
            }

            if (!this._app) {
              throw new Error('Wallet not ready')
            }

            this._connecting = true
          }

          if (this._app.hasBeenRestored() && this._app.accounts.activeAccounts.length > 0) {
            // Try to eager connect if session is restored
            try {
              this.eagerConnectDeeplink()
              this._connected = true
              this._connecting = false
              this._appSessionActive = true
              resolve()
              return
            } catch (error) {
              // If we fail because of whatever reason
              // Reset session since it might be corrupted
              const [app] = await NightlyConnectAdapter.initApp(this._appInitData)
              this._app = app
            }
          }

          const recentName = getRecentStandardWalletForNetwork(this.network)
          if (
            this._useEagerConnect &&
            recentName !== null &&
            isStandardConnectedForNetwork(this.network)
          ) {
            await this.connectToStandardWallet(recentName, resolve)

            if (this._connected) {
              return
            }
          }
          this._app.on('userConnected', () => {
            try {
              if (this._chosenMobileWalletName) {
                persistRecentStandardWalletForNetwork(this._chosenMobileWalletName, this.network)
              } else {
                clearRecentStandardWalletForNetwork(this.network)
              }
              if (!this._app || this._app.accounts.activeAccounts.length <= 0) {
                this._connecting = false
                // If user does not pass any accounts, we should disconnect
                this.disconnect()
              }
              this._connected = true
              this._connecting = false
              this._appSessionActive = true
              this._modal?.closeModal()
              resolve()
            } catch {
              this.disconnect()
            }
          })

          if (this._modal) {
            this._modal.onClose = () => {
              if (this._connecting) {
                this._connecting = false
                const error = new Error('Connection cancelled')
                reject(error)
              }
            }
            this._modal.openModal(this._app.sessionId, (walletName) => {
              if (
                isMobileBrowser() &&
                !this.walletsList.find((w) => w.name === walletName)?.injectedWallet
              ) {
                this.connectToMobileWallet(walletName)
              } else {
                this.connectToStandardWallet(walletName, resolve)
              }
            })
          }
          // eslint-disable-next-line @typescript-eslint/no-explicit-any
        } catch (error: any) {
          this._connecting = false
          reject(error)
        }
      }

      innerConnect()
    })

  disconnect = async () => {
    try {
      // Some apps might use disconnect to reset state / recreate session
      clearSessionIdForNetwork(this.network)
      this._appSessionActive = false
      this._app = await AppPolkadot.build(this._appInitData)
      if (this._innerStandardAdapter) {
        this._innerStandardAdapter = undefined
        persistStandardDisconnectForNetwork(this.network)
      }
      // Update recent wallet
      this.walletsList = getPolkadotWalletsList(
        this._metadataWallets,
        getRecentStandardWalletForNetwork(this.network) ?? undefined
      )
      if (this._modal) {
        this._modal.walletsList = this.walletsList
      }
      this._connected = false
    } finally {
      this._connecting = false
    }
  }
}
