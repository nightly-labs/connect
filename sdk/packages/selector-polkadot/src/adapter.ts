/* eslint-disable @typescript-eslint/no-empty-function */
import {
  AppPolkadot,
  AppPolkadotInitialize,
  WalletMetadata
} from '@nightlylabs/nightly-connect-polkadot'
import {
  ConnectionOptions,
  ConnectionType,
  IWalletListItem,
  NightlyConnectSelectorModal,
  XMLOptions,
  clearRecentWalletForNetwork,
  clearSessionIdForNetwork,
  defaultConnectionOptions,
  getRecentWalletForNetwork,
  isMobileBrowser,
  logoBase64,
  persistRecentWalletForNetwork,
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
  private _connectionOptions: ConnectionOptions = defaultConnectionOptions

  private _app: AppPolkadot | undefined
  private _innerStandardAdapter: Injected | undefined
  private _modal: NightlyConnectSelectorModal | undefined

  private _appInitData: AppSelectorInitialize

  private _metadataWallets: WalletMetadata[] = []
  private _walletsList: IPolkadotWalletListItem[] = []

  private _chosenMobileWalletName: string | undefined

  private _loading: boolean

  constructor(appInitData: AppSelectorInitialize, connectionOptions?: ConnectionOptions) {
    this._connecting = false
    this._connected = false
    this._appInitData = appInitData
    this._loading = false
    this._connectionOptions = { ...this._connectionOptions, ...connectionOptions }
    // If not persistent, clear session id
    if (!this._appInitData.persistent) {
      clearSessionIdForNetwork(this._appInitData.network)
    }
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
  get walletsFromRegistry() {
    return this._metadataWallets
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

  get sessionId() {
    return this._app?.sessionId
  }

  get qrCode() {
    if (this._modal) return this._modal.qrCode
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
    connectionOptions?: ConnectionOptions,
    anchorRef?: HTMLElement | null,
    uiOverrides?: {
      variablesOverride?: object
      stylesOverride?: string
      qrConfigOverride?: Partial<XMLOptions>
    }
  ) => {
    const adapter = new NightlyConnectAdapter(appInitData, connectionOptions)

    adapter.walletsList = getPolkadotWalletsList(
      [],
      getRecentWalletForNetwork(adapter.network)?.walletName ?? undefined
    )
    if (!adapter._connectionOptions.disableModal) {
      adapter._modal = new NightlyConnectSelectorModal(
        adapter.walletsList as IWalletListItem[],
        appInitData.url ?? 'https://nc2.nightly.app',
        networkToData(adapter.network),
        anchorRef,
        uiOverrides?.variablesOverride,
        uiOverrides?.stylesOverride,
        uiOverrides?.qrConfigOverride
      )
    }
    const [app, metadataWallets] = await NightlyConnectAdapter.initApp(appInitData)

    adapter._app = app
    adapter._metadataWallets = metadataWallets

    adapter.walletsList = getPolkadotWalletsList(
      metadataWallets,
      getRecentWalletForNetwork(adapter.network)?.walletName ?? undefined
    )

    return adapter
  }

  public static buildLazy = (
    appInitData: AppSelectorInitialize,
    connectionOptions?: ConnectionOptions,
    anchorRef?: HTMLElement | null,
    uiOverrides?: {
      variablesOverride?: object
      stylesOverride?: string
      qrConfigOverride?: Partial<XMLOptions>
    }
  ) => {
    const adapter = new NightlyConnectAdapter(appInitData, connectionOptions)

    adapter.walletsList = getPolkadotWalletsList(
      [],
      getRecentWalletForNetwork(adapter.network)?.walletName ?? undefined
    )
    // Fetch wallets from registry
    adapter.fetchWalletsFromRegistry().then((metadataWallets) => {
      adapter._metadataWallets = metadataWallets
      adapter.walletsList = getPolkadotWalletsList(
        metadataWallets,
        getRecentWalletForNetwork(adapter.network)?.walletName ?? undefined
      )
    })

    if (!adapter._connectionOptions.disableModal) {
      adapter._modal = new NightlyConnectSelectorModal(
        adapter.walletsList as IWalletListItem[],
        appInitData.url ?? 'https://nc2.nightly.app',
        networkToData(adapter.network),
        anchorRef,
        uiOverrides?.variablesOverride,
        uiOverrides?.stylesOverride,
        uiOverrides?.qrConfigOverride
      )
    }

    // If init on connect is not enabled, we should initialize app
    if (!adapter._connectionOptions.initOnConnect) {
      adapter._loading = true
      NightlyConnectAdapter.initApp(appInitData)
        .then(([app, metadataWallets]) => {
          adapter._app = app
          adapter._metadataWallets = metadataWallets
          adapter.walletsList = getPolkadotWalletsList(
            metadataWallets,
            getRecentWalletForNetwork(adapter.network)?.walletName ?? undefined
          )

          adapter._loading = false
        })
        .catch(() => {
          adapter._loading = false
          throw new Error('Failed to initialize adapter')
        })
    }
    return adapter
  }

  // Checks if we can restore user session
  canEagerConnect = async () => {
    // If eager connect is disabled, we can't eager connect
    if (this._connectionOptions.disableEagerConnect) {
      return false
    }
    // Get recent wallet for network
    const recentWallet = getRecentWalletForNetwork(this.network)
    // If there is no recent wallet, we can't eager connect
    if (recentWallet === null) return false

    // If we user wallet standard, we can eager connect
    if (
      recentWallet.walletName !== null &&
      recentWallet.walletType === ConnectionType.WalletStandard
    ) {
      return true
    }

    // If we user nightly connect we need to make sure app is restored
    if (recentWallet.walletType === ConnectionType.Nightly) {
      if (this._connectionOptions.initOnConnect) {
        return false
      }
      // Wait for app to be restored
      if (this._loading) {
        for (let i = 0; i < 2000; i++) {
          await sleep(10)
          if (!this._loading) {
            break
          }
        }
      }
      // If app is restored and has active accounts, we can eager connect
      if (this._loading) {
        return false
      }
      if (
        this._app &&
        this._app.hasBeenRestored() &&
        this._app.accounts.activeAccounts.length > 0
      ) {
        return true
      }
    }

    return false
  }

  connectToMobileWallet = (walletName: string) => {
    try {
      if (this._modal) {
        this._modal.setStandardWalletConnectProgress(true)
      }

      const wallet = this.walletsList.find((w) => w.name === walletName)

      if (!this._app) {
        throw new Error('Wallet not ready')
      }

      if (typeof wallet === 'undefined') {
        throw new Error('Wallet not found')
      }

      if (wallet.mobile === null) {
        throw new Error('Deeplink not found')
      }

      // If we have a native deeplink, we should use it
      if (wallet.mobile.native !== null) {
        this._app.connectDeeplink({
          walletName: wallet.name,
          url: wallet.mobile.native
        })

        this._chosenMobileWalletName = walletName

        triggerConnect(
          wallet.mobile.native,
          this._app.sessionId,
          this._appInitData.url ?? 'https://nc2.nightly.app'
        )
        return
      }

      // If we have a universal deeplink, we should use it
      if (wallet.mobile.universal !== null) {
        this._app.connectDeeplink({
          walletName: wallet.name,
          url: wallet.mobile.universal
        })

        this._chosenMobileWalletName = walletName

        triggerConnect(
          wallet.mobile.universal,
          this._app.sessionId,
          this._appInitData.url ?? 'https://nc2.nightly.app'
        )
        return
      }
      // Fallback to redirecting to app browser
      // aka browser inside the app
      if (!wallet.mobile.redirectToAppBrowser) {
        const redirectToAppBrowser = wallet.mobile.redirectToAppBrowser
        if (redirectToAppBrowser !== null && redirectToAppBrowser.indexOf('{{url}}') > -1) {
          const url = redirectToAppBrowser.replace(
            '{{url}}',
            encodeURIComponent(window.location.toString())
          )

          window.open(url, '_blank', 'noreferrer noopener')

          return
        }
      }
    } catch (err) {
      // clear recent wallet
      clearRecentWalletForNetwork(this.network)
      if (this._modal) {
        this._modal.setStandardWalletConnectProgress(false)
      }
      throw err
    }
  }
  // Generic connect to standard wallet
  connectToStandardWallet = async (walletName: string) => {
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

      persistRecentWalletForNetwork(this.network, {
        walletName,
        walletType: ConnectionType.WalletStandard
      })

      this._modal?.closeModal()
    } catch (err) {
      // clear recent wallet
      clearRecentWalletForNetwork(this.network)
      if (this._modal) {
        this._modal.setStandardWalletConnectProgress(false)
      }
      throw err
    }
  }
  connectToWallet = async (walletName: string) => {
    if (isMobileBrowser() && !this.walletsList.find((w) => w.name === walletName)?.injectedWallet) {
      return this.connectToMobileWallet(walletName)
    } else {
      return await this.connectToStandardWallet(walletName)
    }
  }
  connect = async () =>
    new Promise<void>((resolve, reject) => {
      const innerConnect = async () => {
        try {
          if (this._connecting) {
            reject("Can't connect while connecting")
            return
          }
          if (this._connected) {
            resolve()
            return
          }

          const recentWallet = getRecentWalletForNetwork(this.network)
          if (!this._connectionOptions.disableEagerConnect && recentWallet !== null) {
            // Eager connect standard if possible
            if (recentWallet.walletType === ConnectionType.WalletStandard) {
              await this.connectToStandardWallet(recentWallet.walletName)
              resolve()
              return
            }
            // Eager connect remote if possible
            if (recentWallet.walletType === ConnectionType.Nightly) {
              if (this._app?.hasBeenRestored() && this._app.accounts.activeAccounts.length > 0) {
                // Try to eager connect if session is restored
                try {
                  this._connected = true
                  this._connecting = false
                  resolve()
                  return
                } catch (error) {
                  // If we fail because of whatever reason
                  // Reset session since it might be corrupted
                  const [app] = await NightlyConnectAdapter.initApp(this._appInitData)
                  this._app = app
                }
              }
            }
          }

          if (this._connectionOptions.disableModal) {
            reject('Modal is disabled')
            return
          }
          if (this._connectionOptions.initOnConnect) {
            this._loading = true
            NightlyConnectAdapter.initApp(this._appInitData)
              .then(([app, metadataWallets]) => {
                this._app = app
                this._metadataWallets = metadataWallets
                this.walletsList = getPolkadotWalletsList(
                  metadataWallets,
                  getRecentWalletForNetwork(this.network)?.walletName ?? undefined
                )
                this._loading = false
              })
              .catch(() => {
                this._loading = false
                throw new Error('Failed to initialize adapter')
              })
          }
          // Interval that checks if app has connected
          let loadingInterval: NodeJS.Timeout

          // opening modal and waiting for sessionId
          if (this._modal) {
            this._connecting = true
            this._modal.onClose = () => {
              clearInterval(loadingInterval)
              if (this._connecting) {
                this._connecting = false
                const error = new Error('Connection cancelled')
                reject(error)
              }
            }
            this._modal.openModal(this._app?.sessionId ?? undefined, async (walletName: string) => {
              // If we are on mobile and wallet is not injected, we should connect to mobile wallet
              if (
                isMobileBrowser() &&
                !this.walletsList.find((w) => w.name === walletName)?.injectedWallet
              ) {
                this.connectToMobileWallet(walletName)
              } else {
                await this.connectToStandardWallet(walletName)
                resolve()
              }
            })

            // loop until app is connected or we timeout
            let checks = 0
            loadingInterval = setInterval(async (): Promise<void> => {
              checks++
              if (this._app) {
                // Clear interval if app is connected
                clearInterval(loadingInterval)
                if (this._modal) this._modal.sessionId = this._app.sessionId
                this._app.on('userConnected', () => {
                  try {
                    persistRecentWalletForNetwork(this.network, {
                      walletName: this._chosenMobileWalletName || '',
                      walletType: ConnectionType.Nightly
                    })

                    if (!this._app || this._app.accounts.activeAccounts.length <= 0) {
                      this._connecting = false
                      this._connected = false
                      // If user does not pass any accounts, we should disconnect
                      this.disconnect()
                      return
                    }
                    this._connected = true
                    this._connecting = false
                    this._modal?.closeModal()
                    resolve()
                  } catch {
                    this.disconnect()
                  }
                })
                return
              }

              // timeout after 5 seconds
              if (checks > 500) {
                clearInterval(loadingInterval)
                // reject(new Error('Connecting takes too long'))
                // TODO we need to have a way to show error on modal
              }
            }, 10)
          }
          // eslint-disable-next-line @typescript-eslint/no-explicit-any
        } catch (error: any) {
          this._connecting = false
          reject(error)
        } finally {
          this._connecting = false
        }
      }

      innerConnect()
    })
  fetchWalletsFromRegistry = async () => {
    this._metadataWallets = await AppPolkadot.getWalletsMetadata(
      `${this._appInitData.url ?? 'https://nc2.nightly.app'}/get_wallets_metadata`
    )
    return this._metadataWallets
  }
  fetchWalletList = async () => {
    const metadataWallets = await this.fetchWalletsFromRegistry()
    this.walletsList = getPolkadotWalletsList(
      metadataWallets,
      getRecentWalletForNetwork(this.network)?.walletName ?? undefined
    )
    return this.walletsList
  }

  disconnect = async () => {
    try {
      // Some apps might use disconnect to reset state / recreate session
      clearSessionIdForNetwork(this.network)
      clearRecentWalletForNetwork(this.network)
      this._innerStandardAdapter = undefined
      this._app = await AppPolkadot.build(this._appInitData)

      // Update recent wallet
      this.walletsList = getPolkadotWalletsList(
        this._metadataWallets,
        getRecentWalletForNetwork(this.network)?.walletName ?? undefined
      )
      if (this._modal) {
        this._modal.walletsList = this.walletsList as IWalletListItem[]
      }
      this._connected = false
    } finally {
      this._connecting = false
    }
  }
}
