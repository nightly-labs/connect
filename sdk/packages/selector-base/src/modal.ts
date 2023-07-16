import '@nightlylabs/wallet-selector-modal'
import { type NightlySelector, getNightlySelectorElement } from '@nightlylabs/wallet-selector-modal'
import { IWalletListItem, NetworkData } from './types'

export class NightlyConnectSelectorModal {
  _modal: NightlySelector | undefined

  _anchor: HTMLElement
  _onOpen: (() => void) | undefined
  _onClose: (() => void) | undefined
  _networkData: NetworkData
  _relay: string
  _walletsList: IWalletListItem[]

  _open = false

  constructor(
    walletsList: IWalletListItem[],
    relay: string,
    networkData: NetworkData,
    anchorRef?: HTMLElement | null,
    onOpen?: () => void,
    onClose?: () => void
  ) {
    this._walletsList = walletsList
    this._relay = relay
    this._networkData = networkData
    this._anchor = anchorRef ?? document.body
    this._onOpen = onOpen
    this._onClose = onClose
    this.createSelectorElement()
  }

  createSelectorElement = () => {
    this._modal = getNightlySelectorElement()
    this._modal.onClose = this.onCloseModal

    this._modal.network = this._networkData.network
    this._modal.relay = this._relay
    this._modal.chainIcon = this._networkData.icon
    this._modal.chainName = this._networkData.name
    this._modal.selectorItems = this._walletsList
  }

  setStandardWalletConnectProgress = (isConnectingToStandardWallet: boolean) => {
    if (this._modal) {
      this._modal.connecting = isConnectingToStandardWallet
    }
  }

  public openModal = (sessionId: string, onSelectListWallet: (name: string) => void) => {
    if (this._modal && this._open === false) {
      this._modal.onWalletClick = onSelectListWallet
      this._modal.sessionId = sessionId
      this._anchor.appendChild(this._modal)
      this._open = true
      this._onOpen?.()
    }
  }

  public onCloseModal = () => {
    if (this._modal && this._open === true) {
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
}
