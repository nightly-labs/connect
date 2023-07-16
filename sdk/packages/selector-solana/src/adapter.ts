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
  QueryNetwork,
  triggerConnect,
  persistStandardDisconnectForNetwork,
  sleep
} from '@nightlylabs/wallet-selector-base'
import {
  BaseMessageSignerWalletAdapter,
  WalletAdapterCompatibleStandardWallet,
  WalletName,
  WalletNotConnectedError,
  WalletNotReadyError,
  WalletReadyState,
  WalletSignTransactionError,
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
      : WalletReadyState.NotDetected

  private _app: AppSolana | undefined
  private _appSessionActive: boolean
  private _innerStandardAdapter: StandardWalletAdapter | undefined
  private _modal: NightlyConnectSelectorModal | undefined

  private _appInitData: AppInitData
  private _eagerConnectForStandardWallets: boolean

  private _walletsList: IWalletListItem[] = []

  private _chosenMobileWalletName: string | undefined

  private _loading: boolean

  constructor(appInitData: AppInitData, eagerConnectForStandardWallets?: boolean) {
    super()
    this._connecting = false
    this._connected = false
    this._publicKey = null
    this._appInitData = appInitData
    this._eagerConnectForStandardWallets = !!eagerConnectForStandardWallets
    this._appSessionActive = false
    this._loading = false
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

  public static async build(
    appInitData: AppInitData,
    eagerConnectForStandardWallets?: boolean,
    anchorRef?: HTMLElement | null
  ) {
    const adapter = new NightlyConnectAdapter(appInitData, eagerConnectForStandardWallets)

    if (adapter._readyState === WalletReadyState.Unsupported) {
      return adapter
    }

    adapter._readyState = WalletReadyState.Installed

    const [app, metadataWallets] = await Promise.all([
      AppSolana.build(appInitData),
      AppSolana.getWalletsMetadata('https://nc2.nightly.app/get_wallets_metadata')
        .then((list) =>
          list.map((wallet) => ({
            name: wallet.name,
            icon: wallet.image.default,
            deeplink: wallet.mobile,
            link: wallet.homepage
          }))
        )
        .catch(() => [] as MetadataWallet[])
    ])

    adapter._app = app

    adapter._walletsList = getWalletsList(
      metadataWallets,
      solanaWalletsFilter,
      getRecentStandardWalletForNetwork(SOLANA_NETWORK) ?? undefined
    )

    adapter._modal = new NightlyConnectSelectorModal(
      adapter._walletsList,
      appInitData.url ?? 'https://nc2.nightly.app',
      {
        network: QueryNetwork.SOLANA,
        name: SOLANA_NETWORK,
        icon: 'https://assets.coingecko.com/coins/images/4128/small/solana.png'
      },
      anchorRef,
      undefined,
      () => {
        adapter._connecting = false
      }
    )

    return adapter
  }

  public static buildLazy(
    appInitData: AppInitData,
    eagerConnectForStandardWallets?: boolean,
    anchorRef?: HTMLElement | null
  ) {
    const adapter = new NightlyConnectAdapter(appInitData, eagerConnectForStandardWallets)

    if (adapter._readyState === WalletReadyState.Unsupported) {
      return adapter
    }

    adapter._loading = true

    adapter._readyState = WalletReadyState.Installed

    Promise.all([
      AppSolana.build(appInitData),
      AppSolana.getWalletsMetadata('https://nc2.nightly.app/get_wallets_metadata')
        .then((list) =>
          list.map((wallet) => ({
            name: wallet.name,
            icon: wallet.image.default,
            deeplink: wallet.mobile,
            link: wallet.homepage
          }))
        )
        .catch(() => [] as MetadataWallet[])
    ]).then(([app, metadataWallets]) => {
      adapter._app = app

      adapter._walletsList = getWalletsList(
        metadataWallets,
        solanaWalletsFilter,
        getRecentStandardWalletForNetwork(SOLANA_NETWORK) ?? undefined
      )

      adapter._modal = new NightlyConnectSelectorModal(
        adapter._walletsList,
        appInitData.url ?? 'https://nc2.nightly.app',
        {
          network: QueryNetwork.SOLANA,
          name: SOLANA_NETWORK,
          icon: 'https://assets.coingecko.com/coins/images/4128/small/solana.png'
        },
        anchorRef,
        undefined,
        () => {
          adapter._connecting = false
        }
      )

      adapter._loading = false
    })

    return adapter
  }

  eagerConnectDeeplink = () => {
    if (isMobileBrowser() && this._app) {
      const mobileWalletName = getRecentStandardWalletForNetwork(SOLANA_NETWORK)
      const wallet = this._walletsList.find((w) => w.name === mobileWalletName)

      if (typeof wallet === 'undefined') {
        return
      }

      if (wallet.deeplink === null) {
        return
      }

      if (wallet.deeplink.universal !== null) {
        this._app.connectDeeplink({
          walletName: wallet.name,
          url: wallet.deeplink.universal
        })
        return
      }

      if (wallet.deeplink.native !== null) {
        this._app.connectDeeplink({
          walletName: wallet.name,
          url: wallet.deeplink.native
        })
      }
    }
  }

  connectToMobileWallet = (walletName: string) => {
    if (this._modal) {
      this._modal.setStandardWalletConnectProgress(true)
    }

    const wallet = this._walletsList.find((w) => w.name === walletName)

    if (!this._app || typeof wallet === 'undefined') {
      return
    }

    if (wallet.deeplink === null) {
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
    }
  }

  connectToStandardWallet = async (walletName: string) => {
    if (this._modal) {
      this._modal.setStandardWalletConnectProgress(true)
    }

    const wallet = this._walletsList.find((w) => w.name === walletName)
    if (typeof wallet?.standardWallet === 'undefined') {
      return
    }

    const adapter = new StandardWalletAdapter({
      wallet: wallet.standardWallet as WalletAdapterCompatibleStandardWallet
    })

    try {
      await adapter.connect()
      persistRecentStandardWalletForNetwork(walletName, SOLANA_NETWORK)
      persistStandardConnectForNetwork(SOLANA_NETWORK)
      this._innerStandardAdapter = adapter
      this._publicKey = adapter.publicKey
      this._connected = true
      this._connecting = false
      this.emit('connect', this._publicKey!)
      this._modal?.closeModal()
    } catch {
      if (this._modal) {
        this._modal.setStandardWalletConnectProgress(false)
      }
    }
  }

  async connect() {
    try {
      if (this._readyState !== WalletReadyState.Installed) throw new WalletNotReadyError()

      if (this._loading) { // we do it to ensure proper connect flow in case if adapter is lazily built, but e. g. solana wallets selector uses its own eager connect
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

      if (this.connected || this.connecting || !this._app) {
        return
      }

      this._connecting = true

      if (this._app.hasBeenRestored() && !!this._app.connectedPublicKeys.length) {
        this.eagerConnectDeeplink()
        this._publicKey = this._app.connectedPublicKeys[0]
        this._connected = true
        this._connecting = false
        this._appSessionActive = true
        this.emit('connect', this._publicKey)
        return
      }

      const recentName = getRecentStandardWalletForNetwork(SOLANA_NETWORK)
      if (
        this._eagerConnectForStandardWallets &&
        recentName !== null &&
        isStandardConnectedForNetwork(SOLANA_NETWORK)
      ) {
        await this.connectToStandardWallet(recentName)

        if (this._connected) {
          return
        }
      }

      this._app.on('userConnected', (e) => {
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
      })

      if (this._modal) {
        this._modal.openModal(this._app.sessionId, (walletName) => {
          if (
            isMobileBrowser() &&
            !this._walletsList.find((w) => w.name === walletName)?.standardWallet
          ) {
            this.connectToMobileWallet(walletName)
          } else {
            this.connectToStandardWallet(walletName)
          }
        })
      }
    } catch (error: any) {
      this._connecting = false

      this.emit('error', error)
      throw error
    }
  }

  async disconnect() {
    if (this.connected) {
      if (this._appSessionActive) {
        clearSessionIdForNetwork(SOLANA_NETWORK)
        this._appSessionActive = false
        AppSolana.build(this._appInitData).then(
          (app) => {
            this._app = app
          },
          (err) => {
            console.log(err)
          }
        )
      }
      if (this._innerStandardAdapter) {
        await this._innerStandardAdapter.disconnect()
        this._innerStandardAdapter = undefined
        persistStandardDisconnectForNetwork(SOLANA_NETWORK)
      }
      this._publicKey = null
      this._connected = false

      this.emit('disconnect')
    }
  }

  async signTransaction<T extends Transaction | VersionedTransaction>(transaction: T) {
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

  async signAllTransactions<T extends Transaction | VersionedTransaction>(transactions: T[]) {
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

  async signMessage(message: Uint8Array): Promise<Uint8Array> {
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
        throw new WalletSignTransactionError(error?.message, error)
      }
    } catch (error: any) {
      this.emit('error', error)
      throw error
    }
  }
}
