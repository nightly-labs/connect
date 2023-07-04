import { customElement, property, state } from 'lit/decorators.js'
import { html } from 'lit/static-html.js'
import { tailwindElement } from '../../shared/tailwind.element'
import search from '../../static/svg/searchIcon.svg'
import style from './nightly-wallet-selector-page.css'
import '../nightly-wallet-selector-item/nightly-wallet-selector-item'
import { LitElement } from 'lit'

@customElement('nightly-wallet-selector-page')
export class NightlyWalletSelectorPage extends LitElement {
  static styles = tailwindElement(style)

  @property({ type: String })
  chainIcon = ''
  @property({ type: String })
  chainName = ''
  @property({ type: Array })
  get selectorItems(): { name: string; icon: string; status: string }[] {
    return this._selectorItems
  }

  set selectorItems(value: { name: string; icon: string; status: string }[]) {
    this._selectorItems = value
    this.filteredItems = value.filter((item) => {
      return item.name.toLowerCase().includes(this.searchText)
    })
  }

  private _selectorItems: { name: string; icon: string; status: string }[] = []

  @state()
  filteredItems: { name: string; icon: string; status: string }[] = []
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
          <div class="walletInputSearchContainer">
            <input
              placeholder="Search"
              class="walletInputSearch"
              @input=${this.handleSearchInput}
            />
            <img src="${search}" />
          </div>
          ${this.filteredItems.length === 0
            ? this.renderNotFoundIcon()
            : this.renderSelectorItems()}
        </div>
      </div>
    `
  }

  renderSelectorItems() {
    const recentDetectedItems = this.filteredItems.filter(
      (item) => item.status.toLowerCase() === 'recent' || item.status.toLowerCase() === 'detected'
    )
    const otherItems = this.filteredItems.filter(
      (item) => item.status.toLowerCase() !== 'recent' && item.status.toLowerCase() !== 'detected'
    )

    return html`
      <div class="walletSelectorButtons">
        <div class="recentDetectedContainer">
          ${recentDetectedItems.map((item) => {
            return html`
              <nightly-wallet-selector-item
                name=${item.name}
                icon=${item.icon}
                status=${item.status}
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
                status=${item.status}
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
