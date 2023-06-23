import { customElement, property } from 'lit/decorators.js'
import { TailwindElement } from '../../../shared/tailwind.element'
import { html } from 'lit'
import style from './nightly-all-wallets-selector.css?inline'
import vector from '../../../static/svg/backButton.svg'
import search from '../../../static/svg/searchIcon.svg'

@customElement('nightly-all-wallets-selector')
export class NightlyAllWalletsSelector extends TailwindElement(style) {
  @property({ type: Function })
  showAllWallets!: () => void

  @property({ type: Function })
  onWalletClick!: (name: string) => void

  @property({ type: Function })
  handleSearchInput!: (event: InputEvent) => void

  @property({ type: String })
  searchInputValue = ''

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

  render() {
    return html`
      <div class="walletSelectorButtons">
        <div class="headerContainer">
          <div class="buttonContainer">
            <button @click=${this.showAllWallets}>
              <img src=${vector} />
            </button>
          </div>
          <div class="textContainer">
            <span> All wallets </span>
          </div>
        </div>
        <div class="inputContainer">
          <div class="walletInputSearchContainer">
            <input
              placeholder="Search"
              class="walletInputSearch"
              .value=${this.searchInputValue}
              @input=${this.handleSearchInput}
            />
            <img src="${search}" />
          </div>
        </div>
        <div class="recentDetectedContainer">
          ${this.filteredItems.map((item) => {
            return html`
              <div class="nightlyWalletSelectorItem">
                <nightly-wallet-selector-item
                  name=${item.name}
                  icon=${item.icon}
                  status=${item.status}
                  @click=${() => this.onWalletClick(item.name)}
                ></nightly-wallet-selector-item>
              </div>
            `
          })}
        </div>
      </div>
    `
  }
}

declare global {
  interface HTMLElementTagNameMap {
    'nightly-all-wallets-selector': NightlyAllWalletsSelector
  }
}
