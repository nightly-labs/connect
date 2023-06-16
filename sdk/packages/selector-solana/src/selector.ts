import { AppSolana, AppSolanaInitialize } from '@nightlylabs/nightly-connect-solana'
import '@nightlylabs/wallet-selector-modal'
import { NightlyModal } from '@nightlylabs/wallet-selector-modal'
import { StandardWalletAdapter } from '@solana/wallet-standard'
import { NightlyConnectSolanaWallet } from './wallet'
import { PublicKey } from '@solana/web3.js'

export class NCSolanaSelector {
  private _modal: NightlyModal | undefined
  private _app: AppSolana | undefined

  onSelectWallet: ((adapter: StandardWalletAdapter) => void) | undefined
  onOpen: (() => void) | undefined
  onClose: (() => void) | undefined

  constructor(selectorProps: { appInitData: AppSolanaInitialize }) {
    AppSolana.build(selectorProps.appInitData).then((app) => {
      this._app = app
      this._app.on('userConnected', (e) => {
        this.onSelectWallet?.(
          new StandardWalletAdapter({
            wallet: new NightlyConnectSolanaWallet(app, new PublicKey(e.publicKeys[0]))
          })
        )
      })
    })
  }

  public openModal = () => {
    if (!this._app) {
      return
    }

    if (!this._modal) {
      this._modal = document.createElement('nightly-modal')
      this._modal.onClose = this.closeModal

      this._modal.network = 'SOLANA'
      this._modal.sessionId = this._app.sessionId
      this._modal.relay = 'https://relay.nightly.app'
      this._modal.chainIcon = 'https://assets.coingecko.com/coins/images/4128/small/solana.png'
      this._modal.chainName = 'Solana'
      this._modal.selectorItems = []
      this._modal.onWalletClick = (name) => {
        console.log(name)
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
