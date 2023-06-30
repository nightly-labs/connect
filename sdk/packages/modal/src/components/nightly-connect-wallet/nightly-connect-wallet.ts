import { LitElement, html } from 'lit'
import { customElement, property } from 'lit/decorators.js'
import { tailwindElement } from '../../shared/tailwind.element'
import vector from '../../static/svg/backButton.svg'
import { Breakpoint, getBreakpointFromWidthInConnectWallet } from '../../utils/utils'
import style from './nightly-connect-wallet.css'
@customElement('nightly-connect-wallet')
export class NightlyConnectWallet extends LitElement {
  static styles = tailwindElement(style)

  @property({ type: Boolean })
  connecting = false

  @property({ type: String })
  nameLink = ''

  @property({ type: String })
  link = ''

  @property({ type: String })
  walletIcon = ''

  @property({ type: String })
  coinName = ''

  @property()
  // eslint-disable-next-line @typescript-eslint/no-empty-function
  tryAgainClick = () => {}

  @property()
  // eslint-disable-next-line @typescript-eslint/no-empty-function
  fallback = () => {}

  render() {
    return html`
      <div class="mainContainer">
        <div class="wrapperConnectPage">
          <div class="headerContainer">
            <div class="buttonContainer">
              <button id="connect-wallet-fallback-button" @click=${this.fallback}>
                <img class="vector" src=${vector} />
              </button>
            </div>
            <div class="textContainer">
              <span>Connect wallet</span>
            </div>
          </div>
          <div class="coinInfoContainer">
            <img src=${this.walletIcon} />
            <span class="coinName">${this.coinName}</span>
            ${this.connecting
              ? html` <div class="connectingContainer">
                  <span>Connecting... </span>
                  <div class="custom-loader"></div>
                </div>`
              : html` <span class="error">Connecting failed</span> `}
          </div>
          <div class="reConnectWrapper">
            <p>
              Connecting takes too long? Make sure ${this.nameLink} is installed on your device.
              Otherwise, visit
              <a id="connect-wallet-page-link-wallet-website" class="link" href="${this.link}"
                >${this.nameLink} website</a
              >
              to download it.
            </p>
            <button id="connect-wallet-page-try-again-button" @click=${this.tryAgainClick}>
              Try again
            </button>
          </div>
        </div>
      </div>
    `
  }
}

declare global {
  interface HTMLElementTagNameMap {
    'nightly-connect-wallet': NightlyConnectWallet
  }
}
