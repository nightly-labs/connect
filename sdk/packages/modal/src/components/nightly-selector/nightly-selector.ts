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
  canAnimateDesktopInitialView = false

  @state()
  currentView = SelectorView.DESKTOP_SELECT

  @state()
  isMobile = false

  // queried elements

  @query('#innerHeightObserverEl')
  _innerHeightObserverEl!: HTMLElement

  // media queries

  mobileQuery = window.matchMedia('(max-width: 640px)')
  smallerMobileQuery = window.matchMedia('(max-width: 482px)')
  smallestMobileQuery = window.matchMedia('(max-width: 374px)')

  // callbacks

  calcMobileContentHeight = () => {
    switch (this.currentView) {
      case SelectorView.MOBILE_QR:
        return this.smallestMobileQuery.matches ? 332 : this.smallerMobileQuery.matches ? 420 : 510
      case SelectorView.MOBILE_ALL:
        return 526
      case SelectorView.CONNECTING:
        return this.smallestMobileQuery.matches ? 440 : this.smallerMobileQuery.matches ? 420 : 400
      default:
        return 182
    }
  }

  setCurrentView = (val: SelectorView) => {
    this.currentView = val
    this.mobileContentHeight = this.calcMobileContentHeight()
  }

  handleClose = () => {
    this.fireClosingAnimation = true
    setTimeout(
      () => {
        this.onClose()
      },
      this.mobileQuery.matches ? 240 : 80
    )
  }

  onSelectWallet = (name: string) => {
    const wallet = this.selectorItems.find((w) => w.name === name)

    this.walletIcon = wallet?.icon ?? ''
    this.currentWalletName = wallet?.name ?? ''
    this.link = wallet?.link ?? ''

    this.setCurrentView(SelectorView.CONNECTING)

    this.canAnimateDesktopInitialView = true

    this.onWalletClick(name)
  }

  tryAgainClick = () => {
    this.onSelectWallet(this.currentWalletName)
  }

  backToPage = () => {
    if (this.mobileQuery.matches) {
      this.setCurrentView(SelectorView.MOBILE_INIT)
    } else {
      this.setCurrentView(SelectorView.DESKTOP_SELECT)
    }
  }

  returnToMobileInit = () => {
    this.setCurrentView(SelectorView.MOBILE_INIT)
  }

  goToMobileQr = () => {
    this.setCurrentView(SelectorView.MOBILE_QR)
  }

  goToMobileAll = () => {
    this.setCurrentView(SelectorView.MOBILE_ALL)
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

    if (this.mobileQuery.matches) {
      this.isMobile = true
      this.setCurrentView(SelectorView.MOBILE_INIT)
    }

    this.mobileQuery.addEventListener('change', () => {
      this.isMobile = this.mobileQuery.matches
      if (this.currentView !== SelectorView.CONNECTING) {
        this.setCurrentView(
          this.mobileQuery.matches ? SelectorView.MOBILE_INIT : SelectorView.DESKTOP_SELECT
        )
      }

      this.mobileContentHeight = this.calcMobileContentHeight()
    })

    this.smallerMobileQuery.addEventListener('change', () => {
      this.mobileContentHeight = this.calcMobileContentHeight()
    })

    this.smallestMobileQuery.addEventListener('change', () => {
      this.mobileContentHeight = this.calcMobileContentHeight()
    })
  }

  disconnectedCallback(): void {
    super.disconnectedCallback()
    this.fireClosingAnimation = false
    if (this.mobileQuery.matches) {
      this.setCurrentView(SelectorView.MOBILE_INIT)
    } else {
      this.setCurrentView(SelectorView.DESKTOP_SELECT)
    }
  }

  renderConnect() {
    return html`
      <nightly-connect-wallet
        id="modalConnect"
        class="fadeEntry"
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
        class="${this.canAnimateDesktopInitialView && this.currentView !== SelectorView.CONNECTING
          ? 'fadeEntry'
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
    return html`
      <nightly-all-wallets-selector
        class="fadeEntry"
        .showAllWallets=${this.returnToMobileInit.bind(this)}
        .onWalletClick=${this.onSelectWallet}
        .selectorItems=${this.selectorItems}
      ></nightly-all-wallets-selector>
    `
  }

  renderMobileInit() {
    return html`
      <nightly-wallet-wrapper
        class="fadeEntry"
        .sessionId=${this.sessionId}
        .showAllWallets=${this.goToMobileAll}
        .onWalletClick=${this.onSelectWallet}
        .openQrPage=${this.goToMobileQr}
        .selectorItems=${this.selectorItems}
      ></nightly-wallet-wrapper>
    `
  }

  renderMobileQr() {
    return html`
      <nightly-qr-code
        class="fadeEntry"
        .chainName=${this.chainName}
        .sessionId=${this.sessionId}
        .relay=${this.relay}
        .showAllWallets=${this.returnToMobileInit}
      ></nightly-qr-code>
    `
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
        class="nc_modalOverlay ${this.fireClosingAnimation ? 'fadeOutOpacity' : ''}"
        @click=${this.handleClose}
      >
        <div
          @click=${(e: MouseEvent) => {
            e.stopPropagation()
          }}
          class="nc_modalWrapper ${this.fireClosingAnimation ? 'slideOutMobile' : ''}"
        >
          <nightly-header .onClose=${this.handleClose}></nightly-header>
          <div
            class="nc_modalContent"
            style=${styleMap(
              this.isMobile
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
