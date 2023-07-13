import '@nightlylabs/wallet-selector-modal'
import {
  type NightlySelector,
  getNightlySelectorElement,
  WalletSelectorItem
} from '@nightlylabs/wallet-selector-modal'
import { triggerConnect, isMobileBrowser } from './utils'
import { getWalletsList } from './detection'
import { Adapter, AppInitData, MetadataWallet, NetworkData } from './types'
import { Wallet } from '@wallet-standard/core'
import {
  getRecentStandardWalletForNetwork,
  persistRecentStandardWalletForNetwork
} from './persistence'

export class NCBaseSelector<A extends Adapter> {
  _modal: NightlySelector | undefined
  _metadataWallets: MetadataWallet[]
  _adapterCreator: (wallet: Wallet) => A
  _walletsFilterCb: (wallet: Wallet) => boolean
  _networkData: NetworkData
  _appInitData: AppInitData
  _sessionId: string
  _connectDeeplink: (walletName: string, url: string) => void
  _eagerConnect: boolean
  _anchor: HTMLElement
  _onConnected: (adapter: A) => void
  _onOpen: (() => void) | undefined
  _onClose: (() => void) | undefined
  _selectorWalletsList: Array<WalletSelectorItem & { standardWallet?: Wallet }> = []
  _open = false

  constructor(
    appInitData: AppInitData,
    metadataWallets: MetadataWallet[],
    adapterCreator: (wallet: Wallet) => A,
    walletsFilterCb: (wallet: Wallet) => boolean,
    networkData: NetworkData,
    sessionId: string,
    connectDeeplink: (name: string, url: string) => void,
    onConnected: (adapter: A) => void,
    eagerConnect?: boolean,
    anchorRef?: HTMLElement,
    onOpen?: () => void,
    onClose?: () => void
  ) {
    this._appInitData = appInitData
    this._metadataWallets = metadataWallets
    this._adapterCreator = adapterCreator
    this._walletsFilterCb = walletsFilterCb
    this._networkData = networkData
    this._sessionId = sessionId
    this._connectDeeplink = connectDeeplink
    this._onConnected = onConnected
    this._eagerConnect = eagerConnect ?? false
    this._anchor = anchorRef ?? document.body
    this._onOpen = onOpen
    this._onClose = onClose
    this.createSelectorElement()
    this.setSelectorStandardWallets()
  }

  get sessionId() {
    return this._sessionId
  }

  set sessionId(id: string) {
    this._sessionId = id

    if (this._modal) {
      this._modal.sessionId = id
    }
  }

  eagerConnectToRecent = () => {
    const recentName = getRecentStandardWalletForNetwork(this._networkData.name)
    if (this._eagerConnect && recentName !== null) {
      this.connectToStandardWallet(recentName)
    }
  }

  connectToStandardWallet = (name: string) => {
    const wallet = this._selectorWalletsList.find((w) => w.name === name)
    if (typeof wallet?.standardWallet === 'undefined') {
      return
    }

    const adapter = this._adapterCreator(wallet.standardWallet)
    this._modal!.connecting = true
    adapter
      .connect()
      .then(() => {
        persistRecentStandardWalletForNetwork(name, this._networkData.name)
        this._onConnected(adapter)
        this.closeModal()
      })
      .catch(() => {
        this._modal!.connecting = false
      })
  }

  setSelectorStandardWallets = () => {
    const recentName = getRecentStandardWalletForNetwork(this._networkData.name)
    this._selectorWalletsList = getWalletsList(this._metadataWallets, this._walletsFilterCb).map(
      (w) => ({
        name: w.name,
        icon: w.icon,
        link: w.link ?? '',
        detected: w.detected,
        recent: w.name === recentName,
        standardWallet: w.standardWallet
      })
    )
    if (this._modal) {
      this._modal.selectorItems = this._selectorWalletsList
    }
  }

  _chosenMobileWalletName: string | undefined

  createSelectorElement = () => {
    this._modal = getNightlySelectorElement()
    this._modal.onClose = this.onCloseModal

    this._modal.network = this._networkData.network
    this._modal.sessionId = this.sessionId
    this._modal.relay = this._appInitData.url ?? 'https://nc2.nightly.app'
    this._modal.chainIcon = this._networkData.icon
    this._modal.chainName = this._networkData.name
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

        this._chosenMobileWalletName = name

        triggerConnect(
          walletData.deeplink.universal ?? walletData.deeplink.native!,
          this.sessionId,
          this._appInitData.url ?? 'https://nc2.nightly.app'
        )

        this._modal!.connecting = true
      } else {
        this.connectToStandardWallet(name)
      }
    }
  }

  public openModal = () => {
    if (this._modal && this._open === false) {
      this.setSelectorStandardWallets()
      this._anchor.appendChild(this._modal)
      this._open = true
      this._onOpen?.()
    }
  }

  public onCloseModal = () => {
    if (this._modal && this._open === true) {
      this._modal.connecting = true
      this._anchor.removeChild(this._modal)
      this._open = false
      this._onClose?.()
    }
  }

  public closeModal = () => {
    if (this._modal && this._open === true) {
      this._modal.handleClose()
    }
  }

  public eagerConnectDeeplink = (network: string) => {
    if (isMobileBrowser()) {
      const mobileWalletName = getRecentStandardWalletForNetwork(network)
      const wallet = this._metadataWallets.find((w) => w.name === mobileWalletName)
      if (
        typeof wallet !== 'undefined' &&
        wallet.deeplink !== null &&
        (wallet.deeplink.universal !== null || wallet.deeplink.native !== null)
      ) {
        this._connectDeeplink(wallet.name, wallet.deeplink.universal ?? wallet.deeplink.native!)
      }
    }
  }
}
