import { LitElement, html } from 'lit'
import { customElement, property, query, state } from 'lit/decorators.js'
import { tailwindElement } from '../../shared/tailwind.element'
import style from './nightly-selector.css'
import { WalletSelectorItem } from '../../utils/types'
import { styleMap } from 'lit/directives/style-map.js'
import { animate } from '@lit-labs/motion'
import '../nightly-modal/nightly-modal'
import '../nightly-connect-wallet/nightly-connect-wallet'
import '../nightly-wallet-selector-page/nightly-wallet-selector-small-page/nightly-wallet-selector-small-page'
import '../nightly-header/nightly-header'

@customElement('nightly-selector')
export class NightlySelector extends LitElement {
  static styles = tailwindElement(style)

  // props

  @property()
  // eslint-disable-next-line @typescript-eslint/no-empty-function
  onClose = () => {}

  @property({ type: Array })
  selectorItems: WalletSelectorItem[] = []

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
  relay = ''

  @property({ type: Boolean })
  connecting = false

  // state

  @state()
  fireClosingAnimation = false

  @state()
  mobileContentHeight = 186

  @state()
  useConnectTransition = false

  @state()
  connectingViewOpen = false

  @state()
  link = ''

  @state()
  walletIcon = ''

  @state()
  currentWalletName = ''

  // queried elements
  @query('#modalConnect')
  _modalConnect!: HTMLElement

  @query('#modalSelect')
  _modalSelect!: HTMLElement

  // observers

  connectObserver: ResizeObserver | undefined
  selectObserver: ResizeObserver | undefined

  // callbacks

  handleClose = () => {
    this.fireClosingAnimation = true
    setTimeout(
      () => {
        this.onClose()
      },
      window.matchMedia('(max-width: 640px)') ? 240 : 80
    )
  }

  onSelectWallet = (name: string) => {
    const wallet = this.selectorItems.find((w) => w.name === name)

    this.walletIcon = wallet?.icon ?? ''
    this.currentWalletName = wallet?.name ?? ''
    this.link = wallet?.link ?? ''

    this.useConnectTransition = true

    this.connectingViewOpen = true

    this.onWalletClick(name)
  }

  tryAgainClick = () => {
    this.onSelectWallet(this.currentWalletName)
  }

  backToPage = () => {
    setTimeout(() => {
      this.useConnectTransition = false
    }, 300)
    this.connectingViewOpen = false
  }

  // lifecycle callbacks

  constructor() {
    super()
    this.handleClose = this.handleClose.bind(this)
    this.onSelectWallet = this.onSelectWallet.bind(this)
    this.tryAgainClick = this.tryAgainClick.bind(this)
    this.backToPage = this.backToPage.bind(this)
  }

  disconnectedCallback(): void {
    super.disconnectedCallback()
    this.fireClosingAnimation = false
    this.useConnectTransition = false
    this.connectingViewOpen = false
    this.mobileContentHeight = 186
  }

  renderConnect() {
    setTimeout(() => {
      this.mobileContentHeight = Math.max(this._modalConnect.scrollHeight, 186)
      if (!this.connectObserver) {
        this.connectObserver = new ResizeObserver(() => {
          if (!this._modalConnect) {
            return
          }
          this.mobileContentHeight = Math.max(this._modalConnect.scrollHeight, 186)
        })
      }
      this.connectObserver.observe(this._modalConnect)
    }, 0)
    return html`
      <nightly-connect-wallet
        id="modalConnect"
        class="modalConnect"
        .coinName=${this.currentWalletName}
        .connecting=${this.connecting}
        .tryAgainClick=${this.tryAgainClick}
        .fallback=${this.backToPage}
        .link=${this.link}
        .nameLink=${this.currentWalletName}
        .walletIcon=${this.walletIcon}
        ${animate({
          properties: ['opacity'],
          keyframeOptions: { duration: 320 },
          skipInitial: true,
          in: [
            {
              opacity: 0
            },
            {
              offset: 0.25,
              opacity: 0
            },
            {
              offset: 1,
              opacity: 1
            }
          ]
        })}
      ></nightly-connect-wallet>
    `
  }

  renderSelect() {
    setTimeout(() => {
      this.mobileContentHeight = Math.max(this._modalSelect.scrollHeight, 186)
      if (!this.selectObserver) {
        this.selectObserver = new ResizeObserver(() => {
          if (!this._modalSelect) {
            return
          }
          this.mobileContentHeight = Math.max(this._modalSelect.scrollHeight, 186)
        })
      }
      this.selectObserver.observe(this._modalSelect)
    }, 0)
    return html`<div id="modalSelect">
      <nightly-wallet-selector-small-page
        class="modalMobile"
        .onWalletClick=${this.onSelectWallet}
        .selectorItems=${this.selectorItems}
        .sessionId=${this.sessionId}
        .chainName=${this.chainName}
        .relay=${this.relay}
        ${animate({
          properties: ['opacity'],
          keyframeOptions: { duration: 320 },
          skipInitial: true,
          in: [
            {
              opacity: 0
            },
            {
              offset: 0.25,
              opacity: 0
            },
            {
              offset: 1,
              opacity: 1
            }
          ]
        })}
      ></nightly-wallet-selector-small-page>
      <nightly-modal
        class="modalDesktop"
        .chainIcon=${this.chainIcon}
        .chainName=${this.chainName}
        .onWalletClick=${this.onSelectWallet}
        .selectorItems=${this.selectorItems}
        .sessionId=${this.sessionId}
        .relay=${this.relay}
        ${animate({
          properties: ['opacity'],
          keyframeOptions: { duration: 250 },
          skipInitial: true,
          in: [
            {
              opacity: 0
            },
            {
              offset: 0.25,
              opacity: 0
            },
            {
              offset: 1,
              opacity: 1
            }
          ]
        })}
      ></nightly-modal>
    </div>`
  }

  render() {
    return html`
      <div
        class="nightlySelectorOverlay ${this.fireClosingAnimation ? 'fadeOutOpacity' : ''}"
        @click=${this.handleClose}
      >
        <div
          @click=${(e: MouseEvent) => {
            e.stopPropagation()
          }}
          class="nightlySelectorWrapper ${this.fireClosingAnimation ? 'slideOutMobile' : ''}"
        >
          <nightly-header .onClose=${this.handleClose}></nightly-header>
          <div
            class="nightlySelectorContent"
            style=${styleMap(
              window.innerWidth <= 640
                ? {
                    height: this.mobileContentHeight + 'px',
                    transition: this.useConnectTransition ? 'height 250ms' : 'none'
                  }
                : {}
            )}
            ${animate({
              properties: ['height'],
              keyframeOptions: { duration: 0 },
              skipInitial: true
            })}
          >
            ${this.connectingViewOpen ? this.renderConnect() : this.renderSelect()}
          </div>
        </div>
      </div>
    `
  }
}

declare global {
  interface HTMLElementTagNameMap {
    'nightly-selector': NightlySelector
  }
}
