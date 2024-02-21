import { AppSolana, SOLANA_NETWORK } from '@nightlylabs/nightly-connect-solana'
import {
  AppInitData,
  clearRecentWalletForNetwork,
  clearSessionIdForNetwork,
  getRecentWalletForNetwork,
  getWalletsList,
  isMobileBrowser,
  IWalletListItem,
  logoBase64,
  MetadataWallet,
  NightlyConnectSelectorModal,
  persistRecentWalletForNetwork,
  triggerConnect,
  sleep,
  XMLOptions,
  ConnectionType,
  ConnectionOptions,
  defaultConnectionOptions
} from '@nightlylabs/wallet-selector-base'
import {
  BaseMessageSignerWalletAdapter,
  WalletAdapterCompatibleStandardWallet,
  WalletName,
  WalletNotConnectedError,
  WalletNotReadyError,
  WalletReadyState,
  WalletSignMessageError,
  WalletSignTransactionError,
  WalletWindowClosedError,
  isVersionedTransaction
} from '@solana/wallet-adapter-base'
import { StandardWalletAdapter } from '@solana/wallet-standard'
import { PublicKey, Transaction, TransactionVersion, VersionedTransaction } from '@solana/web3.js'
import { solanaWalletsFilter } from './detection'

export class NightlyConnectAdapter extends BaseMessageSignerWalletAdapter {
  name = 'Nightly Connect' as WalletName<'Nightly Connect'>
  url = 'https://nightly.app'
  icon = logoBase64

  readonly supportedTransactionVersions: ReadonlySet<TransactionVersion> = new Set(['legacy', 0])

  private _connecting: boolean
  private _connected: boolean
  private _publicKey: PublicKey | null
  private _readyState: WalletReadyState =
    typeof window === 'undefined' || typeof document === 'undefined'
      ? WalletReadyState.Unsupported
      : WalletReadyState.Loadable

  private _app: AppSolana | undefined
  private _appSessionActive: boolean
  private _innerStandardAdapter: StandardWalletAdapter | undefined
  private _modal: NightlyConnectSelectorModal | undefined

  private _appInitData: AppInitData

  private _metadataWallets: MetadataWallet[] = []
  private _walletsList: IWalletListItem[] = []

  private _chosenMobileWalletName: string | undefined

  private _loading: boolean

  private _connectionOptions: ConnectionOptions = defaultConnectionOptions

