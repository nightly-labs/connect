import { customElement, property, state } from 'lit/decorators.js'
import { tailwindElement } from '../../../shared/tailwind.element'
import { LitElement, html } from 'lit'
import {
  Breakpoint,
  getBreakpointFromWidth,
  getNumberOfItems,
  walletsSort
} from '../../../utils/utils'
import style from './nightly-wallet-wrapper.css'
import { WalletSelectorItem } from '../../../utils/types'

@customElement('nightly-wallet-wrapper')
export class NightlyWalletWrapper extends LitElement {
  static styles = tailwindElement(style)

  @property({ type: Function })
  showAllWallets!: () => void

  @property({ type: Function })
  onWalletClick!: (name: string) => void

  @property({ type: Function })
  openQrPage: () => void = () => {
    console.log('show wallets')
  }

  @property({ type: Array })
  get selectorItems(): WalletSelectorItem[] {
    return this._selectorItems
  }

  set selectorItems(value: WalletSelectorItem[]) {
    this._selectorItems = value
  }

  @state()
  breakpoint: Breakpoint = 'lg'

  constructor() {
    super()
    this.updateBreakpoint()
    window.addEventListener('resize', () => {
      this.updateBreakpoint()
    })
  }

  updateBreakpoint() {
    const screenWidth = window.innerWidth
    this.breakpoint = getBreakpointFromWidth(screenWidth)
  }

  private _selectorItems: WalletSelectorItem[] = []

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
            ${this._selectorItems
              .sort(walletsSort)
              .slice(0, numberOfItems)
              .map(
                (wallet) =>
                  html`
                    <nightly-wallet-selector-item
                      class="topWalletsItem"
                      name=${wallet.name}
                      icon=${wallet.icon}
                      status=${wallet.recent ? 'Recent' : wallet.detected ? 'Detected' : ''}
                      @click=${() => this.onWalletClick(wallet.name)}
                    ></nightly-wallet-selector-item>
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
