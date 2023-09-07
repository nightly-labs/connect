import { customElement, property, state } from 'lit/decorators.js'
import { tailwindElement } from '../../shared/tailwind.element'
import { LitElement, html } from 'lit'
import style from './nightly-all-wallets-selector.css'
import vector from '../../static/svg/backButton.svg'
import search from '../../static/svg/searchIcon.svg'
import { WalletSelectorItem } from '../../utils/types'
import { walletsSort } from '../../utils/utils'

@customElement('nightly-all-wallets-selector')
export class NightlyAllWalletsSelector extends LitElement {
  static styles = tailwindElement(style)

  @property({ type: Function })
  showAllWallets!: () => void

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

  render() {
    return html`
      <div class="walletSelectorButtons">
        <div class="headerContainer">
          <button class="buttonContainer" @click=${this.showAllWallets}>
            <img src=${vector} />
          </button>
          <div class="textContainer">
            <span> All wallets </span>
          </div>
          <div class="buttonContainer"></div>
        </div>
        <div class="inputContainer">
          <div class="walletInputSearchContainer">
            <input
              placeholder="Search"
              class="walletInputSearch"
              @input=${this.handleSearchInput}
            />
            <img class="walletInputIcon" src="${search}" />
          </div>
        </div>
        ${this.filteredItems.length ? this.renderItems() : this.renderNotFoundIcon()}
      </div>
    `
  }

  renderNotFoundIcon() {
    return html`
      <div class="NotFoundContainer">
        <img
          src="https://registry.connect.nightly.app/images/fox_sad.gif"
          alt="Not Found"
          class="NotFoundGif"
        />
        <span class="NotFoundHeading">Nothing found...</span>
        <span class="NotFoundInfo">Make sure you’ve typed the name correctly.</span>
      </div>
    `
  }

  renderItems() {
    return html`
      <div class="recentDetectedContainer">
        ${this.filteredItems.map((item) => {
          return html`
            <nightly-wallet-selector-item
              class="nightlyWalletSelectorItem"
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

  handleSearchInput(event: InputEvent) {
    const searchInput = event.target as HTMLInputElement
    const searchText = searchInput.value.toLowerCase()
    this.searchText = searchText

    this.filteredItems = this.selectorItems.filter((item) => {
      return item.name.toLowerCase().includes(searchText)
    })
  }
}

declare global {
  interface HTMLElementTagNameMap {
    'nightly-all-wallets-selector': NightlyAllWalletsSelector
  }
}
