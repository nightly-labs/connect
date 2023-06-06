import { NightlyWalletSelectorItem } from './../nightly-wallet-selector-item/nightly-wallet-selector-item'
import { customElement } from 'lit/decorators.js'
import { html } from 'lit/static-html.js'
import search from '../../static/svg/searchIcon.svg'
import Phantom from '../../static/svg/PhantomIcon.svg'
import MetaMask from '../../static/svg/MetaMaskIcon.svg'
import Coinbase from '../../static/svg/CoinbaseIcon.svg'
import Glow from '../../static/svg/GlowIcon.svg'
import ZenGO from '../../static/svg/ZenGOIcon.svg'
import Trust from '../../static/svg/TrustIcon.svg'
import Binance from '../../static/svg/BinanceIcon.svg'
import Sollet from '../../static/svg/SolletIcon.svg'
import NightlyIcon from '../../static/svg/NightlyIcon.svg'
import { css } from 'lit'
import NotFoundIcon from '../../static/svg/nothingFound.svg'

@customElement('nightly-wallet-selector-page')
export class NightlyWalletSelectorPage extends NightlyWalletSelectorItem {
  static styles = css`
    .walletSelectorPage {
      display: flex;
      flex-direction: column;
      justify-content: center;
      align-items: center;
      width: 288px;
      height: 464px;
      background-color: #202137;
    }
    .walletInputSearchContainer {
      display: flex;
      justify-content: space-between;
      align-items: center;
      margin: 0 16px;
      padding: 8px;
      width: 256px;
      height: 20px;
      background-color: #0f0f1a;
      border-radius: 8px;
    }
    .walletInputSearchContainer img {
      width: 20px;
      height: 20px;
    }
    .walletInputSearch {
      background-color: #0f0f1a;
      color: #707a8d;
      font-size: 12px;
      font-weight: 400;
      width: 99%;
      line-height: 22px;
      letter-spacing: 0.02em;
      border: 0;
    }
    .walletInputSearch:focus {
      outline: none;
    }
    .walletSelectorButtons {
      display: grid;
      grid-template-columns: 1fr 1fr 1fr;
      justify-content: center;
      align-items: center;
      width: 264px;

      padding: 8px 12px 0px 12px;
    }
    .NotFoundContainer {
      padding-top: 16px;
      display: flex;
      flex-direction: column;
      align-items: center;
      justify-content: center;
    }
    .NotFoundHeading {
      padding-top: 16px;
      color: #f7f7f7;
      font-weight: 700;
      font-size: 16px;
      line-height: 22px;
      letter-spacing: 0.02em;
    }
    .NotFoundInfo {
      padding-top: 4px;
      width: 171px;
      color: #707a8d;
      text-align: center;
      font-weight: 400;
      font-size: 12px;
      line-height: 16px;
      letter-spacing: 0.02em;
    }
  `

  selectorItems = [
    { name: 'Phantom', icon: Phantom, status: '' },
    { name: 'MetaMask', icon: MetaMask, status: '' },
    { name: 'Coinbase', icon: Coinbase, status: '' },
    { name: 'Nightly Wallet', icon: NightlyIcon, status: '' },
    { name: 'Glow Wallet', icon: Glow, status: '' },
    { name: 'ZenGO', icon: ZenGO, status: '' },
    { name: 'Trust', icon: Trust, status: '' },
    { name: 'Binance Wallet', icon: Binance, status: '' },
    { name: 'Sollet', icon: Sollet, status: '' }
  ]

  filteredItems = [...this.selectorItems]
  showNotFoundIcon = false

  render() {
    return html`
      <div class="walletSelectorPage">
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
    return this.filteredItems.map((item) => {
      return html`
        <nightly-wallet-selector-item
          name=${item.name}
          icon=${item.icon}
          status=${item.status}
          @click=${this.handleWalletSelectorClick}
        ></nightly-wallet-selector-item>
      `
    })
  }

  renderNotFoundIcon() {
    return html`
      <div class="NotFoundContainer">
        <img src="${NotFoundIcon}" alt="Not Found" />
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

    this.requestUpdate() // Odświeżenie renderowania komponentu
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
