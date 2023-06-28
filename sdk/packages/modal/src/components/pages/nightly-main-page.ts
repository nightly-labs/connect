import { LitElement, html } from 'lit'
import { customElement, property } from 'lit/decorators.js'
import { tailwindElement } from '../../shared/tailwind.element'
import { Breakpoint, getBreakpointFromWidthInMainPage } from '../../utils/utils'
import '../nightly-modal/nightly-modal'
import style from './nightly-main-page.css'
import '../nightly-connect-wallet/nightly-connect-wallet'
import '../nightly-wallet-selector-page/nightly-wallet-selector-small-page/nightly-wallet-selector-small-page'
import '../nightly-header-small-page/nightly-header-small-page'
import '../nightly-header/nightly-header'
@customElement('nightly-main-page')
export class NightlyMainPage extends LitElement {
  static styles = tailwindElement(style)

  @property()
  // eslint-disable-next-line @typescript-eslint/no-empty-function
  onClose = () => {}

  @property({ type: Array })
  selectorItems = []

  @property({ type: Boolean })
  openWalletConncet = false

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

  @property({ type: Boolean })
  useSmallHeader = false

  timeoutRef: number | undefined = undefined

  breakpoint: Breakpoint

  openConnectWallet() {
    this.openWalletConncet = true
    this.requestUpdate()
  }

  constructor() {
    super()
    this.useSmallHeader = false
    this.onWalletClick = this.onWalletClick.bind(this)
    this.openConnectWallet = this.openConnectWallet.bind(this)
    this.breakpoint = 'sm'
    this.updateBreakpoint()
    this.resizeListener()
  }

  updateBreakpoint() {
    const screenWidth = window.innerWidth
    this.breakpoint = getBreakpointFromWidthInMainPage(screenWidth)
  }

  resizeListener() {
    window.addEventListener('resize', () => {
      this.updateBreakpoint()
      this.requestUpdate()
    })
  }

  render() {
    let additionalContent
    let headerClass
    let headerComponent

    if (this.openWalletConncet) {
      additionalContent = this.renderConnectWallet()
      headerClass = 'headerWrapConnectWallet'
      headerComponent =
        this.breakpoint === 'xs'
          ? html`<nightly-header-small-page .onClose=${this.onClose}></nightly-header-small-page>`
          : html`<nightly-header .onClose=${this.onClose}></nightly-header>`
    } else if (this.breakpoint === 'xs') {
      additionalContent = html`
        <nightly-wallet-selector-small-page
          .breakpoint=${this.breakpoint}
          .hasUpdated=${this.hasUpdated}
          .isUpdatePending=${this.isUpdatePending}
          .network=${this.network}
          .onWalletClick=${this.openConnectWallet}
          .onClose=${this.onClose}
          .selectorItems=${this.selectorItems}
          .sessionId=${this.sessionId}
        ></nightly-wallet-selector-small-page>
      `
      headerClass = 'headerWrapXS'
      headerComponent = html`<nightly-header-small-page
        .onClose=${this.onClose}
      ></nightly-header-small-page>`
    } else {
      additionalContent = html`
        <div class="nightlyModal">
          <div>
            <nightly-modal
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
            ></nightly-modal>
          </div>
        </div>
      `
      headerClass = 'headerWrapModal'
      headerComponent = html`<nightly-header .onClose=${this.onClose}></nightly-header>`
    }

    return html`
      <div style="display: flex; flex-direction: column; height: 100vh; z-index: 1;">
        <div class="${headerClass}">${headerComponent}</div>
        <div>${additionalContent}</div>
      </div>
    `
  }

  backToPage = () => {
    this.openWalletConncet = false
    this.requestUpdate()
  }

  renderConnectWallet() {
    return html`
      <div class="connectWallet">
        <nightly-connect-wallet
          .breakpoint=${this.breakpoint}
          .coinName=${this.coinName}
          .connecting=${this.connecting}
          .onClose=${this.onClose}
          .tryAgainClick=${this.tryAgainClick}
          .fallback=${this.backToPage}
          .link=${this.link}
          .nameLink=${this.nameLink}
          .walletIcon=${this.walletIcon}
        ></nightly-connect-wallet>
      </div>
    `
  }
}

declare global {
  interface HTMLElementTagNameMap {
    'nightly-main-page': NightlyMainPage
  }
}
