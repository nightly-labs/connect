import { customElement, property } from 'lit/decorators.js'
import { html } from 'lit/static-html.js'
import { TailwindElement } from '../../shared/tailwind.element'
import foxSadGIF from '../../static/gif/fox_sad.gif'
import Binance from '../../static/svg/BinanceIcon.svg'
import Coinbase from '../../static/svg/CoinbaseIcon.svg'
import Glow from '../../static/svg/GlowIcon.svg'
import MetaMask from '../../static/svg/MetaMaskIcon.svg'
import NightlyIcon from '../../static/svg/NightlyIcon.svg'
import Phantom from '../../static/svg/PhantomIcon.svg'
import Sollet from '../../static/svg/SolletIcon.svg'
import Trust from '../../static/svg/TrustIcon.svg'
import ZenGO from '../../static/svg/ZenGOIcon.svg'
import search from '../../static/svg/searchIcon.svg'
import style from './nightly-wallet-selector-page.css?inline'
import ChainIcon from '../../static/svg/ChainIcon.svg'

@customElement('nightly-wallet-selector-page')
export class NightlyWalletSelectorPage extends TailwindElement(style) {
  @property({ type: String })
  name = ''

  @property({ type: String })
  status = ''

  selectorItems = [
    { name: 'Phantom', icon: Phantom, status: 'detected' },
    { name: 'MetaMask', icon: MetaMask, status: '' },
    { name: 'Coinbase', icon: Coinbase, status: 'recent' },
    { name: 'Nightly Wallet', icon: NightlyIcon, status: 'detected' },
    { name: 'Glow Wallet', icon: Glow, status: '' },
    { name: 'ZenGO', icon: ZenGO, status: '' },
    { name: 'Trust', icon: Trust, status: '' },
    { name: 'Binance Wallet', icon: Binance, status: 'detected' },
    { name: 'Binance Wallet', icon: Binance, status: '' },
    { name: 'Binance Wallet', icon: Binance, status: '' },
    { name: 'Binance Wallet', icon: Binance, status: '' },
    { name: 'Binance Wallet', icon: Binance, status: '' },
    { name: 'Binance Wallet', icon: Binance, status: '' },
    { name: 'Binance Wallet', icon: Binance, status: '' },
    { name: 'Binance Wallet', icon: Binance, status: '' },
    { name: 'Sollet', icon: Sollet, status: 'recent' }
  ]

  filteredItems = [...this.selectorItems]
  showNotFoundIcon = false

  render() {
    return html`
      <div class="walletSelectorPage">
        <div class="walletSelectorHeader">
          <span>Wallets</span>
          <div class="walletSelectorBlockchain">
            <img src=${ChainIcon} />
            <span>Solana</span>
          </div>
        </div>
        <div class="walletInputSearchContainer">
          <input placeholder="Search" class="walletInputSearch" @input=${this.handleSearchInput} />
          <img src="${search}" />
        </div>
        <div class="${this.showNotFoundIcon ? 'NotFoundContainer' : 'walletSelectorButtons'}">
          ${this.showNotFoundIcon ? this.renderNotFoundIcon() : this.renderSelectorItems()}
        </div>
      </div>
    `
  }

  renderSelectorItems() {
    const sortedItems = this.filteredItems.sort((a, b) => {
      if (a.status === 'recent' || a.status === 'detected') return -1
      if (b.status === 'recent' || b.status === 'detected') return 1
      return 0
    })

    const recentDetectedItems = sortedItems.filter(
      (item) => item.status === 'recent' || item.status === 'detected'
    )
    const otherItems = sortedItems.filter(
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
                @click=${this.handleWalletSelectorClick}
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
                @click=${this.handleWalletSelectorClick}
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

  handleWalletSelectorClick(event: { target: { name: string } }) {
    console.log('Kliknięto element menu:', event.target.name)
  }
}

declare global {
  interface HTMLElementTagNameMap {
    'nightly-wallet-selector-page': NightlyWalletSelectorPage
  }
}
