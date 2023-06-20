import { AppSolana, AppSolanaInitialize } from '@nightlylabs/nightly-connect-solana'
import '@nightlylabs/wallet-selector-modal'
import { NightlyModal } from '@nightlylabs/wallet-selector-modal'
import { StandardWalletAdapter } from '@solana/wallet-standard'
import { NightlyConnectSolanaWallet } from './wallet'
import { PublicKey } from '@solana/web3.js'
import { getSolanaWalletsList } from './detection'
import { getWallet } from '@nightlylabs/wallet-selector-base'
import { WalletAdapterCompatibleStandardWallet } from '@solana/wallet-adapter-base'

export class NCSolanaSelector {
  private _modal: NightlyModal | undefined
  private _app: AppSolana

  appInitData: AppSolanaInitialize

  onConnected: ((adapter: StandardWalletAdapter) => void) | undefined
  onOpen: (() => void) | undefined
  onClose: (() => void) | undefined

  constructor(appInitData: AppSolanaInitialize, app: AppSolana) {
    this.appInitData = appInitData
    this._app = app
    this.setApp(app)
  }

  private setApp = (app: AppSolana) => {
    this._app = app
    this._app.on('userConnected', (e) => {
      const adapter = new StandardWalletAdapter({
        wallet: new NightlyConnectSolanaWallet(app, new PublicKey(e.publicKeys[0]), async () => {
          const app = await AppSolana.build(this.appInitData)
          this.setApp(app)
        })
      })
      adapter.connect().then(() => {
        this.onConnected?.(adapter)
        this.closeModal()
      })
    })
  }

  public static build = async (appInitData: AppSolanaInitialize) => {
    const app = await AppSolana.build(appInitData)
    const selector = new NCSolanaSelector(appInitData, app)

    return selector
  }

  public openModal = () => {
    if (!this._modal) {
      this._modal = document.createElement('nightly-modal')
      this._modal.onClose = this.closeModal

      this._modal.network = 'SOLANA'
      this._modal.sessionId = this._app.sessionId
      this._modal.relay = 'https://nc2.nightly.app'
      this._modal.chainIcon = 'https://assets.coingecko.com/coins/images/4128/small/solana.png'
      this._modal.chainName = 'Solana'
      this._modal.selectorItems = getSolanaWalletsList([]).map((w) => ({
        name: w.name,
        icon: w.icon,
        status: w.recent ? 'Recent' : w.detected ? 'Detected' : ''
      })) as any
      this._modal.onWalletClick = (name) => {
        const wallet = getWallet(name)
        if (typeof wallet === 'undefined') {
          return
        }

        const adapter = new StandardWalletAdapter({
          wallet: wallet as WalletAdapterCompatibleStandardWallet
        })
        adapter.connect().then(() => {
          this.onConnected?.(adapter)
          this.closeModal()
        })
      }

      document.body.appendChild(this._modal)
    } else {
      this._modal.style.display = 'block'
    }
    this.onOpen?.()
  }

  public closeModal = () => {
    if (this._modal) {
      this._modal.style.display = 'none'
      this.onClose?.()
    }
  }
}
