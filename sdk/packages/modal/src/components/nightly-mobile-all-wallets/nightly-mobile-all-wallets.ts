import { customElement, property, state } from 'lit/decorators.js'
import { tailwindElement } from '../../shared/tailwind.element'
import { LitElement, html } from 'lit'
import style from './nightly-mobile-all-wallets.css'
import { WalletSelectorItem } from '../../utils/types'
import { walletsSort } from '../../utils/utils'

@customElement('nightly-mobile-all-wallets')
export class NightlyMobileAllWallets extends LitElement {
  static styles = tailwindElement(style)

  @property({ type: Function })
  goBack!: () => void

  @property({ type: Function })
  onWalletClick!: (name: string) => void

  @property({ type: Array })
  get selectorItems(): WalletSelectorItem[] {
    return this._selectorItems
  }

  set selectorItems(value: WalletSelectorItem[]) {
    this._selectorItems = [...value].sort(walletsSort)
    this.filteredItems = this._selectorItems.filter((item) => {
      return item.name.toLowerCase().includes(this.searchText)
    })
  }

  private _selectorItems: WalletSelectorItem[] = []

  @state()
  filteredItems: WalletSelectorItem[] = []

  @state()
  searchText = ''

  handleSearchInput(event: InputEvent) {
    const searchInput = event.target as HTMLInputElement
    const searchText = searchInput.value.toLowerCase()
    this.searchText = searchText

    this.filteredItems = this.selectorItems.filter((item) => {
      return item.name.toLowerCase().includes(searchText)
    })
  }

  renderNotFoundIcon() {
    return html`
      <div class="nc_mobileAllWalletsEmptyListWrapper">
        <img
          src="https://registry.nightly.app/images/fox_sad.gif"
          alt="Not Found"
          class="nc_mobileAllWalletsEmptyListImage"
        />
        <span class="nc_mobileAllWalletsEmptyListHeading">Nothing found...</span>
        <span class="nc_mobileAllWalletsEmptyListDesc"
          >Make sure you've typed the name correctly.</span
        >
      </div>
    `
  }

  renderItems() {
    return html`
      <div class="nc_mobileAllWalletsListGrid">
        ${this.filteredItems.map((item) => {
          return html`
            <nightly-wallet-selector-item
              name=${item.name}
              icon=${item.icon}
              status=${item.recent ? 'Recent' : item.detected ? 'Detected' : ''}
              @click=${() => this.onWalletClick(item.name)}
            ></nightly-wallet-selector-item>
          `
        })}
      </div>
    `
  }

  render() {
    return html`
      <div class="nc_mobileAllWalletsWrapper">
        <div class="nc_mobileAllWalletsTopBar">
          <button class="nc_mobileAllWalletsBackButton" @click=${this.goBack}></button>
          <span class="nc_mobileAllWalletsTitle">All wallets</span>
          <div class="nc_mobileAllWalletsTopJustify"></div>
        </div>
        <div class="nc_mobileAllWalletsSearchBar">
          <div class="nc_mobileAllWalletsInputWrapper">
            <input
              placeholder="Search"
              class="nc_mobileAllWalletsInnerInput"
              @input=${this.handleSearchInput}
            />
            <div class="nc_mobileAllWalletsInputIcon"></div>
          </div>
        </div>
        ${this.filteredItems.length ? this.renderItems() : this.renderNotFoundIcon()}
      </div>
    `
  }
}

declare global {
  interface HTMLElementTagNameMap {
    'nightly-mobile-all-wallets': NightlyMobileAllWallets
  }
}
