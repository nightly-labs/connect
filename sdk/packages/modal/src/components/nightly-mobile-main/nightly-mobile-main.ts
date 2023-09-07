import { customElement, property, state } from 'lit/decorators.js'
import { tailwindElement } from '../../shared/tailwind.element'
import { LitElement, html } from 'lit'
import { walletsSort } from '../../utils/utils'
import style from './nightly-mobile-main.css'
import { WalletSelectorItem } from '../../utils/types'
import '../nightly-wallet-selector-item/nightly-wallet-selector-item'

@customElement('nightly-mobile-main')
export class NightlyMobileMain extends LitElement {
  static styles = tailwindElement(style)

  @property({ type: Function })
  showAllWallets!: () => void

  @property({ type: Function })
  onWalletClick!: (name: string) => void

  @property({ type: Function })
  // eslint-disable-next-line @typescript-eslint/no-empty-function
  openQrPage: () => void = () => {}

  private _selectorItems: WalletSelectorItem[] = []

  @property({ type: Array })
  get selectorItems(): WalletSelectorItem[] {
    return this._selectorItems
  }

  set selectorItems(value: WalletSelectorItem[]) {
    this._selectorItems = [...value].sort(walletsSort)

    this.setItemsCount()
  }

  @state()
  numberOfItems = 2

  mobileQuery = window.matchMedia('(max-width: 640px)')
  smallerMobileQuery = window.matchMedia('(max-width: 482px)')
  smallestMobileQuery = window.matchMedia('(max-width: 374px)')

  setItemsCount = () => {
    if (this.smallestMobileQuery.matches) {
      this.numberOfItems = this.selectorItems.length > 3 ? 2 : 3
    } else if (this.smallerMobileQuery.matches) {
      this.numberOfItems = this.selectorItems.length > 4 ? 3 : 4
    } else if (this.mobileQuery.matches) {
      this.numberOfItems = this.selectorItems.length > 5 ? 4 : 5
    }
  }

  constructor() {
    super()

    this.mobileQuery.addEventListener('change', this.setItemsCount)

    this.smallerMobileQuery.addEventListener('change', this.setItemsCount)

    this.smallestMobileQuery.addEventListener('change', this.setItemsCount)
  }

  render() {
    return html`
      <div class="nc_mobileMainWrapper">
        <div class="nc_mobileMainTopBar">
          <span class="nc_mobileMainTopBarText">Connect wallet</span>
          <button class="nc_mobileMainTopBarQrButton" @click=${this.openQrPage}>QR Code</button>
        </div>
        <div class="nc_mobileMainWalletsList">
          ${this.selectorItems
            .slice(0, this.numberOfItems)
            .map(
              (wallet) =>
                html`
                  <nightly-wallet-selector-item
                    name=${wallet.name}
                    icon=${wallet.icon}
                    status=${wallet.recent ? 'Recent' : wallet.detected ? 'Detected' : ''}
                    @click=${() => this.onWalletClick(wallet.name)}
                  ></nightly-wallet-selector-item>
                `
            )}
          ${this.selectorItems.length > this.numberOfItems
            ? html`
                <div class="nc_mobileMainShowAllButton" @click=${this.showAllWallets}>
                  <div class="nc_mobileMainShowAllButtonIconsGrid">
                    ${this.selectorItems
                      .slice(this.numberOfItems, this.numberOfItems + 4)
                      .map(
                        (item) =>
                          html`<img
                            src=${item.icon}
                            class="nc_mobileMainShowAllButtonIcon"
                            alt=${item.name}
                          />`
                      )}
                  </div>
                  Other wallet
                </div>
              `
            : ''}
        </div>
      </div>
    `
  }
}

declare global {
  interface HTMLElementTagNameMap {
    'nightly-mobile-main': NightlyMobileMain
  }
}
