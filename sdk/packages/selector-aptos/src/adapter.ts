/* eslint-disable @typescript-eslint/no-empty-function */
import {
  AccountInfo,
  AptosChangeNetworkMethod,
  AptosConnectMethod,
  AptosGetAccountMethod,
  AptosGetNetworkMethod,
  AptosSignMessageMethod,
  AptosSignTransactionMethod,
  AptosWallet,
  NetworkInfo,
  UserResponse,
  UserResponseStatus
} from '@aptos-labs/wallet-standard'

import { AnyRawTransaction } from '@aptos-labs/ts-sdk'
import { AptosSignAndSubmitTransactionMethod } from '@aptos-labs/wallet-standard'
import { AppAptos, APTOS_NETWORK, deserializeConnectData } from '@nightlylabs/nightly-connect-aptos'
import { AppBaseInitialize } from '@nightlylabs/nightly-connect-base'
import {
  AppAptosInitData,
  clearRecentWalletForNetwork,
  clearSessionIdForNetwork,
  ConnectionOptions,
  ConnectionType,
  defaultConnectionOptions,
  getRecentWalletForNetwork,
  ISelectedWallet,
  isMobileBrowser,
  IWalletListItem,
  logoBase64,
  NetworkData,
  NightlyConnectSelectorModal,
  persistRecentWalletForNetwork,
  sleep,
  triggerConnect,
  WalletMetadata,
  XMLOptions
} from '@nightlylabs/wallet-selector-base'
import EventEmitter from 'eventemitter3'
import { getAptosWalletsList } from './detection'

export type AptosAdapterEvents = {
  connect(accountInfo: AccountInfo): void
  disconnect(): void
  error(error: any): void
  accountChange(accountInfo: AccountInfo): void
  networkChange(networkInfo: NetworkInfo): void
}

export class NightlyConnectAptosAdapter extends EventEmitter<AptosAdapterEvents> {
  // TODO: add later "implements WalletAdapter"
  name = 'Nightly Connect' as const
  icon = logoBase64
  // Remote app instance
  private _app: AppAptos | undefined
  // Adapter for standard wallets
  private _innerStandardAdapter: AptosWallet | undefined
  // Remote app might be loading
  private _appLoading: boolean
  // Is wallet currently connecting
  private _connecting: boolean
  // Is wallet currently connected
  private _connected: boolean
  // Modal instance
  private _modal: NightlyConnectSelectorModal | undefined
  // Init data for remote app
  private _appInitData: AppBaseInitialize
  // Custom network from app init data
  private _network: string
  // List of wallets to display
  private _walletsList: IWalletListItem[] = []
  // Name of the wallet to be displayed on mobile
  private _chosenMobileWalletName: string | undefined
  // What type of connection is used
  private _connectionType: ConnectionType | undefined
  // Wallets metadata
  private _metadataWallets: WalletMetadata[] = []
  private _connectionOptions: ConnectionOptions = defaultConnectionOptions
  // Data from NC connection
  private _networkInfo: NetworkInfo | undefined
  private _accountInfo: AccountInfo | undefined

  // interval used for checking for wallets with delayed detection
  private _detectionIntervalId: NodeJS.Timeout | undefined
  // max number of tries to get delayed wallets
  private _maxNumberOfChecks = 10

  private _selectedWallet: ISelectedWallet | undefined = undefined

  get walletsList() {
    return this._walletsList
  }

  set walletsList(list: IWalletListItem[]) {
    this._walletsList = list
    if (this._modal) {
      this._modal.walletsList = list
    }
  }

  get selectedWallet() {
    return this._selectedWallet
  }

  get sessionId() {
    return this._app?.sessionId
  }

  get qrCode() {
    return this._modal?.qrCode
  }

