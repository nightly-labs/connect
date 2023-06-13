import { customElement, property } from 'lit/decorators.js'
import { html } from 'lit/static-html.js'
import { TailwindElement } from '../../../shared/tailwind.element'
import style from './nightly-wallet-selector-small-page.css?inline'
import search from '../../../static/svg/searchIcon.svg'
import foxSadGIF from '../../../static/gif/fox_sad.gif'
import vector from '../../../static/svg/backButton.svg'
import '../../nightly-header/nightly-header'

@customElement('nightly-wallet-selector-small-page')
export class NightlyWalletSelectorSmallPage extends TailwindElement(style) {
  @property({ type: Boolean }) showAll = false
  @property({ type: String }) chainIcon = ''
  @property({ type: Array })
  @property({ type: String })
  searchInputValue = ''
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

  constructor() {
    super()
  }
  get topWallets(): { name: string; icon: string; status: string }[] {
    return this._selectorItems.slice(0, 7)
  }

  render() {
    return html`
      <div>
        ${this.showAll
          ? this.filteredItems.length > 0
            ? this.renderFullPage()
            : this.renderNotFoundIcon()
          : this.renderTopWallets()}
      </div>
    `
  }
  renderTopWallets() {
    return html`
      <nightly-header></nightly-header>
      <div class="mainContainer">
        <div class="infoConatiner">
          <p>Connect wallet</p>
          <button>QR Code</button>
        </div>
        <div class="mainContainerWalletSellector">
          ${this.topWallets.map(
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
    `
  }

  showAllWallets() {
    this.showAll = !this.showAll
    this.requestUpdate()
  }
  renderFullPage() {
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
              <nightly-wallet-selector-item
                name=${item.name}
                icon=${item.icon}
                status=${item.status}
                @click=${(event: Event) => this.handleWalletSelectorClick(event, item.name)}
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
