import { customElement, property } from 'lit/decorators.js'
import { html } from 'lit/static-html.js'
import { tailwindElement } from '../../../shared/tailwind.element'
import foxSadGIF from '../../../static/gif/fox_sad.gif'
import search from '../../../static/svg/searchIcon.svg'
import { Breakpoint, getBreakpointFromWidth } from '../../../utils/utils'
import '../nightly-all-wallets-selector/nightly-all-wallets-selector'
import '../nightly-qrCode/nightly-qrCode'
import '../nightly-wallet-wrapper/nightly-wallet-wrapper'
import style from './nightly-wallet-selector-small-page.css'
import { LitElement } from 'lit'
@customElement('nightly-wallet-selector-small-page')
export class NightlyWalletSelectorSmallPage extends LitElement {
  static styles = tailwindElement(style)

  @property({})
  // eslint-disable-next-line @typescript-eslint/no-empty-function
  onClose = () => {}

  @property({ type: Boolean })
  showAll = false

  @property({ type: Boolean })
  isTopWalletsView = true

  @property({ type: Boolean })
  isQrPageVisible = false

  @property({ type: String })
  chainIcon = ''

  @property({ type: String })
  searchInputValue = ''

  @property({ type: String })
  sessionId = ''

  @property({ type: String })
  network = ''

  @property({ type: Function })
  // eslint-disable-next-line @typescript-eslint/no-empty-function
  onWalletClick: (name: string) => void = () => {}

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

  breakpoint: Breakpoint

  constructor() {
    super()
    this.handleSearchInput = this.handleSearchInput.bind(this)
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

  showAllWallets() {
    if (this.isQrPageVisible) {
      this.isQrPageVisible = false
      this.isTopWalletsView = true
      this.showAll = false
    } else {
      const oppositeState = this.showAll
      this.isTopWalletsView = oppositeState
      this.showAll = !oppositeState
    }
    this.requestUpdate()
  }

  handleSearchInput(event: InputEvent) {
    const searchInput = event.target as HTMLInputElement
    this.searchInputValue = searchInput.value

    const searchText = this.searchInputValue.toLowerCase()

    this.filteredItems = this.selectorItems.filter((item) => {
      return item.name.toLowerCase().includes(searchText)
    })

    this.showNotFoundIcon = this.filteredItems.length === 0
    this.requestUpdate()
  }

  openQrPage() {
    this.isQrPageVisible = true
    this.isTopWalletsView = false
    this.requestUpdate()
  }

  render() {
    if (this.showNotFoundIcon) {
      return this.renderNotFoundIcon()
    }

    return html`
      <div style="">
        ${(() => {
          if (!this.isTopWalletsView && this.showAll) {
            const allWalletsSelector = this.shadowRoot?.querySelector(
              'nightly-all-wallets-selector'
            )
            allWalletsSelector?.classList.add('visible')
            return html`
              <nightly-all-wallets-selector
                .showAllWallets=${this.showAllWallets.bind(this)}
                .onWalletClick=${this.onWalletClick.bind(this)}
                .searchInputValue=${this.searchInputValue}
                .selectorItems=${this.selectorItems}
                .filteredItems=${this.filteredItems}
                .handleSearchInput=${this.handleSearchInput}
                .showNotFoundIcon=${this.showNotFoundIcon}
              ></nightly-all-wallets-selector>
            `
          }

          if (!this.isTopWalletsView && this.isQrPageVisible) {
            return html`
              <div class="nightlyQrCode">
                <nightly-qr-code
                  .network=${this.network}
                  .sessionId=${this.sessionId}
                  .showAllWallets=${this.showAllWallets.bind(this)}
                ></nightly-qr-code>
              </div>
            `
          }

          return html`
            <div class="walletWrapperContainer ${this.isQrPageVisible ? 'open' : 'nightlyQrCode'}">
              <nightly-wallet-wrapper
                .network=${this.network}
                .sessionId=${this.sessionId}
                .breakpoint=${this.breakpoint}
                .showAllWallets=${this.showAllWallets.bind(this)}
                .onWalletClick=${this.onWalletClick.bind(this)}
                .openQrPage=${() => this.openQrPage()}
                .selectorItems=${this.selectorItems}
              ></nightly-wallet-wrapper>
            </div>
          `
        })()}
      </div>
    `
  }

  renderNotFoundIcon() {
    return html`
      <div class="NotFoundContainer">
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
        <img src="${foxSadGIF}" alt="Not Found" class="NotFoundGif" />
        <span class="NotFoundHeading">Nothing found...</span>
        <span class="NotFoundInfo">Make sure you’ve typed the name correctly.</span>
      </div>
    `
  }
}

declare global {
  interface HTMLElementTagNameMap {
    'nightly-wallet-selector-small-page': NightlyWalletSelectorSmallPage
  }
}
