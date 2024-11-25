import { AppSolana, SOLANA_NETWORK } from '@nightlylabs/nightly-connect-solana'
import {
  AppInitData,
  ConnectionOptions,
  ConnectionType,
  ISelectedWallet,
  IWalletListItem,
  NightlyConnectSelectorModal,
  WalletMetadata,
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
import {
  BaseMessageSignerWalletAdapter,
  WalletAdapterCompatibleStandardWallet,
  WalletAdapterEvents,
  WalletName,
  WalletReadyState,
  WalletWindowClosedError,
  isVersionedTransaction
} from '@solana/wallet-adapter-base'
import { StandardWalletAdapter } from '@solana/wallet-standard'
import { PublicKey, Transaction, TransactionVersion, VersionedTransaction } from '@solana/web3.js'
import { StandardEventsChangeProperties } from '@wallet-standard/core'
import { getSolanaWalletsList } from './detection'

type NightlyConnectAdapterEvents = WalletAdapterEvents & {
  change(properties: StandardEventsChangeProperties): void
}

// TODO refactor this to use interface instead of class
export class NightlyConnectAdapter extends BaseMessageSignerWalletAdapter {
  name = 'Nightly Connect' as WalletName<'Nightly Connect'>
  url = 'https://nightly.app'
  icon = logoBase64

  readonly supportedTransactionVersions: ReadonlySet<TransactionVersion> = new Set(['legacy', 0])
  // Is wallet currently connecting
  private _connecting: boolean
  // Is wallet currently connected
  private _connected: boolean
  // Public key of the connected wallet
  private _publicKey: PublicKey | null
  // Ready state of the wallet
  private _readyState: WalletReadyState =
    typeof window === 'undefined' || typeof document === 'undefined'
      ? WalletReadyState.Unsupported
      : WalletReadyState.Loadable
  // Remote app instance
  private _app: AppSolana | undefined
  // Remote app might be loading
  private _appLoading: boolean
  // What type of connection is used
  private _connectionType: ConnectionType | undefined
  // Inner standard adapter
  private _innerStandardAdapter: StandardWalletAdapter | undefined
  // Modal instance
  private _modal: NightlyConnectSelectorModal | undefined
  // Init data for remote app
  private _appInitData: AppInitData
  // Wallets metadata
  private _metadataWallets: WalletMetadata[] = []
  // List of wallets to display
  private _walletsList: IWalletListItem[] = []
  // Name of the wallet to be displayed on mobile
  private _chosenMobileWalletName: string | undefined

  // interval used for checking for wallets with delayed detection
  private _detectionIntervalId: NodeJS.Timeout | undefined
  // max number of tries to get delayed wallets
  private _maxNumberOfChecks = 10
  // Selected wallet
  private _selectedWallet: ISelectedWallet | undefined = undefined
  // Connection options
  private _connectionOptions: ConnectionOptions = defaultConnectionOptions
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  private _eventHandlers: Map<string, (...args: any[]) => void> = new Map()
  constructor(appInitData: AppInitData, connectionOptions?: ConnectionOptions) {
    super()
    this._connecting = false
    this._connected = false
    this._publicKey = null
    this._appInitData = appInitData
    if (appInitData.persistent !== false) this._appInitData.persistent = true

    this._appLoading = false
    this._connectionOptions = { ...this._connectionOptions, ...connectionOptions }
    // If not persistent, clear session id
    if (!this._appInitData.persistent) {
      clearSessionIdForNetwork(SOLANA_NETWORK)
    }
  }

  get connecting() {
    return this._connecting
  }

  get connected() {
    return this._connected
  }

  get readyState() {
    return this._readyState
  }

  get publicKey() {
    return this._publicKey
  }

  get selectedWallet() {
    return this._selectedWallet
  }

  set walletsList(list: IWalletListItem[]) {
    this._walletsList = list
    if (this._modal) {
      this._modal.walletsList = list
    }
  }
  get walletsList() {
    return this._walletsList
  }

  get sessionId() {
    return this._app?.sessionId
  }

  get qrCode() {
    return this._modal?.qrCode
  }

  public static initApp = async (
    appInitData: AppInitData
  ): Promise<[AppSolana, WalletMetadata[]]> => {
    try {
      return await Promise.all([
        AppSolana.build(appInitData),
        AppSolana.getWalletsMetadata(
          `${appInitData.url ?? 'https://nc2.nightly.app'}/get_wallets_metadata`
        )
      ])
    } catch {
      clearSessionIdForNetwork(SOLANA_NETWORK)
      return await Promise.all([
        AppSolana.build(appInitData),
        AppSolana.getWalletsMetadata(
          `${appInitData.url ?? 'https://nc2.nightly.app'}/get_wallets_metadata`
        )
      ])
    }
  }

  on<T extends keyof NightlyConnectAdapterEvents>(
    event: T,
    fn: NightlyConnectAdapterEvents[T] extends (...args: infer Args) => void
      ? (...args: Args) => void
      : never,
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    context?: any
  ): this {
    if (event === 'change') {
      this._eventHandlers.set('change', fn)
      return this
    } else {
      return super.on(event, fn, context)
    }
  }
  // TODO remove once we move to full interface
  emit<T extends keyof NightlyConnectAdapterEvents>(
    event: T,
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    ...args: any
  ): boolean {
    if (event === 'change') {
      this._eventHandlers.get('change')?.(...args)
    } else {
      super.emit(event, ...args)
    }
    return true
  }

  public static build = async (
    appInitData: AppInitData,
    connectionOptions?: ConnectionOptions,
    anchorRef?: HTMLElement | null,
    uiOverrides?: {
      variablesOverride?: object
      stylesOverride?: string
      qrConfigOverride?: Partial<XMLOptions>
    }
  ) => {
    const adapter = new NightlyConnectAdapter(appInitData, connectionOptions)

    try {
      if (adapter._readyState === WalletReadyState.Unsupported) {
        return adapter
      }

      adapter.walletsList = getSolanaWalletsList(
        [],
        getRecentWalletForNetwork(SOLANA_NETWORK)?.walletName ?? undefined
      )

      if (!adapter._connectionOptions.disableModal)
        adapter._modal = new NightlyConnectSelectorModal(
          adapter.walletsList,
          appInitData.url ?? 'https://nc2.nightly.app',
          {
            name: SOLANA_NETWORK,
            icon: 'https://assets.coingecko.com/coins/images/4128/small/solana.png'
          },
          anchorRef,
          uiOverrides?.variablesOverride,
          uiOverrides?.stylesOverride,
          uiOverrides?.qrConfigOverride
        )

      const [app, metadataWallets] = await NightlyConnectAdapter.initApp(appInitData)

      adapter._app = app
      adapter._metadataWallets = metadataWallets

      adapter.walletsList = getSolanaWalletsList(
        metadataWallets,
        getRecentWalletForNetwork(SOLANA_NETWORK)?.walletName ?? undefined
      )

      adapter.checkForArrivingWallets(metadataWallets)

      // Add event listener for userConnected
      app.on('userConnected', async () => {
        try {
          persistRecentWalletForNetwork(SOLANA_NETWORK, {
            walletName: adapter._chosenMobileWalletName || '',
            walletType: ConnectionType.Nightly
          })

          if (!adapter._app || adapter._app.connectedPublicKeys.length <= 0) {
            adapter._connected = false
            // If user does not pass any accounts, we should disconnect
            adapter.disconnect()
            return
          }
          adapter.setSelectedWallet({
            isRemote: true
          })
          adapter._publicKey = adapter._app.connectedPublicKeys[0]
          adapter._connected = true
          adapter._connectionType = ConnectionType.Nightly
          adapter.emit('connect', adapter._publicKey)
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
    appInitData: AppInitData,
    connectionOptions?: ConnectionOptions,
    anchorRef?: HTMLElement | null,
    uiOverrides?: {
      variablesOverride?: object
      stylesOverride?: string
      qrConfigOverride?: Partial<XMLOptions>
    }
  ) => {
    const adapter = new NightlyConnectAdapter(appInitData, connectionOptions)

    if (adapter._readyState === WalletReadyState.Unsupported) {
      return adapter
    }

    adapter.walletsList = getSolanaWalletsList(
      [],
      getRecentWalletForNetwork(SOLANA_NETWORK)?.walletName ?? undefined
    )

    // Fetch wallets from registry
    adapter.fetchWalletsFromRegistry().then((metadataWallets) => {
      adapter._metadataWallets = metadataWallets

      adapter.walletsList = getSolanaWalletsList(
        metadataWallets,
        getRecentWalletForNetwork(SOLANA_NETWORK)?.walletName ?? undefined
      )
    })

    if (!adapter._connectionOptions.disableModal)
      adapter._modal = new NightlyConnectSelectorModal(
        adapter.walletsList,
        appInitData.url ?? 'https://nc2.nightly.app',
        {
          name: SOLANA_NETWORK,
          icon: 'https://assets.coingecko.com/coins/images/4128/small/solana.png'
        },
        anchorRef,
        uiOverrides?.variablesOverride,
        uiOverrides?.stylesOverride,
        uiOverrides?.qrConfigOverride
      )

    // If init on connect is not enabled, we should initialize app
    if (!adapter._connectionOptions.initOnConnect) {
      adapter._appLoading = true

      NightlyConnectAdapter.initApp(appInitData)
        .then(([app, metadataWallets]) => {
          adapter._app = app
          adapter._metadataWallets = metadataWallets
          adapter._appLoading = false
          adapter.walletsList = getSolanaWalletsList(
            metadataWallets,
            getRecentWalletForNetwork(SOLANA_NETWORK)?.walletName ?? undefined
          )

          adapter.checkForArrivingWallets(metadataWallets)

          // Add event listener for userConnected
          app.on('userConnected', async () => {
            try {
              persistRecentWalletForNetwork(SOLANA_NETWORK, {
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
              adapter._publicKey = adapter._app.connectedPublicKeys[0]
              adapter._connected = true
              adapter._connectionType = ConnectionType.Nightly
              adapter.emit('connect', adapter._publicKey)
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

  // Checks if we can restore user session
  canEagerConnect = async () => {
    // If eager connect is disabled, we can't eager connect
    if (this._connectionOptions.disableEagerConnect) return false

    // Get recent wallet for network
    const recentWallet = getRecentWalletForNetwork(SOLANA_NETWORK)

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
      clearRecentWalletForNetwork(SOLANA_NETWORK)
      if (this._modal) {
        this._modal.setStandardWalletConnectProgress(false)
      }
      this._selectedWallet = undefined

      throw err
    }
  }

  // Generic connect to standard wallet
  connectToStandardWallet = async (walletName: string) => {
    try {
      if (this._modal) {
        this._modal.setStandardWalletConnectProgress(true)
      }

      const wallet = this.walletsList.find((w) => w.name === walletName)
      this.setSelectedWallet({ wallet })
      const standardWallet = wallet?.standardWallet

      if (typeof standardWallet === 'undefined') {
        if (this._modal) {
          this._modal.setStandardWalletConnectProgress(false)
        }
        throw new Error('Wallet not found')
      }

      const adapter = new StandardWalletAdapter({
        wallet: standardWallet as WalletAdapterCompatibleStandardWallet
      })

      await adapter.connect()

      this._innerStandardAdapter = adapter
      this._publicKey = adapter.publicKey

      this._connected = true
      this._connecting = false
      this._connectionType = ConnectionType.WalletStandard
      this.emit('connect', this._publicKey!)
      // Subscribe to change event
      adapter.wallet.features['standard:events'].on('change', (a) => {
        this.emit('change', a)
      })
      persistRecentWalletForNetwork(SOLANA_NETWORK, {
        walletName,
        walletType: ConnectionType.WalletStandard
      })

      this._modal?.closeModal()
    } catch (err) {
      // clear recent wallet
      clearRecentWalletForNetwork(SOLANA_NETWORK)
      if (this._modal) {
        this._modal.setStandardWalletConnectProgress(false)
      }
      this._selectedWallet = undefined

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

  connect = async () =>
    new Promise<void>((resolve, reject) => {
      const innerConnect = async () => {
        try {
          if (this.connecting) {
            reject('Cannot connect while connecting')
            return
          }

          if (this.connected) {
            resolve()
            return
          }

          const recentWallet = getRecentWalletForNetwork(SOLANA_NETWORK)
          if (!this._connectionOptions.disableEagerConnect && recentWallet !== null) {
            // Eager connect standard if possible
            if (recentWallet.walletType === ConnectionType.WalletStandard) {
              await this.connectToStandardWallet(recentWallet.walletName)
              this._connectionType = ConnectionType.WalletStandard
              resolve()
              return
            }

            // Eager connect remote if possible
            if (recentWallet.walletType === ConnectionType.Nightly) {
              if (this._app?.hasBeenRestored() && this._app.connectedPublicKeys.length > 0) {
                // Try to eager connect if session is restored
                try {
                  this._publicKey = this._app.connectedPublicKeys[0]
                  this._connected = true
                  this._connecting = false
                  this._connectionType = ConnectionType.Nightly
                  this.setSelectedWallet({ isRemote: true })
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
                  this.emit('connect', this._publicKey)
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
            this._appLoading = true
            NightlyConnectAdapter.initApp(this._appInitData)
              .then(([app, metadataWallets]) => {
                this._app = app
                this._appLoading = false
                this._metadataWallets = metadataWallets

                this.walletsList = getSolanaWalletsList(
                  metadataWallets,
                  getRecentWalletForNetwork(SOLANA_NETWORK)?.walletName ?? undefined
                )

                this.checkForArrivingWallets(metadataWallets)

                // Add event listener for userConnected
                app.on('userConnected', async () => {
                  try {
                    persistRecentWalletForNetwork(SOLANA_NETWORK, {
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
                    this._publicKey = this._app.connectedPublicKeys[0]
                    this._connected = true
                    this._connectionType = ConnectionType.Nightly
                    this.emit('connect', this._publicKey)
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

          // Interval that checks if app has connected
          let loadingInterval: NodeJS.Timeout
          if (this._modal) {
            this._connecting = true
            this._modal.onClose = () => {
              clearInterval(loadingInterval)
              if (this._connecting) {
                this._connecting = false
                const error = new WalletWindowClosedError()
                this.emit('error', error)
                reject(error)
              }
            }
            this._modal.openModal(this._app?.sessionId ?? undefined, async (walletName) => {
              // If we are on mobile and wallet is not injected, we should connect to mobile wallet
              if (
                isMobileBrowser() &&
                !this.walletsList.find((w) => w.name === walletName)?.standardWallet
              ) {
                this.connectToMobileWallet(walletName)
              } else {
                try {
                  await this.connectToStandardWallet(walletName)
                  resolve()
                } catch (error) {
                  reject(error)
                }
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

                this._app.on('userConnected', async () => {
                  try {
                    if (!this._app || this._app.connectedPublicKeys.length <= 0) {
                      this._connected = false
                      reject(new Error('No accounts found'))
                    }
                    this._connected = true
                    this._modal?.closeModal()
                    resolve()
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
          }
        } catch (error: any) {
          this._connecting = false

          this.emit('error', error)
          reject(error)
        }
      }

      innerConnect()
    })

  fetchWalletsFromRegistry: () => Promise<WalletMetadata[]> = async () => {
    this._metadataWallets = await AppSolana.getWalletsMetadata(
      `${this._appInitData.url ?? 'https://nc2.nightly.app'}/get_wallets_metadata`
    )
    return this._metadataWallets
  }

  fetchAllWallets = async () => {
    const metadataWallets = await this.fetchWalletsFromRegistry()
    this.walletsList = getSolanaWalletsList(
      metadataWallets,
      getRecentWalletForNetwork(SOLANA_NETWORK)?.walletName ?? undefined
    )
    return this.walletsList
  }

  checkForArrivingWallets = (metadataWallets: WalletMetadata[]) => {
    clearInterval(this._detectionIntervalId)
    let checks = 0

    this._detectionIntervalId = setInterval(() => {
      if (checks >= this._maxNumberOfChecks || this.connected) {
        clearInterval(this._detectionIntervalId)
      }
      checks++
      this.walletsList = getSolanaWalletsList(
        metadataWallets,
        getRecentWalletForNetwork(SOLANA_NETWORK)?.walletName ?? undefined
      )
    }, 500)
  }

  disconnect = async () => {
    // Disconnect remote app and clear session
    if (this._connectionType === ConnectionType.Nightly) {
      clearSessionIdForNetwork(SOLANA_NETWORK)
      this._appLoading = true
      try {
        this._app = await AppSolana.build(this._appInitData)
        // Add event listener for userConnected
        this._app.on('userConnected', async () => {
          try {
            persistRecentWalletForNetwork(SOLANA_NETWORK, {
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
            this._publicKey = this._app.connectedPublicKeys[0]
            this._connected = true
            this._connectionType = ConnectionType.Nightly
            this.emit('connect', this._publicKey)
          } catch {
            this.disconnect()
          }
        })
      } catch (err) {
        console.log(err)
      } finally {
        this._appLoading = false
      }
    }
    // Disconnect standard wallet
    if (this._connectionType === ConnectionType.WalletStandard) {
      if (this._innerStandardAdapter) {
        await this._innerStandardAdapter.disconnect()
        this._innerStandardAdapter = undefined
        clearRecentWalletForNetwork(SOLANA_NETWORK)
      }
    }
    this.walletsList = getSolanaWalletsList(
      this._metadataWallets,
      getRecentWalletForNetwork(SOLANA_NETWORK)?.walletName ?? undefined
    )
    this._connectionType = undefined
    this._publicKey = null
    this._connected = false
    this._selectedWallet = undefined
    this.emit('disconnect')
    clearInterval(this._detectionIntervalId)
  }

  signTransaction = async <T extends Transaction | VersionedTransaction>(transaction: T) => {
    try {
      // Check if connection is established
      if (!this._connectionType) {
        throw new Error('Not connected')
      }
      // Check if remote connection is established
      if (this._connectionType === ConnectionType.Nightly) {
        // App needs to be ready here
        if (isVersionedTransaction(transaction)) {
          return (await this._app!.signVersionedTransaction(transaction)) as T
        } else {
          const signedVersioned = await this._app!.signTransaction(transaction)
          return Transaction.from(signedVersioned.serialize()) as T
        }
      }
      // Check if standard wallet is ready
      if (this._innerStandardAdapter) {
        return this._innerStandardAdapter.signTransaction!(transaction)
      }
      throw new Error('Unreachable')
    } catch (error: any) {
      this.emit('error', error)
      throw error
    }
  }

  signAllTransactions = async <T extends Transaction | VersionedTransaction>(transactions: T[]) => {
    try {
      // Check if connection is established
      if (!this._connectionType) {
        throw new Error('Not connected')
      }
      // Check if remote connection is established
      if (this._connectionType === ConnectionType.Nightly) {
        // App needs to be ready here
        if (isVersionedTransaction(transactions[0])) {
          return (await this._app!.signAllVersionedTransactions(
            transactions as VersionedTransaction[]
          )) as T[]
        } else {
          const signedVersioned = await this._app!.signAllTransactions(
            transactions as Transaction[]
          )
          return signedVersioned.map((t) => Transaction.from(t.serialize()) as T)
        }
      }
      // Check if standard wallet is ready
      if (this._innerStandardAdapter) {
        return await this._innerStandardAdapter.signAllTransactions!(transactions)
      }
      throw new Error('Unreachable')
    } catch (error: any) {
      this.emit('error', error)
      throw error
    }
  }

  signMessage = async (message: Uint8Array): Promise<Uint8Array> => {
    try {
      // Check if connection is established
      if (!this._connectionType) {
        throw new Error('Not connected')
      }
      // Check if remote connection is established
      if (this._connectionType === ConnectionType.Nightly) {
        // App needs to be ready here
        return await this._app!.signMessage(new TextDecoder().decode(message))
      }
      // Check if standard wallet is ready
      if (this._innerStandardAdapter) {
        return await this._innerStandardAdapter.signMessage!(message)
      }
      throw new Error('Unreachable')
    } catch (error: any) {
      this.emit('error', error)
      throw error
    }
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

  changeNetwork = async ({ genesisHash, url }: { genesisHash: string; url?: string }) => {
    try {
      // Check if connection is established
      if (!this._connectionType) {
        throw new Error('Not connected')
      }
      // check if we are connected with nightly wallet
      if (this.selectedWallet?.name !== 'Nightly') {
        throw new Error('Only supported on Nightly wallet')
      }
      // Check if remote connection is established
      if (this._connectionType === ConnectionType.Nightly) {
        const response = await this._app!.changeNetwork({ genesisHash, url })
        if (response.success) {
          return
        }

        throw new Error('Network change unsuccessful')
      }
      // @ts-expect-error Window not declared
      const nightlySolana = window.nightly?.solana

      await nightlySolana.changeNetwork({
        genesisHash,
        url
      })
    } catch (err) {
      this.emit('error', err)
      throw err
    }
  }
}
