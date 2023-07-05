import '@nightlylabs/wallet-selector-modal'
import { type NightlySelector, getNightlySelectorElement } from '@nightlylabs/wallet-selector-modal'
import { triggerConnect, isMobileBrowser } from './utils'
import { getWallet, getWalletsList } from './detection'
import { Adapter, AppInitData, MetadataWallet, NetworkData } from './types'
import { Wallet } from '@wallet-standard/core'

export class NCBaseSelector<A extends Adapter> {
  _modal: NightlySelector | undefined
  _metadataWallets: MetadataWallet[]
  _adapterCreator: (wallet: Wallet) => A
  _walletsFilterCb: (wallet: Wallet) => boolean
  _networkData: NetworkData
  _appInitData: AppInitData
  _sessionId: string
  _connectDeeplink: (walletName: string, url: string) => void
  _anchor: HTMLElement

  onConnected: ((adapter: A) => void) | undefined
  onOpen: (() => void) | undefined
  onClose: (() => void) | undefined

  constructor(
    appInitData: AppInitData,
    metadataWallets: MetadataWallet[],
    adapterCreator: (wallet: Wallet) => A,
    walletsFilterCb: (wallet: Wallet) => boolean,
    networkData: NetworkData,
    sessionId: string,
    connectDeeplink: (name: string, url: string) => void,
    anchorRef?: HTMLElement
  ) {
    this._appInitData = appInitData
    this._metadataWallets = metadataWallets
    this._adapterCreator = adapterCreator
    this._walletsFilterCb = walletsFilterCb
    this._networkData = networkData
    this._sessionId = sessionId
    this._connectDeeplink = connectDeeplink
    this._anchor = anchorRef ?? document.body
    this.createSelectorElement()
  }

  createSelectorElement = () => {
    this._modal = getNightlySelectorElement()
    this._modal.onClose = this.closeModal

    this._modal.network = this._networkData.network
    this._modal.sessionId = this._sessionId
    this._modal.relay = this._appInitData.url ?? 'https://nc2.nightly.app'
    this._modal.chainIcon = this._networkData.icon
    this._modal.chainName = this._networkData.name
    this._modal.selectorItems = getWalletsList(this._metadataWallets, this._walletsFilterCb).map(
      (w) => ({
        name: w.name,
        icon: w.icon,
        status: w.recent ? 'Recent' : w.detected ? 'Detected' : '',
        link: w.link ?? ''
      })
    )
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

        this._connectDeeplink(
          walletData.name,
          walletData.deeplink.universal ?? walletData.deeplink.native!
        )

        triggerConnect(
          walletData.deeplink.universal ?? walletData.deeplink.native!,
          this._sessionId,
          this._appInitData.url ?? 'https://nc2.nightly.app'
        )

        this._modal!.connecting = true
      } else {
        const wallet = getWallet(name)
        if (typeof wallet === 'undefined') {
          return
        }

        const adapter = this._adapterCreator(wallet)
        this._modal!.connecting = true
        adapter
          .connect()
          .then(() => {
            this.onConnected?.(adapter)
            this.closeModal()
          })
          .catch(() => {
            this._modal!.connecting = false
          })
      }
    }
  }

  public openModal = () => {
    if (this._modal) {
      this._anchor.appendChild(this._modal)
    }
    this.onOpen?.()
  }

  public closeModal = () => {
    if (this._modal) {
      this._anchor.removeChild(this._modal)
      this.onClose?.()
    }
  }
}
