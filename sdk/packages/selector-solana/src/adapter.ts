import { AppSolana, SOLANA_NETWORK } from '@nightlylabs/nightly-connect-solana'
import {
  AppInitData,
  clearRecentStandardWalletForNetwork,
  clearSessionIdForNetwork,
  getRecentStandardWalletForNetwork,
  isDesktopConnectedForNetwork,
  isMobileBrowser,
  IWalletListItem,
  logoBase64,
  NightlyConnectSelectorModal,
  persistDesktopConnectForNetwork,
  persistRecentStandardWalletForNetwork,
  triggerConnect
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
    anchorRef?: HTMLElement,
    onOpen?: () => void,
    onClose?: () => void
  ) {
    const adapter = new NightlyConnectAdapter(appInitData, eagerConnectForStandardWallets)

    await adapter.eagerConnectToRecentStandardWallet()

    adapter._readyState = WalletReadyState.Installed

    return adapter
  }

  public static buildLazy(
    appInitData: AppInitData,
    eagerConnectForStandardWallets?: boolean,
    anchorRef?: HTMLElement,
    onOpen?: () => void,
    onClose?: () => void
  ) {
    const adapter = new NightlyConnectAdapter(appInitData, eagerConnectForStandardWallets)

    if (adapter._readyState !== WalletReadyState.Unsupported) {
      // TODO - use this in then after build and get wallets finish
      adapter.eagerConnectToRecentStandardWallet().then(() => {
        adapter._readyState = WalletReadyState.Installed

        adapter.emit('readyStateChange', adapter._readyState)
      })
    }

    return adapter
  }

  eagerConnectDeeplink = (network: string) => {
    if (isMobileBrowser() && this._app) {
      const mobileWalletName = getRecentStandardWalletForNetwork(network)
      const wallet = this._walletsList.find((w) => w.name === mobileWalletName)
      if (
        typeof wallet !== 'undefined' &&
        wallet.deeplink !== null &&
        (wallet.deeplink.universal !== null || wallet.deeplink.native !== null)
      ) {
        this._app.connectDeeplink({
          walletName: wallet.name,
          url: wallet.deeplink.universal ?? wallet.deeplink.native!
        })
      }
    }
  }

  connectToMobileWallet = (walletName: string) => {
    if (this._modal) {
      this._modal.setStandardWalletConnectProgress(true)
    }

    const walletData = this._walletsList.find((w) => w.name === walletName)

    if (
      !this._app ||
      typeof walletData === 'undefined' ||
      walletData.deeplink === null ||
      (walletData.deeplink.universal === null && walletData.deeplink.native === null)
    ) {
      return
    }

    this._app.connectDeeplink({
      walletName,
      url: walletData.deeplink.universal ?? walletData.deeplink.native!
    })

    this._chosenMobileWalletName = walletName

    triggerConnect(
      walletData.deeplink.universal ?? walletData.deeplink.native!,
      this._app.sessionId,
      this._appInitData.url ?? 'https://nc2.nightly.app'
    )
  }

  eagerConnectToRecentStandardWallet = async () => {
    const recentName = getRecentStandardWalletForNetwork(SOLANA_NETWORK)
    if (
      this._eagerConnectForStandardWallets &&
      recentName !== null &&
      isDesktopConnectedForNetwork(SOLANA_NETWORK)
    ) {
      await this.connectToStandardWallet(recentName, (adapter) => {
        this._innerStandardAdapter = adapter
      })
    }
  }

  connectToStandardWallet = async (
    walletName: string,
    onSuccess: (adapter: StandardWalletAdapter) => void
  ) => {
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

    await adapter
      .connect()
      .then(() => {
        onSuccess(adapter)
      })
      .catch(() => {
        if (this._modal) {
          this._modal.setStandardWalletConnectProgress(false)
        }
      })
  }

  async connect() {
    return new Promise<void>((resolve, reject) => {
      try {
        if (this.connected || this.connecting) {
          resolve()
          return
        }
        if (this._readyState !== WalletReadyState.Installed) throw new WalletNotReadyError()

        this._connecting = true

        if (this._app!.base.hasBeenRestored && !!this._app!.base.connectedPublicKeys.length) {
          this.eagerConnectDeeplink(SOLANA_NETWORK)
          this._publicKey = new PublicKey(this._app!.base.connectedPublicKeys[0])
          this._connected = true
          this._connecting = false
          this.emit('connect', this._publicKey)
          resolve()
          return
        } else if (this._innerStandardAdapter) {
          this._publicKey = this._innerStandardAdapter.publicKey
          this._connected = true
          this._connecting = false
          this.emit('connect', this._publicKey!)
          resolve()
          return
        }

        this._app!.on!('userConnected', (e) => {
          if (this._chosenMobileWalletName) {
            persistRecentStandardWalletForNetwork(this._chosenMobileWalletName, SOLANA_NETWORK)
          } else {
            clearRecentStandardWalletForNetwork(SOLANA_NETWORK)
          }
          this._publicKey = new PublicKey(e.publicKeys[0])
          this._connected = true
          this._connecting = false
          this.emit('connect', this._publicKey)
          resolve()
        })

        this._modal!.openModal(this._app!.sessionId, (walletName) => {
          if (isMobileBrowser()) {
            this.connectToMobileWallet(walletName)
          } else {
            this.connectToStandardWallet(walletName, (adapter) => {
              persistRecentStandardWalletForNetwork(walletName, SOLANA_NETWORK)
              persistDesktopConnectForNetwork(SOLANA_NETWORK)
              this._innerStandardAdapter = adapter
              this._publicKey = adapter.publicKey
              this._connected = true
              this._connecting = false
              this.emit('connect', this._publicKey!)
              resolve()
            })
          }
        })
      } catch (error: any) {
        this._connecting = false

        this.emit('error', error)
        reject(error)
      }
    })
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
