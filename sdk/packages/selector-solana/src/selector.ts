import { AppSolana, AppSolanaInitialize } from '@nightlylabs/nightly-connect-solana'
import '@nightlylabs/wallet-selector-modal'
import { NightlyModal } from '@nightlylabs/wallet-selector-modal'
import { StandardWalletAdapter } from '@solana/wallet-standard'
import { NightlyConnectSolanaWallet } from './wallet'
import { PublicKey } from '@solana/web3.js'
import { getSolanaWalletsList } from './detection'
import { getWallet, modalStyle, triggerConnect } from '@nightlylabs/wallet-selector-base'
import { WalletAdapterCompatibleStandardWallet } from '@solana/wallet-adapter-base'
import { Deeplink } from '@nightlylabs/nightly-connect-solana/dist/browser/cjs/types/bindings/Deeplink'
import { isMobileBrowser } from '@nightlylabs/wallet-selector-base'

export class NCSolanaSelector {
  private _modal: NightlyModal | undefined
  private _modalRoot: HTMLDivElement | undefined
  private _app: AppSolana
  private _metadataWallets: Array<{
    name: string
    icon: string
    deeplink: Deeplink | null
  }>

  appInitData: AppSolanaInitialize

  onConnected: ((adapter: StandardWalletAdapter) => void) | undefined
  onOpen: (() => void) | undefined
  onClose: (() => void) | undefined

  constructor(
    appInitData: AppSolanaInitialize,
    app: AppSolana,
    metadataWallets: Array<{
      name: string
      icon: string
      deeplink: Deeplink | null
    }>
  ) {
    this.appInitData = appInitData
    this._app = app
    this._metadataWallets = metadataWallets
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
    const metadataWallets = await AppSolana.getWalletsMetadata(
      'https://nc2.nightly.app/get_wallets_metadata'
    )
      .then((list) =>
        list.map((wallet) => ({
          name: wallet.name,
          icon: wallet.image.default,
          deeplink: wallet.mobile
        }))
      )
      .catch(() => [] as any)
    const selector = new NCSolanaSelector(appInitData, app, metadataWallets)

    return selector
  }

  public openModal = () => {
    if (!this._modalRoot) {
      this._modal = document.createElement('nightly-modal')
      this._modal.onClose = this.closeModal

      this._modal.network = 'SOLANA'
      this._modal.sessionId = this._app.sessionId
      this._modal.relay = 'https://nc2.nightly.app'
      this._modal.chainIcon = 'https://assets.coingecko.com/coins/images/4128/small/solana.png'
      this._modal.chainName = 'Solana'
      this._modal!.selectorItems = getSolanaWalletsList(this._metadataWallets).map((w) => ({
        name: w.name,
        icon: w.icon,
        status: w.recent ? 'Recent' : w.detected ? 'Detected' : ''
      })) as any
      this._modal.onWalletClick = (name) => {
        if (isMobileBrowser()) {
          const walletData = this._metadataWallets.find((w) => w.name === name)

          if (
            typeof walletData === 'undefined' ||
            walletData.deeplink === null ||
            (walletData.deeplink.universal === null && walletData.deeplink.native === null)
          ) {
            return
          }

          this._app.connectDeeplink({
            walletName: walletData.name,
            url: walletData.deeplink.universal ?? walletData.deeplink.native!
          })

          triggerConnect(
            walletData.deeplink.universal ?? walletData.deeplink.native!,
            this._app.sessionId,
            'https://nc2.nightly.app'
          )
        } else {
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
      }

      const style = document.createElement('style')
      style.textContent = modalStyle
      document.head.appendChild(style)

      this._modalRoot = document.createElement('div')
      this._modalRoot.classList.add('nightlyConnectSelectorOverlay')

      this._modal.classList.add('nightlyConnectSelector')
      this._modalRoot.appendChild(this._modal)

      document.body.appendChild(this._modalRoot)
    } else {
      this._modalRoot.style.display = 'block'
    }
    this.onOpen?.()
  }

  public closeModal = () => {
    if (this._modalRoot) {
      this._modalRoot.style.display = 'none'
      this.onClose?.()
    }
  }
}