  constructor(appInitData: AppAptosInitData, connectionOptions?: ConnectionOptions) {
    super()
    this._connecting = false
    this._connected = false
    this._network = appInitData.network ?? APTOS_NETWORK
    this._appInitData = { ...appInitData, network: appInitData.network ?? APTOS_NETWORK }
    if (appInitData.persistent !== false) this._appInitData.persistent = true
    this._appLoading = false
    this._connectionOptions = { ...this._connectionOptions, ...connectionOptions }

    // If not persistent, clear session id
    if (!this._appInitData.persistent) {
      clearSessionIdForNetwork(this._network)
    }
  }

  public static initApp = async (
    appInitData: AppBaseInitialize
  ): Promise<[AppAptos, WalletMetadata[]]> => {
    try {
      return await Promise.all([
        AppAptos.build(appInitData),
        AppAptos.getWalletsMetadata(
          `${appInitData.url ?? 'https://nc2.nightly.app'}/get_wallets_metadata`
        )
      ])
    } catch {
      clearSessionIdForNetwork(appInitData.network)
      return await Promise.all([
        AppAptos.build(appInitData),
        AppAptos.getWalletsMetadata(
          `${appInitData.url ?? 'https://nc2.nightly.app'}/get_wallets_metadata`
        )
      ])
    }
  }

  public static build = async (
    appInitData: AppAptosInitData,
    connectionOptions?: ConnectionOptions,
    anchorRef?: HTMLElement | null,
    uiOverrides?: {
      variablesOverride?: object
      stylesOverride?: string
      qrConfigOverride?: Partial<XMLOptions>
      networkDataOverride?: Partial<NetworkData>
    }
  ) => {
    const adapter = new NightlyConnectAptosAdapter(appInitData, connectionOptions)

    try {
      const network = appInitData.network ?? APTOS_NETWORK
      adapter._network = network
      adapter.walletsList = getAptosWalletsList(
        [],
        getRecentWalletForNetwork(network)?.walletName ?? undefined
      )

      if (!adapter._connectionOptions.disableModal)
        adapter._modal = new NightlyConnectSelectorModal(
          adapter.walletsList,
          appInitData.url ?? 'https://nc2.nightly.app',
          {
            name: uiOverrides?.networkDataOverride?.name ?? network,
            icon:
              uiOverrides?.networkDataOverride?.icon ??
              'https://registry.nightly.app/networks/aptos.png'
          },
          anchorRef,
          uiOverrides?.variablesOverride,
          uiOverrides?.stylesOverride,
          uiOverrides?.qrConfigOverride
        )

      const [app, metadataWallets] = await NightlyConnectAptosAdapter.initApp({
        ...appInitData,
        network
      })

      adapter._app = app
      adapter._metadataWallets = metadataWallets

      adapter.walletsList = getAptosWalletsList(
        metadataWallets,
        getRecentWalletForNetwork(network)?.walletName ?? undefined
      )

      adapter.checkForArrivingWallets(metadataWallets)

      // Add event listener for userConnected
      app.on('userConnected', async (accountInfo, networkInfo) => {
        try {
          persistRecentWalletForNetwork(network, {
            walletName: adapter._chosenMobileWalletName || '',
            walletType: ConnectionType.Nightly
          })

          if (!adapter._app || adapter._app.connectedPublicKeys.length <= 0) {
            adapter._connected = false
            // If user does not pass any accounts, we should disconnect
            adapter.disconnect()
            return
          }
          adapter.setSelectedWallet({ isRemote: true })
          adapter._accountInfo = accountInfo
          adapter._networkInfo = networkInfo
          adapter._connected = true
          adapter._connectionType = ConnectionType.Nightly
          adapter.emit('connect', accountInfo)
        } catch {
          adapter.disconnect()
        }
      })
    } catch {
      console.log('Error building adapter')
    }

    return adapter
  }

