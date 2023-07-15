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
  persistStandardDisconnectForNetwork
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

  constructor(appInitData: AppInitData, eagerConnectForStandardWallets?: boolean) {
    super()
    this._connecting = false
    this._connected = false
    this._publicKey = null
    this._appInitData = appInitData
    this._eagerConnectForStandardWallets = !!eagerConnectForStandardWallets
    this._appSessionActive = false
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
    onCanEagerConnect?: (canEagerConnect: boolean) => void, // I've tried to do this using events, but was unable to because of inability to extend solana adapter events interface
    anchorRef?: HTMLElement
  ) {
    const adapter = new NightlyConnectAdapter(appInitData, eagerConnectForStandardWallets)

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

    adapter._readyState = WalletReadyState.Installed

    onCanEagerConnect?.(adapter.canEagerConnect())

    return adapter
  }

  public static buildLazy(
    appInitData: AppInitData,
    eagerConnectForStandardWallets?: boolean,
    onCanEagerConnect?: (canEagerConnect: boolean) => void, // I've tried to do this using events, but was unable to because of inability to extend solana adapter events interface
    anchorRef?: HTMLElement
  ) {
    const adapter = new NightlyConnectAdapter(appInitData, eagerConnectForStandardWallets)

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

      if (adapter._readyState !== WalletReadyState.Unsupported) {
        adapter._readyState = WalletReadyState.Installed

        adapter.emit('readyStateChange', adapter._readyState)
      }

      onCanEagerConnect?.(adapter.canEagerConnect())
    })

    return adapter
  }

  canEagerConnect = () => {
    if (
      this._app &&
      this._app.base.hasBeenRestored &&
      !!this._app.base.connectedPublicKeys.length
    ) {
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

  eagerConnectDeeplink = (network: string) => {
    if (isMobileBrowser() && this._app) {
      const mobileWalletName = getRecentStandardWalletForNetwork(network)
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
      if (this.connected || this.connecting || !this._app) {
        return
      }
      if (this._readyState !== WalletReadyState.Installed) throw new WalletNotReadyError()

      this._connecting = true

      if (this._app.base.hasBeenRestored && !!this._app.base.connectedPublicKeys.length) {
        this.eagerConnectDeeplink(SOLANA_NETWORK)
        this._publicKey = new PublicKey(this._app.base.connectedPublicKeys[0])
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
      if (this._app && this._appSessionActive) {
        clearSessionIdForNetwork(SOLANA_NETWORK)
        this._appSessionActive = false
        AppSolana.build(this._appInitData).then(
          (app) => {
            this._app === app
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
      if (!(this._app && this._appSessionActive) && !this._innerStandardAdapter) {
        throw new WalletNotConnectedError()
      }

      try {
        if (this._app && this._appSessionActive) {
          if (isVersionedTransaction(transaction)) {
            return (await this._app.signVersionedTransaction(transaction)) as T
          } else {
            const signedVersioned = await this._app.signTransaction(transaction)
            return Transaction.from(signedVersioned.serialize()) as T
          }
        } else {
          return this._innerStandardAdapter!.signTransaction!(transaction)
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
      if (!(this._app && this._appSessionActive) && !this._innerStandardAdapter) {
        throw new WalletNotConnectedError()
      }

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
        } else {
          return await this._innerStandardAdapter!.signAllTransactions!(transactions)
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
      if (!(this._app && this._appSessionActive) && !this._innerStandardAdapter) {
        throw new WalletNotConnectedError()
      }

      try {
        if (this._app && this._appSessionActive) {
          return await this._app.signMessage(new TextDecoder().decode(message))
        } else {
          return await this._innerStandardAdapter!.signMessage!(message)
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
