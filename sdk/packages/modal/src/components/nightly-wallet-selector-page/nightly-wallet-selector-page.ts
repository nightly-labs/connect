import { customElement, property, state } from 'lit/decorators.js'
import { html } from 'lit/static-html.js'
import { tailwindElement } from '../../shared/tailwind.element'
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

  @property({ type: Function })
  // eslint-disable-next-line @typescript-eslint/no-empty-function
  onWalletClick: (name: string) => void = () => {}

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

  renderSelectorItems() {
    const recentDetectedItems = this.filteredItems.filter((item) => item.recent || item.detected)
    const otherItems = this.filteredItems.filter((item) => !item.recent && !item.detected)

    return html`
      <div class="nc_desktopListWalletsListWrapper">
        ${recentDetectedItems.length
          ? html`<div class="nc_desktopListRecentGrid">
              ${recentDetectedItems.map((item) => {
                return html`
                  <nightly-wallet-selector-item
                    name=${item.name}
                    icon=${item.icon}
                    status=${item.recent ? 'Recent' : item.detected ? 'Detected' : 'Install'}
                    @click=${() => this.onWalletClick(item.name)}
                  ></nightly-wallet-selector-item>
                `
              })}
            </div>`
          : null}
        <div class="nc_desktopListNotDetectedGrid">
          ${otherItems.map((item) => {
            return html`
              <nightly-wallet-selector-item
                name=${item.name}
                icon=${item.icon}
                status=${item.recent ? 'Recent' : item.detected ? 'Detected' : 'Install'}
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
      <div class="nc_desktopListEmptyListWrapper">
        <img
          src="https://registry.nightly.app/images/fox_sad.gif"
          alt="Not Found"
          class="nc_desktopListEmptyListImage"
        />
        <span class="nc_desktopListEmptyListHeading">Nothing found...</span>
        <span class="nc_desktopListEmptyListDesc">Make sure you’ve typed the name correctly.</span>
      </div>
    `
  }

  render() {
    return html`
      <div class="nc_desktopListWrapper">
        <div class="nc_desktopListTopBar">
          Wallets
          <div class="nc_desktopListTopBarChain">
            <img class="nc_desktopListTopBarChainIcon" src=${this.chainIcon} />
            ${this.chainName}
          </div>
        </div>
        <div class="nc_desktopListSearchBar">
          <div class="nc_desktopListInputWrapper">
            <input
              placeholder="Search"
              class="nc_desktopListInnerInput"
              @input=${this.handleSearchInput}
            />
            <div class="nc_desktopListInputIcon"></div>
          </div>
        </div>
        ${this.filteredItems.length === 0 ? this.renderNotFoundIcon() : this.renderSelectorItems()}
      </div>
    `
  }
}

declare global {
  interface HTMLElementTagNameMap {
    'nightly-wallet-selector-page': NightlyWalletSelectorPage
  }
}