  public static buildLazy = (
    appInitData: AppAptosInitData,
    connectionOptions?: ConnectionOptions,
    anchorRef?: HTMLElement | null,
    uiOverrides?: {
      variablesOverride?: object
      stylesOverride?: string
      qrConfigOverride?: Partial<XMLOptions>
      networkDataOverride?: Partial<NetworkData>
    }
  ) => {
    const adapter = new NightlyConnectAptosAdapter(appInitData, connectionOptions)
    const network = appInitData.network ?? APTOS_NETWORK
    adapter._network = network

    adapter.walletsList = getAptosWalletsList(
      [],
      getRecentWalletForNetwork(network)?.walletName ?? undefined
    )

    // Fetch wallets from registry
    adapter.fetchWalletsFromRegistry().then((metadataWallets) => {
      adapter._metadataWallets = metadataWallets

      adapter.walletsList = getAptosWalletsList(
        metadataWallets,
        getRecentWalletForNetwork(network)?.walletName ?? undefined
      )
    })

    if (!adapter._connectionOptions.disableModal)
      adapter._modal = new NightlyConnectSelectorModal(
        adapter.walletsList,
        appInitData.url ?? 'https://nc2.nightly.app',
        {
          name: uiOverrides?.networkDataOverride?.name ?? network,
          icon:
            uiOverrides?.networkDataOverride?.icon ??
            'https://registry.nightly.app/networks/aptos.png'
        },
        anchorRef,
        uiOverrides?.variablesOverride,
        uiOverrides?.stylesOverride,
        uiOverrides?.qrConfigOverride
      )

    if (!adapter._connectionOptions.initOnConnect) {
      adapter._appLoading = true

      NightlyConnectAptosAdapter.initApp({ ...appInitData, network })
        .then(([app, metadataWallets]) => {
          adapter._app = app
          adapter._metadataWallets = metadataWallets
          adapter._appLoading = false

          adapter.walletsList = getAptosWalletsList(
            metadataWallets,
            getRecentWalletForNetwork(network)?.walletName ?? undefined
          )

          adapter.checkForArrivingWallets(metadataWallets)

          app.on('userConnected', async (accountInfo, networkInfo) => {
            try {
              persistRecentWalletForNetwork(network, {
                walletName: adapter._chosenMobileWalletName || '',
                walletType: ConnectionType.Nightly
              })

              if (!adapter._app || adapter._app.connectedPublicKeys.length <= 0) {
                adapter._connected = false
                // If user does not pass any accounts, we should disconnect
                adapter.disconnect()
                return
              }
              adapter.setSelectedWallet({ isRemote: true })
              adapter._accountInfo = accountInfo
              adapter._networkInfo = networkInfo
              adapter._connected = true
              adapter._connectionType = ConnectionType.Nightly
              adapter.emit('connect', accountInfo)
            } catch {
              adapter.disconnect()
            }
          })
        })
        .catch(() => {
          throw new Error('Failed to initialize adapter')
        })
    }

    return adapter
  }
  // Standard methods
  network: AptosGetNetworkMethod = async () => {
    if (!this) {
      throw new Error('Not connected')
    }
    if (this._connectionType === ConnectionType.Nightly) {
      return this._networkInfo!
    }
    if (this._connectionType === ConnectionType.WalletStandard) {
      return await this._innerStandardAdapter!.features['aptos:network'].network()
    }
    // Should not reach this point
    throw new Error('Invalid connection type')
  }

  changeNetwork: AptosChangeNetworkMethod = async (networkInfo: NetworkInfo) => {
    if (!this) {
      throw new Error('Not connected')
    }
    if (this._connectionType === ConnectionType.Nightly) {
      const response = await this._app!.changeNetwork(networkInfo)
      if (response.status === UserResponseStatus.APPROVED) {
        this._networkInfo = networkInfo
      }
      return response
    }
    if (this._connectionType === ConnectionType.WalletStandard) {
      return await this._innerStandardAdapter!.features['aptos:changeNetwork']!.changeNetwork(
        networkInfo
      )
    }
    throw new Error('Should not reach here')
  }

  account: AptosGetAccountMethod = async () => {
    if (!this) {
      throw new Error('Not connected')
    }
    if (this._connectionType === ConnectionType.Nightly) {
      return this._accountInfo!
    }
    if (this._connectionType === ConnectionType.WalletStandard) {
      return await this._innerStandardAdapter!.features['aptos:account'].account()
    }
    throw new Error('Should not reach here')
  }

