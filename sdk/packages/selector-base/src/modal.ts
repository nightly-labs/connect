import { type XMLOptions, type NightlySelector } from '@nightlylabs/wallet-selector-modal'
import { type IWalletListItem, type NetworkData } from './types'
import { isMobileBrowser } from './utils'

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

  set walletsList(list: IWalletListItem[]) {
    const filtered = list.filter((w) =>
      isMobileBrowser() ? w.walletType !== 'extension' : w.walletType !== 'mobile'
    )
    this._walletsList = filtered
    if (this._modal) {
      this._modal.selectorItems = filtered
    }
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
      this._modal.selectorItems = this.walletsList
    })
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
