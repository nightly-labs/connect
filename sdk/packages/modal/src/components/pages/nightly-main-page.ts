import { LitElement, html } from 'lit'
import { customElement, property, state, query } from 'lit/decorators.js'
import { tailwindElement } from '../../shared/tailwind.element'
import '../nightly-modal/nightly-modal'
import style from './nightly-main-page.css'
import '../nightly-connect-wallet/nightly-connect-wallet'
import '../nightly-wallet-selector-page/nightly-wallet-selector-small-page/nightly-wallet-selector-small-page'
import '../nightly-header/nightly-header'
import { animate } from '@lit-labs/motion'
import { styleMap } from 'lit/directives/style-map.js'

@customElement('nightly-main-page')
export class NightlyMainPage extends LitElement {
  static styles = tailwindElement(style)

  @property()
  // eslint-disable-next-line @typescript-eslint/no-empty-function
  onClose = () => {}

  @property({ type: Array })
  selectorItems = []

  @property({ type: Function })
  // eslint-disable-next-line @typescript-eslint/no-empty-function
  onWalletClick: (name: string) => void = () => {}

  @property({ type: String })
  chainIcon = ''

  @property({ type: String })
  chainName = ''

  @property({ type: String })
  sessionId = ''

  @property({ type: String })
  network = ''

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
  tryAgainClick = () => {}

  @state()
  openWalletConncet = false

  openConnectWallet() {
    this.openWalletConncet = true
  }

  constructor() {
    super()
    this.openConnectWallet = this.openConnectWallet.bind(this)
  }

  @query('#modalConnect')
  _modalConnect!: HTMLElement

  @query('#modalSelect')
  _modalSelect!: HTMLElement

  @state()
  mobileContentHeight = 318

  connectObserver: ResizeObserver | undefined
  selectObserver: ResizeObserver | undefined

  renderConnect() {
    setTimeout(() => {
      this.mobileContentHeight = this._modalConnect.scrollHeight
      if (!this.connectObserver) {
        this.connectObserver = new ResizeObserver(() => {
          if (!this._modalConnect) {
            return
          }
          this.mobileContentHeight = this._modalConnect.scrollHeight
        })
      }
      this.connectObserver.observe(this._modalConnect)
    }, 0)
    return html`
      <nightly-connect-wallet
        id="modalConnect"
        class="modalConnect"
        .coinName=${this.coinName}
        .connecting=${this.connecting}
        .tryAgainClick=${this.tryAgainClick}
        .fallback=${this.backToPage}
        .link=${this.link}
        .nameLink=${this.nameLink}
        .walletIcon=${this.walletIcon}
        ${animate({
          properties: ['opacity', 'transform'],
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
    setTimeout(() => {
      this.mobileContentHeight = this._modalSelect.scrollHeight
      if (!this.selectObserver) {
        this.selectObserver = new ResizeObserver(() => {
          if (!this._modalSelect) {
            return
          }
          this.mobileContentHeight = this._modalSelect.scrollHeight
        })
      }
      this.selectObserver.observe(this._modalSelect)
    }, 0)
    return html`<div id="modalSelect">
      <nightly-wallet-selector-small-page
        class="modalMobile"
        .network=${this.network}
        .onWalletClick=${this.openConnectWallet}
        .onClose=${this.onClose}
        .selectorItems=${this.selectorItems}
        .sessionId=${this.sessionId}
        ${animate({
          properties: ['opacity', 'transform'],
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
        .network=${this.network}
        .onClose=${this.onClose}
        .onWalletClick=${this.openConnectWallet}
        .selectorItems=${this.selectorItems}
        .sessionId=${this.sessionId}
        ${animate({
          properties: ['opacity', 'transform'],
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
      ></nightly-modal>
    </div>`
  }

  render() {
    return html`
      <div class="nightlyModal">
        <nightly-header .onClose=${this.onClose}></nightly-header>
        <div
          id="contentWrapper"
          class="contentWrapper"
          style=${styleMap(
            window.innerWidth <= 640 ? { height: this.mobileContentHeight + 'px' } : {}
          )}
        >
          ${this.openWalletConncet ? this.renderConnect() : this.renderSelect()}
        </div>
      </div>
    `
  }

  backToPage = () => {
    this.openWalletConncet = false
  }
}

declare global {
  interface HTMLElementTagNameMap {
    'nightly-main-page': NightlyMainPage
  }
}