  connect: AptosConnectMethod = async (silent?: boolean, networkInfo?: NetworkInfo) => {
    return new Promise<ReturnType<AptosConnectMethod>>((resolve, reject) => {
      const innerConnect = async () => {
        try {
          if (this._connecting) {
            reject('Cannot connect while connecting')
            return
          }

          if (this._connected) {
            // If we are connected, return the account
            const userInfo = {
              status: UserResponseStatus.APPROVED,
              args: await this.account()
            }
            resolve(Promise.resolve(userInfo))
            return
          }

          const recentWallet = getRecentWalletForNetwork(this._network)
          if (!this._connectionOptions.disableEagerConnect && recentWallet !== null) {
            // Eager connect standard if possible
            if (recentWallet.walletType === ConnectionType.WalletStandard) {
              const response = await this.connectToStandardWallet(
                recentWallet.walletName,
                silent,
                networkInfo
              )
              resolve(Promise.resolve(response))
              return
            }

            // Eager connect remote if possible
            if (recentWallet.walletType === ConnectionType.Nightly) {
              if (
                this._app?.hasBeenRestored() &&
                this._app.connectedPublicKeys.length > 0 &&
                this._app.base.clientMetadata !== undefined
              ) {
                // Try to eager connect if session is restored
                try {
                  // TODO add support for Secp256k1 key and features detection
                  const { accountInfo, networkInfo } = deserializeConnectData(
                    this._app.base.clientMetadata
                  )
                  this.setSelectedWallet({ isRemote: true })
                  this._accountInfo = accountInfo
                  this._networkInfo = networkInfo
                  this._connected = true
                  this._connecting = false
                  this._connectionType = ConnectionType.Nightly
                  // we only run it to reassign deeplinks on eager connect mobile browser (not on qr connection)
                  if (isMobileBrowser()) {
                    const wallet = this.walletsList.find((w) => w.name === 'Nightly')
                    if (wallet?.mobile) {
                      // If we have a native deeplink, we should use it
                      if (wallet.mobile?.native !== null) {
                        this._app.connectDeeplink({
                          walletName: wallet.name,
                          url: wallet.mobile.native
                        })
                      }
                      // If we have a universal deeplink, we should use it
                      else if (wallet.mobile?.universal !== null) {
                        this._app.connectDeeplink({
                          walletName: wallet.name,
                          url: wallet.mobile.universal
                        })
                      }
                    }
                  }
                  this.emit('connect', this._accountInfo)
                  resolve(
                    Promise.resolve({
                      status: UserResponseStatus.APPROVED,
                      args: this._accountInfo
                    })
                  )
                  return
                } catch (error) {
                  // If we fail because of whatever reason
                  // Reset session since it might be corrupted
                  const [app] = await NightlyConnectAptosAdapter.initApp(this._appInitData)
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
            this._appLoading = true
            NightlyConnectAptosAdapter.initApp(this._appInitData)
              .then(([app, metadataWallets]) => {
                this._app = app
                this._metadataWallets = metadataWallets
                this._appLoading = false
                this.walletsList = getAptosWalletsList(
                  metadataWallets,
                  getRecentWalletForNetwork(this._network)?.walletName ?? undefined
                )

                this.checkForArrivingWallets(metadataWallets)

                // Add event listener for userConnected
                app.on('userConnected', async (accountInfo, networkInfo) => {
                  try {
                    persistRecentWalletForNetwork(this._network, {
                      walletName: this._chosenMobileWalletName || '',
                      walletType: ConnectionType.Nightly
                    })

                    if (!this._app || this._app.connectedPublicKeys.length <= 0) {
                      this._connected = false
                      // If user does not pass any accounts, we should disconnect
                      this.disconnect()
                      return
                    }
                    this.setSelectedWallet({ isRemote: true })
                    this._accountInfo = accountInfo
                    this._networkInfo = networkInfo
                    this._connected = true
                    this._connectionType = ConnectionType.Nightly
                    this.emit('connect', accountInfo)
                  } catch {
                    this.disconnect()
                  }
                })
              })
              .catch(() => {
                this._appLoading = false
                throw new Error('Failed to initialize adapter')
              })
          }

          if (!this._modal) {
            this._connecting = false
            const error = new Error('Wallet not ready')
            this.emit('error', error)
            reject(error)
            return
          }

          // modal is defined here
          this._connecting = true
          this._modal.onClose = () => {
            if (this._connecting) {
              this._connecting = false
              const error = new Error('Connection cancelled')
              this.emit('error', error)
              reject(error)
              return
            }
          }

          // Try open
          const opened = this._modal!.openModal(
            this._app?.sessionId ?? undefined,
            async (walletName) => {
              if (
                isMobileBrowser() &&
                !this.walletsList.find((w) => w.name === walletName)?.standardWallet
              ) {
                this.connectToMobileWallet(walletName)
              } else {
                try {
                  const response = await this.connectToStandardWallet(
                    walletName,
                    silent,
                    networkInfo
                  )
                  resolve(Promise.resolve(response))
                } catch (error) {
                  reject(error)
                  return
                }
              }
            }
          )

          // If modal is not opened, reject
          // This might be caused by SSR
          if (!opened) {
            this._connecting = false
            const error = new Error('Failed to open modal')
            this.emit('error', error)
            reject(error)
          }

          // loop until app is connected or we timeout
          let checks = 0
          // Interval that checks if app has connected
          const loadingInterval = setInterval(async (): Promise<void> => {
            checks++
            if (this._app) {
              // Clear interval if app is connected
              clearInterval(loadingInterval)
              if (this._modal) this._modal.sessionId = this._app.sessionId

              this._app.on('userConnected', async (accountInfo, networkInfo) => {
                try {
                  if (!this._app || this._app.connectedPublicKeys.length <= 0) {
                    this._connected = false
                    reject(new Error('No accounts found'))
                  }
                  this._connected = true
                  this._connecting = false
                  this._connectionType = ConnectionType.Nightly
                  this._modal?.closeModal()
                  this._accountInfo = accountInfo
                  this._networkInfo = networkInfo
                  resolve(
                    Promise.resolve({ status: UserResponseStatus.APPROVED, args: accountInfo })
                  )
                  return
                } catch (error) {
                  reject(error)
                } finally {
                  this._connecting = false
                }
              })
              return
            }
            // timeout after 5 seconds
            if (checks > 500) {
              clearInterval(loadingInterval)
              // reject(new Error('Connecting takes too long'))
              if (this._modal) this._modal.timeoutError = 'Connecting is taking too long'
            }
          }, 10)
        } catch (error: unknown) {
          this._connecting = false
          this.emit('error', error)
          reject(error)
        }
      }

      innerConnect()
    })
  }
  disconnect = async () => {
    switch (this._connectionType) {
      case ConnectionType.Nightly: {
        clearSessionIdForNetwork(this._network)
        // Refresh app session
        this._app = await AppAptos.build(this._appInitData)
        this._selectedWallet = undefined

        // Add event listener for userConnected
        this._app.on('userConnected', async (accountInfo, networkInfo) => {
          try {
            persistRecentWalletForNetwork(this._network, {
              walletName: this._chosenMobileWalletName || '',
              walletType: ConnectionType.Nightly
            })

            if (!this._app || this._app.connectedPublicKeys.length <= 0) {
              this._connected = false
              // If user does not pass any accounts, we should disconnect
              this.disconnect()
              return
            }
            this.setSelectedWallet({ isRemote: true })
            this._accountInfo = accountInfo
            this._networkInfo = networkInfo
            this._connected = true
            this._connecting = false
            this._connectionType = ConnectionType.Nightly
            this.emit('connect', this._accountInfo)
          } catch {
            this.disconnect()
          }
        })

        break
      }
      case ConnectionType.WalletStandard: {
        if (this._innerStandardAdapter) {
          await this._innerStandardAdapter.features['aptos:disconnect'].disconnect()
          clearRecentWalletForNetwork(this._network)
          this.walletsList = getAptosWalletsList(
            this._metadataWallets,
            getRecentWalletForNetwork(this._network)?.walletName ?? undefined
          )
        }
        break
      }
    }
    this._innerStandardAdapter = undefined
    this._connectionType = undefined
    this._connected = false
    this._connecting = false
    this.emit('disconnect')
    clearInterval(this._detectionIntervalId)
  }

  signMessage: AptosSignMessageMethod = async (messageInput) => {
    try {
      if (!this._connectionType) {
        throw new Error('Wallet not ready')
      }
      switch (this._connectionType) {
        case ConnectionType.Nightly: {
          // App needs to be ready here
          return await this._app!.signMessage(messageInput)
        }
        case ConnectionType.WalletStandard: {
          if (!this._innerStandardAdapter) {
            throw new Error('Wallet not ready')
          }
          return await this._innerStandardAdapter.features['aptos:signMessage'].signMessage(
            messageInput
          )
        }
      }
    } catch (error) {
      this.emit('error', error)
      throw error
    }
  }

  signTransaction: AptosSignTransactionMethod = async (
    transaction: AnyRawTransaction,
    asFeePayer?: boolean
  ) => {
    try {
      if (!this._connectionType) {
        throw new Error('Wallet not ready')
      }
      switch (this._connectionType) {
        case ConnectionType.Nightly: {
          // App needs to be ready here
          return await this._app!.signTransaction(transaction, asFeePayer)
        }
        case ConnectionType.WalletStandard: {
          if (!this._innerStandardAdapter) {
            throw new Error('Wallet not ready')
          }
          return await this._innerStandardAdapter.features['aptos:signTransaction'].signTransaction(
            transaction,
            asFeePayer
          )
        }
      }
    } catch (error) {
      this.emit('error', error)
      throw error
    }
  }

  signAndSubmitTransaction: AptosSignAndSubmitTransactionMethod = async (transactionInput) => {
    try {
      if (!this._connectionType) {
        throw new Error('Wallet not ready')
      }
      switch (this._connectionType) {
        case ConnectionType.Nightly: {
          // App needs to be ready here
          return await this._app!.signAndSubmitTransaction(transactionInput)
        }
        case ConnectionType.WalletStandard: {
          if (!this._innerStandardAdapter) {
            throw new Error('Wallet not ready')
          }
          if (!this._innerStandardAdapter.features['aptos:signAndSubmitTransaction']) {
            throw new Error('Wallet does not support signAndSubmitTransaction')
          }
          return await this._innerStandardAdapter.features[
            'aptos:signAndSubmitTransaction'
          ].signAndSubmitTransaction(transactionInput)
        }
      }
    } catch (error) {
      this.emit('error', error)
      throw error
    }
  }

  canEagerConnect = async () => {
    // If eager connect is disabled, we can't eager connect
    if (this._connectionOptions.disableEagerConnect) return false

    // Get recent wallet for network
    const recentWallet = getRecentWalletForNetwork(this._network)

    // If there is no recent wallet, we can't eager connect
    if (recentWallet === null) return false

    // If we use wallet standard, we can eager connect
    if (
      recentWallet.walletName !== null &&
      recentWallet.walletType === ConnectionType.WalletStandard
    ) {
      return true
    }

    // If we use nightly connect we need to make sure app is restored
    if (recentWallet.walletType === ConnectionType.Nightly) {
      if (this._connectionOptions.initOnConnect) {
        return false
      }

      // Wait for app to be restored
      if (this._appLoading) {
        for (let i = 0; i < 200; i++) {
          await sleep(10)

          if (!this._appLoading) {
            break
          }
        }
      }

      if (this._appLoading) {
        return false
      }

      // If app is restored and has connected public keys, we can eager connect

      if (this._app && this._app.hasBeenRestored() && this._app.connectedPublicKeys.length > 0) {
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
      this.setSelectedWallet({ wallet })

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
    } catch (err) {
      clearRecentWalletForNetwork(this._network)
      if (this._modal) {
        this._modal.setStandardWalletConnectProgress(false)
      }
      throw err
    }
  }

  connectToStandardWallet = async (
    walletName: string,
    silent?: boolean,
    networkInfo?: NetworkInfo
  ): Promise<UserResponse<AccountInfo>> => {
    try {
      if (this._modal) {
        this._modal.setStandardWalletConnectProgress(true)
      }

      const wallet = this.walletsList.find((w) => w.name === walletName)
      this.setSelectedWallet({ wallet })

      if (typeof wallet?.standardWallet === 'undefined') {
        if (this._modal) {
          this._modal.setStandardWalletConnectProgress(false)
        }
        throw new Error('Wallet not found')
      }

      const adapter = wallet.standardWallet as AptosWallet

      const response = await adapter.features['aptos:connect'].connect(silent, networkInfo)
      if (response.status === UserResponseStatus.APPROVED) {
        this._innerStandardAdapter = adapter
        this._connected = true
        this._connecting = false
        this._connectionType = ConnectionType.WalletStandard
        this.emit('connect', response.args)
        adapter.features['aptos:onAccountChange'].onAccountChange((a) => {
          this.emit('accountChange', a)
        })
        adapter.features['aptos:onNetworkChange'].onNetworkChange((a) => {
          this.emit('networkChange', a)
        })
      } else {
        throw new Error('User rejected connection')
      }
      persistRecentWalletForNetwork(this._network, {
        walletName,
        walletType: ConnectionType.WalletStandard
      })

      this._modal?.closeModal()
      return response
    } catch (err) {
      if (this._modal) {
        this._modal.setStandardWalletConnectProgress(false)
      }
      clearRecentWalletForNetwork(this._network)
      this._connecting = false
      this.emit('error', err)
      throw err
    }
  }

  connectToWallet = async (walletName: string) => {
    if (isMobileBrowser() && !this.walletsList.find((w) => w.name === walletName)?.standardWallet) {
      this.connectToMobileWallet(walletName)
    } else {
      await this.connectToStandardWallet(walletName)
    }
  }

  fetchWalletsFromRegistry: () => Promise<WalletMetadata[]> = async () => {
    this._metadataWallets = await AppAptos.getWalletsMetadata(
      `${this._appInitData.url ?? 'https://nc2.nightly.app'}/get_wallets_metadata`
    )
    return this._metadataWallets
  }

  fetchAllWallets = async () => {
    const metadataWallets = await this.fetchWalletsFromRegistry()
    this.walletsList = getAptosWalletsList(
      metadataWallets,
      getRecentWalletForNetwork(this._network)?.walletName ?? undefined
    )
    return this.walletsList
  }

  checkForArrivingWallets = (metadataWallets: WalletMetadata[]) => {
    clearInterval(this._detectionIntervalId)
    let checks = 0

    this._detectionIntervalId = setInterval(() => {
      if (checks >= this._maxNumberOfChecks || this._connected) {
        clearInterval(this._detectionIntervalId)
      }
      checks++
      this.walletsList = getAptosWalletsList(
        metadataWallets,
        getRecentWalletForNetwork(this._network)?.walletName ?? undefined
      )
    }, 500)
  }

  setSelectedWallet = ({
    wallet,
    isRemote = false
  }: {
    wallet?: IWalletListItem
    isRemote?: boolean
  }) => {
    if (!wallet) {
      // Connecting to the nightly mobile app
      wallet = this.walletsList.find((wallet) => wallet.name === 'Nightly')
    }

    if (wallet) {
      this._selectedWallet = {
        name: wallet.name,
        image: wallet.image,
        homepage: wallet.homepage,
        walletType: isRemote ? 'mobile' : wallet.walletType
      }
    }
  }
}
