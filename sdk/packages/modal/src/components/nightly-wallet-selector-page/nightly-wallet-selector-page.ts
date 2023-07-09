import { customElement, property, state } from 'lit/decorators.js'
import { html } from 'lit/static-html.js'
import { tailwindElement } from '../../shared/tailwind.element'
import search from '../../static/svg/searchIcon.svg'
import style from './nightly-wallet-selector-page.css'
import '../nightly-wallet-selector-item/nightly-wallet-selector-item'
import { LitElement } from 'lit'
import { WalletSelectorItem } from '../../utils/types'
import { walletsSort } from '../../utils/utils'

@customElement('nightly-wallet-selector-page')
export class NightlyWalletSelectorPage extends LitElement {
  static styles = tailwindElement(style)

  @property({ type: String })
  chainIcon = ''
  @property({ type: String })
  chainName = ''
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

  @property({ type: Function })
  // eslint-disable-next-line @typescript-eslint/no-empty-function
  onWalletClick: (name: string) => void = () => {}

  render() {
    return html`
      <div class="walletSelectorPage">
        <div class="contentContainer">
          <div class="walletSelectorHeader">
            <span>Wallets</span>
            <div class="walletSelectorBlockchain">
              <img src=${this.chainIcon} />
              <span>${this.chainName}</span>
            </div>
          </div>
          <div class="walletInputSearchWrapper">
            <div class="walletInputSearchContainer">
              <input
                placeholder="Search"
                class="walletInputSearch"
                @input=${this.handleSearchInput}
              />
              <img class="walletInputIcon" src="${search}" />
            </div>
          </div>
          ${this.filteredItems.length === 0
            ? this.renderNotFoundIcon()
            : this.renderSelectorItems()}
        </div>
      </div>
    `
  }

  renderSelectorItems() {
    const recentDetectedItems = this.filteredItems.filter((item) => item.recent || item.detected)
    const otherItems = this.filteredItems.filter((item) => !item.recent && !item.detected)

    return html`
      <div class="walletSelectorButtons">
        <div class="recentDetectedContainer">
          ${recentDetectedItems.map((item) => {
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
        <div class="otherItemsContainer">
          ${otherItems.map((item) => {
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
    'nightly-wallet-selector-page': NightlyWalletSelectorPage
  }
}
