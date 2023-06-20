import { AppSui, AppSuiInitialize } from '@nightlylabs/nightly-connect-sui'
import '@nightlylabs/wallet-selector-modal'
import { NightlyModal } from '@nightlylabs/wallet-selector-modal'
import { StandardWalletAdapter } from '@mysten/wallet-adapter-wallet-standard'
import { NightlyConnectSuiWallet } from './wallet'
import { publicKeyFromSerialized } from '@mysten/sui.js'

export class NCSuiSelector {
  private _modal: NightlyModal | undefined
  private _app: AppSui

  appInitData: AppSuiInitialize

  onConnected: ((adapter: StandardWalletAdapter) => void) | undefined
  onOpen: (() => void) | undefined
  onClose: (() => void) | undefined

  constructor(appInitData: AppSuiInitialize, app: AppSui) {
    this.appInitData = appInitData
    this._app = app
    this.setApp(app)
  }

  private setApp = (app: AppSui) => {
    this._app = app
    this._app.on('userConnected', (e) => {
      const adapter = new StandardWalletAdapter({
        wallet: new NightlyConnectSuiWallet(
          app,
          e.publicKeys.map((pk) => publicKeyFromSerialized('ED25519', pk)),
          async () => {
            const app = await AppSui.build(this.appInitData)
            this.setApp(app)
          }
        )
      })
      adapter.connect().then(() => {
        this.onConnected?.(adapter)
        this.closeModal()
      })
    })
  }

  public static build = async (appInitData: AppSuiInitialize) => {
    const app = await AppSui.build(appInitData)
    const selector = new NCSuiSelector(appInitData, app)

    return selector
  }

  public openModal = () => {
    if (!this._modal) {
      this._modal = document.createElement('nightly-modal')
      this._modal.onClose = this.closeModal

      this._modal.network = 'SUI'
      this._modal.sessionId = this._app.sessionId
      this._modal.relay = 'https://nc2.nightly.app'
      this._modal.chainIcon = 'https://assets.coingecko.com/coins/images/26375/small/sui_asset.jpeg'
      this._modal.chainName = 'Sui'
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
