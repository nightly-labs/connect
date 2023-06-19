import { generateQrCodeXml } from '@nightlylabs/qr-code'
import { customElement, property } from 'lit/decorators.js'
import { html } from 'lit/static-html.js'
import { TailwindElement } from '../../../shared/tailwind.element'
import foxSadGIF from '../../../static/gif/fox_sad.gif'
import vector from '../../../static/svg/backButton.svg'
import search from '../../../static/svg/searchIcon.svg'
import { svgToBase64 } from '../../../utils/images'
import { Breakpoint, getBreakpointFromWidth, getNumberOfItems } from '../../../utils/utils'
import '../../nightly-header-small-page/nightly-header-small-page'
import style from './nightly-wallet-selector-small-page.css?inline'

@customElement('nightly-wallet-selector-small-page')
export class NightlyWalletSelectorSmallPage extends TailwindElement(style) {
  @property()
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

  render() {
    return html`
      <div>
        ${(() => {
          if (this.isQrPageVisible) {
            return this.renderQrCode()
          }

          if (!this.isTopWalletsView && this.showAll) {
            return this.renderFullPage()
          }

          return this.renderTopWallets()
        })()}
      </div>
    `
  }
  renderTopWallets() {
    const numberOfItems = getNumberOfItems(this.breakpoint)
    return html`
      <div class="mainContainer">
        <div class="nightly-headerContainer">
          <nightly-header-small-page onClose=${this.onClose}></nightly-header-small-page>
        </div>
        <div class="walletWrapper">
          <div class="infoConatiner">
            <p>Connect wallet</p>
            <button @click=${this.openQrPage}>QR Code</button>
          </div>
          <div class="mainContainerWalletSellector">
            ${this._selectorItems.slice(0, numberOfItems).map(
              (wallet) =>
                html`
                  <div class="topWalletsItem">
                    <nightly-wallet-selector-item
                      name=${wallet.name}
                      icon=${wallet.icon}
                      status=${wallet.status}
                      @click=${(event: Event) => this.handleWalletSelectorClick(event, wallet.name)}
                    ></nightly-wallet-selector-item>
                  </div>
                `
            )}
            <div class="showListButtonContainer" @click=${this.showAllWallets}>
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

  openQrPage() {
    this.isQrPageVisible = true
    this.requestUpdate()
  }

  renderQrCode() {
    return html`
      <div class="nightly-headerContainer">
        <nightly-header-small-page onClose=${this.onClose}></nightly-header-small-page>
      </div>
      <div class="headerQrCodeWrapper">
        <div class="headerContainer">
          <div class="buttonContainer">
            <button @click=${this.showAllWallets}>
              <img src=${vector} />
            </button>
          </div>
          <div class="textContainer">
            <span> QR Code </span>
          </div>
        </div>
        <div class="qrCodeWrapper">
          <img
            class="code"
            src=${svgToBase64(
              generateQrCodeXml('nightlyconnect:' + this.sessionId + '?network=' + this.network, {
                width: 432,
                height: 432,
                margin: 5
              })
            )}
          />
        </div>
      </div>
    `
  }

  showAllWallets() {
    const oppositeState = this.showAll
    this.isTopWalletsView = oppositeState
    this.showAll = !oppositeState
    this.isQrPageVisible = false
    this.requestUpdate()
  }
  renderFullPage() {
    return html`
      <div class="nightly-headerContainer">
        <nightly-header-small-page onClose=${this.onClose}></nightly-header-small-page>
      </div>
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
                  @click=${(event: Event) => this.handleWalletSelectorClick(event, item.name)}
                ></nightly-wallet-selector-item>
              </div>
            `
          })}
        </div>
      </div>
    `
  }

  renderNotFoundIcon() {
    return html`
      <div class="nightly-headerContainer">
        <nightly-header-small-page onClose=${this.onClose}></nightly-header-small-page>
      </div>
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
  handleWalletSelectorClick(_event: Event, name: string) {
    console.log('A menu item was clicked:', name)
  }
}