  constructor(appInitData: AppInitData, connectionOptions?: ConnectionOptions) {
    super()
    this._connecting = false
    this._connected = false
    this._publicKey = null
    this._appInitData = appInitData
    if (appInitData.persistent !== false) this._appInitData.persistent = true

    this._appSessionActive = false
    this._loading = false
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

  get walletsList() {
    return this._walletsList
  }

  set walletsList(list: IWalletListItem[]) {
    this._walletsList = list
    if (this._modal) {
      this._modal.walletsList = list
    }
  }

  get sessionId() {
    return this._app?.sessionId
  }

  get qrCode() {
    return this._modal?.qrCode
  }

  public static initApp = async (
    appInitData: AppInitData
  ): Promise<[AppSolana, MetadataWallet[]]> => {
    try {
      return await Promise.all([
        AppSolana.build(appInitData),
        AppSolana.getWalletsMetadata(
          `${appInitData.url ?? 'https://nc2.nightly.app'}/get_wallets_metadata`
        )
          .then((list) =>
            list.map((wallet) => ({
              slug: wallet.slug,
              name: wallet.name,
              icon: wallet.image.default,
              deeplink: wallet.mobile,
              link: wallet.homepage,
              walletType: wallet.walletType
            }))
          )
          .catch(() => [] as MetadataWallet[])
      ])
    } catch {
      clearSessionIdForNetwork(SOLANA_NETWORK)
      return await Promise.all([
        AppSolana.build(appInitData),
        AppSolana.getWalletsMetadata(
          `${appInitData.url ?? 'https://nc2.nightly.app'}/get_wallets_metadata`
        )
          .then((list) =>
            list.map((wallet) => ({
              slug: wallet.slug,
              name: wallet.name,
              icon: wallet.image.default,
              deeplink: wallet.mobile,
              link: wallet.homepage,
              walletType: wallet.walletType
            }))
          )
          .catch(() => [] as MetadataWallet[])
      ])
    }
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

    if (adapter._readyState === WalletReadyState.Unsupported) {
      return adapter
    }

    adapter.walletsList = getWalletsList(
      [],
      solanaWalletsFilter,
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

    adapter.walletsList = getWalletsList(
      metadataWallets,
      solanaWalletsFilter,
      getRecentWalletForNetwork(SOLANA_NETWORK)?.walletName ?? undefined
    )

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

    adapter.walletsList = getWalletsList(
      [],
      solanaWalletsFilter,
      getRecentWalletForNetwork(SOLANA_NETWORK)?.walletName ?? undefined
    )

    // Fetch wallets from registry
    adapter.fetchWalletsFromRegistry().then((metadataWallets) => {
      adapter._metadataWallets = metadataWallets.map((wallet) => ({
        slug: wallet.slug,
        name: wallet.name,
        icon: wallet.image.default,
        deeplink: wallet.mobile,
        link: wallet.homepage,
        walletType: wallet.walletType
      }))

      adapter.walletsList = getWalletsList(
        [],
        solanaWalletsFilter,
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
      adapter._loading = true

      NightlyConnectAdapter.initApp(appInitData)
        .then(([app, metadataWallets]) => {
          adapter._app = app
          adapter._metadataWallets = metadataWallets

          adapter.walletsList = getWalletsList(
            metadataWallets,
            solanaWalletsFilter,
            getRecentWalletForNetwork(SOLANA_NETWORK)?.walletName ?? undefined
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
      if (this._loading) {
        for (let i = 0; i < 2000; i++) {
          await sleep(10)
          if (!this._loading) {
            break
          }
        }
      }

      if (this._loading) {
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

      if (!this._app) {
        throw new Error('Wallet not ready')
      }

      if (typeof wallet === 'undefined') {
        throw new Error('Wallet not found')
      }

      if (wallet.deeplink === null) {
        throw new Error('Deeplink not found')
      }

      // If we have a native deeplink, we should use it
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

      // If we have a universal deeplink, we should use it
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
    } catch (err) {
      clearRecentWalletForNetwork(SOLANA_NETWORK)
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

      const wallet = this.walletsList.find((w) => w.name === walletName)?.standardWallet
      if (typeof wallet === 'undefined') {
        if (this._modal) {
          this._modal.setStandardWalletConnectProgress(false)
        }
        throw new Error('Wallet not found')
      }

      const adapter = new StandardWalletAdapter({
        wallet: wallet as WalletAdapterCompatibleStandardWallet
      })

      await adapter.connect()

      this._innerStandardAdapter = adapter
      this._publicKey = adapter.publicKey

      this._connected = true
      this._connecting = false
      this.emit('connect', this._publicKey!)

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

      throw err
    }
  }

  connectToWallet = async (walletName: string) => {
    if (isMobileBrowser() && !this.walletsList.find((w) => w.name === walletName)?.standardWallet) {
      this.connectToMobileWallet(walletName)
    } else {
      this.connectToStandardWallet(walletName)
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

          if (this._readyState !== WalletReadyState.Loadable) throw new WalletNotReadyError()

          const recentWallet = getRecentWalletForNetwork(SOLANA_NETWORK)
          if (!this._connectionOptions.disableEagerConnect && recentWallet !== null) {
            // Eager connect standard if possible
            if (recentWallet.walletType === ConnectionType.WalletStandard) {
              await this.connectToStandardWallet(recentWallet.walletName)
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
                  this._appSessionActive = true
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
            this._loading = true
            NightlyConnectAdapter.initApp(this._appInitData)
              .then(([app, metadataWallets]) => {
                this._app = app
                this._metadataWallets = metadataWallets
                this.walletsList = getWalletsList(
                  metadataWallets,
                  solanaWalletsFilter,
                  getRecentWalletForNetwork(SOLANA_NETWORK)?.walletName ?? undefined
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

                this._app.on('userConnected', (e) => {
                  try {
                    persistRecentWalletForNetwork(SOLANA_NETWORK, {
                      walletName: this._chosenMobileWalletName || '',
                      walletType: ConnectionType.Nightly
                    })

                    if (!this._app || this._app.connectedPublicKeys.length <= 0) {
                      this._connecting = false
                      this._connected = false
                      // If user does not pass any accounts, we should disconnect
                      this.disconnect()
                      return
                    }

                    this._publicKey = new PublicKey(e.publicKeys[0])
                    this._connected = true
                    this._connecting = false
                    this._appSessionActive = true
                    this.emit('connect', this._publicKey)
                    this._modal?.closeModal()
                    resolve()
                  } catch {
                    this.disconnect()
                  }
                })
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

          this.emit('error', error)
          reject(error)
        } finally {
          this._connecting = false
        }
      }

      innerConnect()
    })

  fetchWalletsFromRegistry: () => ReturnType<typeof AppSolana.getWalletsMetadata> = async () => {
    return AppSolana.getWalletsMetadata(
      `${this._appInitData.url ?? 'https://nc2.nightly.app'}/get_wallets_metadata`
    )
  }

  disconnect = async () => {
    if (this.connected) {
      if (this._appSessionActive) {
        clearSessionIdForNetwork(SOLANA_NETWORK)
        this._appSessionActive = false
        this._loading = true
        try {
          this._app = await AppSolana.build(this._appInitData)
        } catch (err) {
          console.log(err)
        } finally {
          this._loading = false
        }
      }
      if (this._innerStandardAdapter) {
        await this._innerStandardAdapter.disconnect()
        this._innerStandardAdapter = undefined
        clearRecentWalletForNetwork(SOLANA_NETWORK)
      }
      this.walletsList = getWalletsList(
        this._metadataWallets,
        solanaWalletsFilter,
        getRecentWalletForNetwork(SOLANA_NETWORK)?.walletName ?? undefined
      )
      this._publicKey = null
      this._connected = false

      this.emit('disconnect')
    }
  }

  signTransaction = async <T extends Transaction | VersionedTransaction>(transaction: T) => {
    try {
      try {
        if (this._app && this._appSessionActive) {
          if (isVersionedTransaction(transaction)) {
            return (await this._app.signVersionedTransaction(transaction)) as T
          } else {
            const signedVersioned = await this._app.signTransaction(transaction)
            return Transaction.from(signedVersioned.serialize()) as T
          }
        } else if (this._innerStandardAdapter) {
          return this._innerStandardAdapter.signTransaction!(transaction)
        } else {
          throw new WalletNotConnectedError()
        }
      } catch (error: any) {
        throw new WalletSignTransactionError(error?.message, error)
      }
    } catch (error: any) {
      this.emit('error', error)
      throw error
    }
  }

  signAllTransactions = async <T extends Transaction | VersionedTransaction>(transactions: T[]) => {
    try {
      try {
        if (this._app && this._appSessionActive) {
          if (isVersionedTransaction(transactions[0])) {
            return (await this._app.signAllVersionedTransactions(
              transactions as VersionedTransaction[]
            )) as T[]
          } else {
            const signedVersioned = await this._app.signAllTransactions(
              transactions as Transaction[]
            )
            return signedVersioned.map((t) => Transaction.from(t.serialize()) as T)
          }
        } else if (this._innerStandardAdapter) {
          return await this._innerStandardAdapter.signAllTransactions!(transactions)
        } else {
          throw new WalletNotConnectedError()
        }
      } catch (error: any) {
        throw new WalletSignTransactionError(error?.message, error)
      }
    } catch (error: any) {
      this.emit('error', error)
      throw error
    }
  }

  signMessage = async (message: Uint8Array): Promise<Uint8Array> => {
    try {
      try {
        if (this._app && this._appSessionActive) {
          return await this._app.signMessage(new TextDecoder().decode(message))
        } else if (this._innerStandardAdapter) {
          return await this._innerStandardAdapter.signMessage!(message)
        } else {
          throw new WalletNotConnectedError()
        }
      } catch (error: any) {
        throw new WalletSignMessageError(error?.message, error)
      }
    } catch (error: any) {
      this.emit('error', error)
      throw error
    }
  }
}
