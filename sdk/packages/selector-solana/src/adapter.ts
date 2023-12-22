import { AppSolana, SOLANA_NETWORK } from '@nightlylabs/nightly-connect-solana'
import {
  AppInitData,
  clearRecentStandardWalletForNetwork,
  clearSessionIdForNetwork,
  getRecentStandardWalletForNetwork,
  getWalletsList,
  isMobileBrowser,
  IWalletListItem,
  logoBase64,
  MetadataWallet,
  NightlyConnectSelectorModal,
  persistRecentStandardWalletForNetwork,
  persistStandardConnectForNetwork,
  isStandardConnectedForNetwork,
  triggerConnect,
  persistStandardDisconnectForNetwork,
  sleep,
  XMLOptions
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
  private _eagerConnectForStandardWallets: boolean

  private _metadataWallets: MetadataWallet[] = []
  private _walletsList: IWalletListItem[] = []

  private _chosenMobileWalletName: string | undefined

  private _loading: boolean

  private _initOnConnect: boolean

  constructor(
    appInitData: AppInitData,
    eagerConnectForStandardWallets?: boolean,
    initOnConnect = false
  ) {
    super()
    this._connecting = false
    this._connected = false
    this._publicKey = null
    this._appInitData = appInitData
    this._eagerConnectForStandardWallets = !!eagerConnectForStandardWallets
    this._appSessionActive = false
    this._loading = false
    this._initOnConnect = initOnConnect
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
    eagerConnectForStandardWallets?: boolean,
    anchorRef?: HTMLElement | null,
    uiOverrides?: {
      variablesOverride?: object
      stylesOverride?: string
      qrConfigOverride?: Partial<XMLOptions>
    }
  ) => {
    const adapter = new NightlyConnectAdapter(appInitData, eagerConnectForStandardWallets)

    if (adapter._readyState === WalletReadyState.Unsupported) {
      return adapter
    }

    adapter.walletsList = getWalletsList(
      [],
      solanaWalletsFilter,
      getRecentStandardWalletForNetwork(SOLANA_NETWORK) ?? undefined
    )

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
      getRecentStandardWalletForNetwork(SOLANA_NETWORK) ?? undefined
    )

    return adapter
  }

  public static buildLazy = (
    appInitData: AppInitData,
    eagerConnectForStandardWallets?: boolean,
    anchorRef?: HTMLElement | null,
    uiOverrides?: {
      variablesOverride?: object
      stylesOverride?: string
      qrConfigOverride?: Partial<XMLOptions>
    }
  ) => {
    const adapter = new NightlyConnectAdapter(appInitData, eagerConnectForStandardWallets)

    if (adapter._readyState === WalletReadyState.Unsupported) {
      return adapter
    }

    adapter.walletsList = getWalletsList(
      [],
      solanaWalletsFilter,
      getRecentStandardWalletForNetwork(SOLANA_NETWORK) ?? undefined
    )

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

    adapter._loading = true

    NightlyConnectAdapter.initApp(appInitData).then(([app, metadataWallets]) => {
      adapter._app = app
      adapter._metadataWallets = metadataWallets

      adapter.walletsList = getWalletsList(
        metadataWallets,
        solanaWalletsFilter,
        getRecentStandardWalletForNetwork(SOLANA_NETWORK) ?? undefined
      )

      adapter._loading = false
    })

    return adapter
  }

  public static buildWithInitOnConnect = (
    appInitData: AppInitData,
    eagerConnectForStandardWallets?: boolean,
    anchorRef?: HTMLElement | null,
    uiOverrides?: {
      variablesOverride?: object
      stylesOverride?: string
      qrConfigOverride?: Partial<XMLOptions>
    }
  ) => {
    const adapter = new NightlyConnectAdapter(appInitData, eagerConnectForStandardWallets, true)

    if (adapter._readyState === WalletReadyState.Unsupported) {
      return adapter
    }

    adapter.walletsList = getWalletsList(
      [],
      solanaWalletsFilter,
      getRecentStandardWalletForNetwork(SOLANA_NETWORK) ?? undefined
    )

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

    return adapter
  }

  canEagerConnect = async () => {
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

    if (this._app && this._app.hasBeenRestored() && this._app.connectedPublicKeys.length > 0) {
      return true
    }

    if (
      this._eagerConnectForStandardWallets &&
      getRecentStandardWalletForNetwork(SOLANA_NETWORK) !== null &&
      isStandardConnectedForNetwork(SOLANA_NETWORK)
    ) {
      return true
    }

    return false
  }

  eagerConnectDeeplink = () => {
    if (isMobileBrowser() && this._app) {
      const mobileWalletName = getRecentStandardWalletForNetwork(SOLANA_NETWORK)
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
        return
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

      const wallet = this.walletsList.find((w) => w.name === walletName)
      if (typeof wallet?.standardWallet === 'undefined') {
        throw new Error('Wallet not found')
      }

      const adapter = new StandardWalletAdapter({
        wallet: wallet.standardWallet as WalletAdapterCompatibleStandardWallet
      })

      await adapter.connect()
      persistRecentStandardWalletForNetwork(walletName, SOLANA_NETWORK)
      persistStandardConnectForNetwork(SOLANA_NETWORK)
      this._innerStandardAdapter = adapter
      this._publicKey = adapter.publicKey
      this._connected = true
      this._connecting = false
      this.emit('connect', this._publicKey!)
      this._modal?.closeModal()
      onSuccess()
    } catch {
      // clear recent wallet
      persistStandardDisconnectForNetwork(SOLANA_NETWORK)
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

          if (this._readyState !== WalletReadyState.Loadable) throw new WalletNotReadyError()

          if (this._initOnConnect) {
            this._connecting = true

            if (!this._app) {
              try {
                const [app, metadataWallets] = await NightlyConnectAdapter.initApp(
                  this._appInitData
                )

                this._app = app
                this._metadataWallets = metadataWallets

                this.walletsList = getWalletsList(
                  metadataWallets,
                  solanaWalletsFilter,
                  getRecentStandardWalletForNetwork(SOLANA_NETWORK) ?? undefined
                )
              } catch {
                if (!this._app) {
                  this._connecting = false
                  throw new WalletNotReadyError()
                }
              }
            }
          } else {
            if (this._loading) {
              // we do it to ensure proper connect flow in case if adapter is lazily built, but e. g. solana wallets selector uses its own eager connect
              for (let i = 0; i < 200; i++) {
                await sleep(10)

                if (!this._loading) {
                  break
                }
              }

              if (this._loading) {
                throw new WalletNotReadyError()
              }
            }

            if (!this._app) {
              throw new WalletNotReadyError()
            }

            this._connecting = true
          }

          if (this._app.hasBeenRestored() && this._app.connectedPublicKeys.length > 0) {
            // Try to eager connect if session is restored
            try {
              this.eagerConnectDeeplink()
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

          const recentName = getRecentStandardWalletForNetwork(SOLANA_NETWORK)
          if (
            this._eagerConnectForStandardWallets &&
            recentName !== null &&
            isStandardConnectedForNetwork(SOLANA_NETWORK)
          ) {
            await this.connectToStandardWallet(recentName, resolve)

            if (this._connected) {
              return
            }
          }

          this._app.on('userConnected', (e) => {
            try {
              if (this._chosenMobileWalletName) {
                persistRecentStandardWalletForNetwork(this._chosenMobileWalletName, SOLANA_NETWORK)
              } else {
                clearRecentStandardWalletForNetwork(SOLANA_NETWORK)
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
          if (this._modal) {
            this._modal.onClose = () => {
              if (this._connecting) {
                this._connecting = false

                const error = new WalletWindowClosedError()

                this.emit('error', error)
                reject(error)
              }
            }
            this._modal.openModal(this._app.sessionId, (walletName) => {
              if (
                isMobileBrowser() &&
                !this.walletsList.find((w) => w.name === walletName)?.standardWallet
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

          this.emit('error', error)
          reject(error)
        }
      }

      innerConnect()
    })

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
        persistStandardDisconnectForNetwork(SOLANA_NETWORK)
      }
      this.walletsList = getWalletsList(
        this._metadataWallets,
        solanaWalletsFilter,
        getRecentStandardWalletForNetwork(SOLANA_NETWORK) ?? undefined
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
