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

  @state()
  hasMovedToConnecting = false

  // queried elements

  @query('#innerHeightObserverEl')
  _innerHeightObserverEl!: HTMLElement

  // observers
  innerHeightObserver: ResizeObserver | undefined

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

    this.hasMovedToConnecting = true

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
  connectedCallback(): void {
    super.connectedCallback()
    setTimeout(() => {
      this.mobileContentHeight = Math.max(this._innerHeightObserverEl.scrollHeight, 186)
      if (!this.innerHeightObserver) {
        this.innerHeightObserver = new ResizeObserver(() => {
          if (!this._innerHeightObserverEl) {
            return
          }
          this.mobileContentHeight = Math.max(this._innerHeightObserverEl.scrollHeight, 186)
        })
      }
      this.innerHeightObserver.observe(this._innerHeightObserverEl)
    }, 0)
  }

  disconnectedCallback(): void {
    super.disconnectedCallback()
    this.fireClosingAnimation = false
    this.useConnectTransition = false
    this.connectingViewOpen = false
    this.mobileContentHeight = 186
  }

  renderConnect() {
    return html`
      <nightly-connect-wallet
        id="modalConnect"
        .coinName=${this.currentWalletName}
        .connecting=${this.connecting}
        .tryAgainClick=${this.tryAgainClick}
        .fallback=${this.backToPage}
        .link=${this.link}
        .nameLink=${this.currentWalletName}
        .walletIcon=${this.walletIcon}
      ></nightly-connect-wallet>
    `
  }

  renderSelect() {
    return html`
      <nightly-wallet-selector-small-page
        id="modalMobile"
        class="modalMobile ${this.hasMovedToConnecting && !this.connectingViewOpen
          ? 'mobileFade'
          : ''}"
        .onWalletClick=${this.onSelectWallet}
        .selectorItems=${this.selectorItems}
        .sessionId=${this.sessionId}
        .chainName=${this.chainName}
        .relay=${this.relay}
      ></nightly-wallet-selector-small-page>
      <nightly-modal
        id="modalDesktop"
        class="modalDesktop ${this.hasMovedToConnecting && !this.connectingViewOpen
          ? 'desktopFade'
          : ''}"
        .chainIcon=${this.chainIcon}
        .chainName=${this.chainName}
        .onWalletClick=${this.onSelectWallet}
        .selectorItems=${this.selectorItems}
        .sessionId=${this.sessionId}
        .relay=${this.relay}
      ></nightly-modal>
    `
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
            <div id="innerHeightObserverEl">
              ${this.connectingViewOpen ? this.renderConnect() : this.renderSelect()}
            </div>
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
