import { LitElement, html } from 'lit'
import { customElement, property, state } from 'lit/decorators.js'
import { tailwindElement } from '../../shared/tailwind.element'
import '../nightly-modal/nightly-modal'
import style from './nightly-main-page.css'
import '../nightly-connect-wallet/nightly-connect-wallet'
import '../nightly-wallet-selector-page/nightly-wallet-selector-small-page/nightly-wallet-selector-small-page'
import '../nightly-header/nightly-header'
import { animate } from '@lit-labs/motion'

@customElement('nightly-main-page')
export class NightlyMainPage extends LitElement {
  static styles = tailwindElement(style)

  @property()
  // eslint-disable-next-line @typescript-eslint/no-empty-function
  onClose = () => {}

  @property({ type: Array })
  selectorItems = []

  @property({ type: Function })
  onWalletClick(name: string): void {
    console.log('Item clicked:', name)
  }
  @property({ type: String })
  chainIcon = ''

  @property({ type: String })
  chainName = ''

  @property({ type: String })
  sessionId = ''

  @property({ type: String })
  network = ''

  @property({ type: String })
  copyMessage = 'Copy'

  @property({ type: Boolean })
  connecting = false

  @property({ type: Boolean })
  connected = false

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
  fallback = () => {}

  @property()
  // eslint-disable-next-line @typescript-eslint/no-empty-function
  tryAgainClick = () => {}

  @state()
  openWalletConncet = false

  openConnectWallet() {
    this.openWalletConncet = true
  }

  constructor() {
    super()
    this.onWalletClick = this.onWalletClick.bind(this)
    this.openConnectWallet = this.openConnectWallet.bind(this)
  }

  renderConnect() {
    return html`
      <nightly-connect-wallet
        class="modalConnect"
        .coinName=${this.coinName}
        .connecting=${this.connecting}
        .tryAgainClick=${this.tryAgainClick}
        .fallback=${this.backToPage}
        .link=${this.link}
        .nameLink=${this.nameLink}
        .walletIcon=${this.walletIcon}
        ${animate({
          properties: ['height', 'opacity', 'transform'],
          in: [
            {
              opacity: 0,
              transform: 'scale(0.9)'
            },
            {
              offset: 0.1,
              opacity: 0,
              transform: 'scale(0.9)'
            },
            {
              offset: 1,
              opacity: 1,
              transform: 'scale(1)'
            }
          ]
        })}
      ></nightly-connect-wallet>
    `
  }

  renderSelect() {
    return html` <nightly-wallet-selector-small-page
        class="modalMobile"
        .hasUpdated=${this.hasUpdated}
        .isUpdatePending=${this.isUpdatePending}
        .network=${this.network}
        .onWalletClick=${this.openConnectWallet}
        .onClose=${this.onClose}
        .selectorItems=${this.selectorItems}
        .sessionId=${this.sessionId}
        ${animate({
          properties: ['height', 'opacity', 'transform'],
          skipInitial: true,
          in: [
            {
              opacity: 0,
              transform: 'scale(0.9)'
            },
            {
              offset: 0.1,
              opacity: 0,
              transform: 'scale(0.9)'
            },
            {
              offset: 1,
              opacity: 1,
              transform: 'scale(1)'
            }
          ]
        })}
      ></nightly-wallet-selector-small-page>
      <nightly-modal
        class="modalDesktop"
        .chainIcon=${this.chainIcon}
        .chainName=${this.chainName}
        .copyMessage=${this.copyMessage}
        .hasUpdated=${this.hasUpdated}
        .isUpdatePending=${this.isUpdatePending}
        .network=${this.network}
        .onClose=${this.onClose}
        .onWalletClick=${this.openConnectWallet}
        .selectorItems=${this.selectorItems}
        .sessionId=${this.sessionId}
        ${animate({
          properties: ['height', 'opacity', 'transform'],
          skipInitial: true,
          in: [
            {
              opacity: 0,
              transform: 'scale(0.9)'
            },
            {
              offset: 0.1,
              opacity: 0,
              transform: 'scale(0.9)'
            },
            {
              offset: 1,
              opacity: 1,
              transform: 'scale(1)'
            }
          ]
        })}
      ></nightly-modal>`
  }

  render() {
    return html`
      <div class="nightlyModal">
        <nightly-header .onClose=${this.onClose}></nightly-header>
        <div class="contentWrapper">
          ${this.openWalletConncet ? this.renderConnect() : this.renderSelect()}
        </div>
      </div>
    `
  }

  backToPage = () => {
    this.openWalletConncet = false
    this.requestUpdate()
  }
}

declare global {
  interface HTMLElementTagNameMap {
    'nightly-main-page': NightlyMainPage
  }
}
