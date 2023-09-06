import { LitElement, html } from 'lit'
import { customElement, property, query, state } from 'lit/decorators.js'
import { tailwindElement } from '../../shared/tailwind.element'
import style from './nightly-selector.css'
import { SelectorView, WalletSelectorItem } from '../../utils/types'
import { styleMap } from 'lit/directives/style-map.js'
import '../nightly-modal/nightly-modal'
import '../nightly-connect-wallet/nightly-connect-wallet'
import '../nightly-header/nightly-header'
import '../nightly-wallet-selector-page/nightly-all-wallets-selector/nightly-all-wallets-selector'
import '../nightly-wallet-selector-page/nightly-qrCode/nightly-qrCode'
import '../nightly-wallet-selector-page/nightly-wallet-wrapper/nightly-wallet-wrapper'

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
  link = ''

  @state()
  walletIcon = ''

  @state()
  currentWalletName = ''

  @state()
  hasMovedOnceToConnecting = false

  @state()
  currentView = SelectorView.DESKTOP_SELECT

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
      window.matchMedia('(max-width: 640px)').matches ? 240 : 80
    )
  }

  onSelectWallet = (name: string) => {
    const wallet = this.selectorItems.find((w) => w.name === name)

    this.walletIcon = wallet?.icon ?? ''
    this.currentWalletName = wallet?.name ?? ''
    this.link = wallet?.link ?? ''

    this.currentView = SelectorView.CONNECTING

    this.hasMovedOnceToConnecting = true

    this.onWalletClick(name)
  }

  tryAgainClick = () => {
    this.onSelectWallet(this.currentWalletName)
  }

  backToPage = () => {
    if (window.matchMedia('(max-width: 640px)').matches) {
      this.currentView = SelectorView.MOBILE_INIT
    } else {
      this.currentView = SelectorView.DESKTOP_SELECT
    }
  }

  returnToMobileInit = () => {
    this.currentView = SelectorView.MOBILE_INIT
  }

  goToMobileQr = () => {
    this.currentView = SelectorView.MOBILE_QR
  }

  goToMobileAll = () => {
    this.currentView = SelectorView.MOBILE_ALL
  }

  // lifecycle callbacks

  constructor() {
    super()

    this.handleClose = this.handleClose.bind(this)
    this.onSelectWallet = this.onSelectWallet.bind(this)
    this.tryAgainClick = this.tryAgainClick.bind(this)
    this.backToPage = this.backToPage.bind(this)
    this.returnToMobileInit = this.returnToMobileInit.bind(this)
    this.goToMobileAll = this.goToMobileAll.bind(this)
    this.goToMobileQr = this.goToMobileQr.bind(this)

    if (window.matchMedia('(max-width: 640px)').matches) {
      this.currentView = SelectorView.MOBILE_INIT
    }

    window.addEventListener('resize', () => {
      if (window.innerWidth <= 640 && this.currentView === SelectorView.DESKTOP_SELECT) {
        this.currentView = SelectorView.MOBILE_INIT
      } else if (
        window.innerWidth > 640 &&
        (this.currentView === SelectorView.MOBILE_INIT ||
          this.currentView === SelectorView.MOBILE_QR ||
          this.currentView === SelectorView.MOBILE_ALL)
      ) {
        this.currentView = SelectorView.DESKTOP_SELECT
      }
    })
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
    if (window.matchMedia('(max-width: 640px)').matches) {
      this.currentView = SelectorView.MOBILE_INIT
    } else {
      this.currentView = SelectorView.DESKTOP_SELECT
    }
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

  renderDesktop() {
    return html`
      <nightly-modal
        id="modalDesktop"
        class="${this.hasMovedOnceToConnecting && this.currentView !== SelectorView.CONNECTING
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

  renderMobileAll() {
    return html`<div class="walletsView">
      <nightly-all-wallets-selector
        class="selectorView"
        .showAllWallets=${this.returnToMobileInit.bind(this)}
        .onWalletClick=${this.onSelectWallet}
        .selectorItems=${this.selectorItems}
      ></nightly-all-wallets-selector>
    </div>`
  }

  renderMobileInit() {
    return html`<div class="topView">
      <nightly-wallet-wrapper
        class="selectorView"
        .sessionId=${this.sessionId}
        .showAllWallets=${this.goToMobileAll}
        .onWalletClick=${this.onSelectWallet}
        .openQrPage=${this.goToMobileQr}
        .selectorItems=${this.selectorItems}
      ></nightly-wallet-wrapper>
    </div>`
  }

  renderMobileQr() {
    return html`<div class="qrView">
      <nightly-qr-code
        class="selectorView"
        .chainName=${this.chainName}
        .sessionId=${this.sessionId}
        .relay=${this.relay}
        .showAllWallets=${this.returnToMobileInit}
      ></nightly-qr-code>
    </div>`
  }

  renderCurrent() {
    switch (this.currentView) {
      case SelectorView.DESKTOP_SELECT:
        return this.renderDesktop()
      case SelectorView.CONNECTING:
        return this.renderConnect()
      case SelectorView.MOBILE_INIT:
        return this.renderMobileInit()
      case SelectorView.MOBILE_QR:
        return this.renderMobileQr()
      case SelectorView.MOBILE_ALL:
        return this.renderMobileAll()
    }
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
                    height: this.mobileContentHeight + 'px'
                  }
                : {}
            )}
          >
            <div id="innerHeightObserverEl">${this.renderCurrent()}</div>
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
