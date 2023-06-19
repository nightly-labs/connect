import { customElement, property } from 'lit/decorators.js'
import { TailwindElement } from '../../shared/tailwind.element'
import style from './nightly-connect-wallet.css?inline'
import { html } from 'lit'
import '../nightly-header/nightly-header'
import binance from '../../static/svg/BinanceIcon.svg'

@customElement('nightly-connect-wallet')
export class NightlyConnectWallet extends TailwindElement(style) {
  @property({ type: String })
  nameLink = ''

  @property({ type: String })
  coinName = ''

  render() {
    return html`
      <nightly-header></nightly-header>
      <div class="mainContainer">
        <div class="headerContainer">
          <div class="buttonContainer">
            <button>b</button>
          </div>
          <div class="textContainer">
            <span>Connect wallet</span>
          </div>
        </div>
        <div class="coinInfoContainer">
          <img src=${binance} />
          <span class="coinName">${this.coinName}</span>
          <div class="custom-loader"></div>

          <span class="connectInfo">Connecting failed</span>
        </div>
        <div class="reConnectWrapper">
          <p>
            Connecting takes too long? Make sure ${this.nameLink} is installed on your device.
            Otherwise, visit
            <a class="link" href="${this.nameLink}" @click=${this._handleClick}>${this.nameLink}</a>
            to download it.
          </p>
          <button>Try again</button>
        </div>
      </div>
    `
  }
  _handleClick() {
    console.log('Link clicked')
  }

  //   showAllWallets() { @click=${this.showAllWallets}   <img src=${vector} />}
}

declare global {
  interface HTMLElementTagNameMap {
    'nightly-connect-wallet': NightlyConnectWallet
  }
}
