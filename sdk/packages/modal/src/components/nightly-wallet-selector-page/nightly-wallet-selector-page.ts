import { customElement, property } from 'lit/decorators.js'
import { html } from 'lit/static-html.js'
import { TailwindElement } from '../../shared/tailwind.element'
import foxSadGIF from '../../static/gif/fox_sad.gif'
import search from '../../static/svg/searchIcon.svg'
import style from './nightly-wallet-selector-page.css?inline'
import '../nightly-wallet-selector-item/nightly-wallet-selector-item'

@customElement('nightly-wallet-selector-page')
export class NightlyWalletSelectorPage extends TailwindElement(style) {
  @property({ type: String })
  chainIcon = ''
  @property({ type: String })
  chainName = ''
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

  private _selectorItems: { name: string; icon: string; status: string }[] = []
  filteredItems: { name: string; icon: string; status: string }[] = []
  showNotFoundIcon = false

  @property({ type: Function })
  // eslint-disable-next-line @typescript-eslint/no-empty-function
  onWalletClick: (name: string) => void = () => {}

  constructor() {
    super()
  }

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
          <div class="${this.showNotFoundIcon ? 'NotFoundContainer' : 'walletSelectorButtons'}">
            ${this.showNotFoundIcon ? this.renderNotFoundIcon() : this.renderSelectorItems()}
          </div>
        </div>
      </div>
    `
  }

  renderSelectorItems() {
    const recentDetectedItems = this.filteredItems.filter(
      (item) => item.status === 'recent' || item.status === 'detected'
    )
    const otherItems = this.filteredItems.filter(
      (item) => item.status !== 'recent' && item.status !== 'detected'
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
        <img src="${foxSadGIF}" alt="Not Found" class="NotFoundGif" />
        <span class="NotFoundHeading">Nothing found...</span>
        <span class="NotFoundInfo">Make sure you’ve typed the name correctly.</span>
      </div>
    `
  }

  handleSearchInput(event: InputEvent) {
    const searchInput = event.target as HTMLInputElement
    const searchText = searchInput.value.toLowerCase()

    this.filteredItems = this.selectorItems.filter((item) => {
      return item.name.toLowerCase().includes(searchText)
    })

    this.showNotFoundIcon = this.filteredItems.length === 0

    this.requestUpdate()
  }
}

declare global {
  interface HTMLElementTagNameMap {
    'nightly-wallet-selector-page': NightlyWalletSelectorPage
  }
}
