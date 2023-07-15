import { AppSolana } from '@nightlylabs/nightly-connect-solana'
import { AppInitData, logoBase64 } from '@nightlylabs/wallet-selector-base'
import {
  BaseMessageSignerWalletAdapter,
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

  private _appInitData: AppInitData

  constructor(appInitData: AppInitData) {
    super()
    this._connecting = false
    this._connected = false
    this._publicKey = null
    this._appInitData = appInitData
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
    const adapter = new NightlyConnectAdapter(appInitData)

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
    const adapter = new NightlyConnectAdapter(appInitData)

    if (adapter._readyState !== WalletReadyState.Unsupported) {
      // TODO - use this in then after build and get wallets finish
      adapter._readyState = WalletReadyState.Installed
      adapter.emit('readyStateChange', adapter._readyState)
    }

    return adapter
  }

  async connect() {
    return new Promise<void>((resolve, reject) => {
      try {
        if (this.connected || this.connecting) {
          resolve()
          return
        }
        if (this._readyState !== WalletReadyState.Loadable) throw new WalletNotReadyError()

        this._connecting = true

        if (!this._app) {
          AppSolana.build(this._appInitData)
            .then((app) => {
              this._app = app
              this._app.on('userConnected', (e) => {
                this._publicKey = new PublicKey(e.publicKeys[0])
                this._connected = true
                this._connecting = false
              })
            })
            .catch((error) => {
              this._connecting = false

              this.emit('error', error)
              reject(error)
            })
        } else {
          // this.modal.openModal(this._app.sessionId, NETWORK.SOLANA)
        }
      } catch (error: any) {
        this._connecting = false

        this.emit('error', error)
        reject(error)
      }
    })
  }

  async disconnect() {
    if (this.connected) {
      if (this._app) {
        this._app = undefined
        this._appSessionActive = false
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
