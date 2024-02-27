import {
  type XMLOptions,
  type NightlySelector,
  WalletSelectorItem
} from '@nightlylabs/wallet-selector-modal'
import { type IWalletListItem, type NetworkData } from './types'

export class NightlyConnectSelectorModal {
  _modal: NightlySelector | undefined

  _anchor: HTMLElement
  _networkData: NetworkData
  _relay: string
  _walletsList: IWalletListItem[] = []

  _open = false

  onOpen: (() => void) | undefined
  onClose: (() => void) | undefined

  constructor(
    walletsList: IWalletListItem[],
    relay: string,
    networkData: NetworkData,
    anchorRef?: HTMLElement | null,
    variablesOverride?: object,
    stylesOverride?: string,
    qrConfigOverride?: Partial<XMLOptions>
  ) {
    this.walletsList = walletsList
    this._relay = relay
    this._networkData = networkData
    this._anchor = anchorRef ?? document.body
    this.createSelectorElement(variablesOverride, stylesOverride, qrConfigOverride)
  }

  get walletsList() {
    return this._walletsList
  }

  get qrCode() {
    return this._modal?.qrCode
  }

  set walletsList(list: IWalletListItem[]) {
    this._walletsList = list
    if (this._modal) {
      this._modal.selectorItems = list.map((item) => ({
        ...item,
        icon: item.image.default,
        link: item.homepage
      })) as WalletSelectorItem[]
    }
  }

  set sessionId(id: string) {
    if (this._modal && id) this._modal.sessionId = id
  }

  set timeoutError(error: string) {
    if (this._modal && error) this._modal.timeoutError = error
  }

  createSelectorElement = (
    variablesOverride?: object,
    stylesOverride?: string,
    qrConfigOverride?: Partial<XMLOptions>
  ) => {
    import('@nightlylabs/wallet-selector-modal').then(({ getNightlySelectorElement }) => {
      this._modal = getNightlySelectorElement(variablesOverride, stylesOverride, qrConfigOverride)
      this._modal.onClose = this.onCloseModal

      this._modal.relay = this._relay
      this._modal.chainIcon = this._networkData.icon
      this._modal.chainName = this._networkData.name
      this._modal.selectorItems = this.walletsList.map((item) => ({
        ...item,
        icon: item.image.default,
        link: item.homepage
      })) as WalletSelectorItem[]
    })
  }

  setStandardWalletConnectProgress = (isConnectingToStandardWallet: boolean) => {
    if (this._modal) {
      this._modal.connecting = isConnectingToStandardWallet
    }
  }

  public openModal = (
    sessionId: string | undefined,
    onSelectListWallet: (name: string) => void
  ) => {
    if (this._modal && this._open === false) {
      this._modal.onWalletClick = onSelectListWallet
      this._modal.sessionId = sessionId ?? ''
      this._anchor.appendChild(this._modal)
      this._open = true
      this.onOpen?.()
      return true
    }
    return false
  }

  public onCloseModal = () => {
    if (this._modal && this._open === true) {
      this._anchor.removeChild(this._modal)
      this._open = false
      this.onClose?.()
    }
  }

  public closeModal = () => {
    if (this._modal && this._open === true) {
      this._modal.handleClose()
    }
  }
}
