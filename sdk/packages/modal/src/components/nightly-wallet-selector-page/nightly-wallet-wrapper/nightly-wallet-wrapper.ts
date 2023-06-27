import { customElement, property } from 'lit/decorators.js'
import { TailwindElement } from '../../../shared/tailwind.element'
import { html } from 'lit'
import { Breakpoint, getBreakpointFromWidth, getNumberOfItems } from '../../../utils/utils'
import style from './nightly-wallet-wrapper.css?inline'

@customElement('nightly-wallet-wrapper')
export class NightlyWalletWrapper extends TailwindElement(style) {
  @property({ type: Function })
  showAllWallets!: () => void

  @property({ type: Function })
  onWalletClick!: (name: string) => void

  @property({ type: Function })
  openQrPage: () => void = () => {
    console.log('show wallets')
  }

  @property({ type: Array })
  get selectorItems(): { name: string; icon: string; status: string }[] {
    return this._selectorItems
  }

  set selectorItems(value: { name: string; icon: string; status: string }[]) {
    const oldValue = this._selectorItems
    this._selectorItems = value
    this.filteredItems = [...value]
    this.requestUpdate('selectorItems', oldValue)
  }

  breakpoint: Breakpoint

  constructor() {
    super()
    // this.showAllWallets = this.showAllWallets.bind(this)
    this.breakpoint = 'lg'
    this.updateBreakpoint()
    this.resizeListener()
  }

  updateBreakpoint() {
    const screenWidth = window.innerWidth
    this.breakpoint = getBreakpointFromWidth(screenWidth)
  }

  resizeListener() {
    window.addEventListener('resize', () => {
      this.updateBreakpoint()
      this.requestUpdate()
    })
  }

  private _selectorItems: { name: string; icon: string; status: string }[] = []
  filteredItems: { name: string; icon: string; status: string }[] = []
  showNotFoundIcon = false
  render() {
    const numberOfItems = getNumberOfItems(this.breakpoint)
    const totalItems = this._selectorItems.length
    const showListButtonContainerDisplay = totalItems > numberOfItems ? 'flex' : 'none'

    return html`
      <div class="mainContainer">
        <div class="walletWrapper">
          <div class="infoConatiner">
            <p>Connect wallet</p>
            <button id="nightly-wallet-selector-page-qrCode-open-button" @click=${this.openQrPage}>
              QR Code
            </button>
          </div>
          <div class="mainContainerWalletSellector">
            ${this._selectorItems.slice(0, numberOfItems).map(
              (wallet) =>
                html`
                  <div class="topWalletsItem">
                    <nightly-wallet-selector-item
                      name=${wallet.name}
                      icon=${wallet.icon}
                      status=${wallet.status}
                      @click=${() => this.onWalletClick(wallet.name)}
                    ></nightly-wallet-selector-item>
                  </div>
                `
            )}
            <div
              class="showListButtonContainer"
              @click=${this.showAllWallets}
              style=${`display: ${showListButtonContainerDisplay}`}
            >
              <button class="showListButton">
                ${this.selectorItems.slice(0, Math.min(this.selectorItems.length, 4)).map(
                  (item) => html`
                    <div>
                      <img src=${item.icon} class="buttonIcons" alt=${item.name} />
                    </div>
                  `
                )}
              </button>
              <span>Other wallet</span>
            </div>
          </div>
        </div>
      </div>
    `
  }
}

declare global {
  interface HTMLElementTagNameMap {
    'nightly-wallet-wrapper': NightlyWalletWrapper
  }
}
