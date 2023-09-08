import { LitElement, html } from 'lit'
import { customElement, property } from 'lit/decorators.js'
import { unsafeHTML } from 'lit/directives/unsafe-html.js'
import { tailwindElement } from '../../shared/tailwind.element'
import vector from '../../static/svg/backButton.svg'
import style from './nightly-connect-wallet.css'

@customElement('nightly-connect-wallet')
export class NightlyConnectWallet extends LitElement {
  static styles = tailwindElement(
    style,
    `
  .nc_connectBackButton {
    background-image: url("${vector}");
  }
  `
  )

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
  goBack = () => {}

  render() {
    return html`
      <div class="nc_connectWrapper">
        <div class="nc_connectTopBar">
          <button class="nc_connectBackButton" @click=${this.goBack}></button>
          <span class="nc_connectTitle">Connect wallet</span>
          <div class="nc_connectTopJustify"></div>
        </div>
        <div class="nc_connectWalletInfo">
          <img class="nc_connectWalletIcon" src=${this.walletIcon} />
          <span class="nc_connectWalletName">${this.coinName}</span>
          ${this.connecting
            ? html`<div class="nc_connectProgress">
                Connecting...
                <div class="nc_connectProgressLoader"></div>
              </div>`
            : html` <span class="nc_connectFail">Connecting failed</span> `}
        </div>
        <div class="nc_connectBottomInfo">
          <p class="nc_connectBottomInfoText">
            Connecting takes too long? Make sure ${this.nameLink} is installed on your device.
            ${this.link.length
              ? unsafeHTML(`Otherwise, visit
            <a class="nc_connectWalletLink" href="${this.link}"
              >${this.nameLink} website</a
            >
            to download it.`)
              : ''}
          </p>
          <button class="nc_connectTryAgainButton" @click=${this.tryAgainClick}>Try again</button>
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
