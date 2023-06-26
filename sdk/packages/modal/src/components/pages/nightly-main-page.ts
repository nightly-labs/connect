import { html } from 'lit'
import { customElement, property } from 'lit/decorators.js'
import { TailwindElement } from '../../shared/tailwind.element'
import { Breakpoint, getBreakpointFromWidthInMainPage } from '../../utils/utils'
import '../nightly-modal/nightly-modal'
import style from './nightly-main-page.css?inline'
import '../nightly-connect-wallet/nightly-connect-wallet'
import '../nightly-wallet-selector-page/nightly-wallet-selector-small-page/nightly-wallet-selector-small-page'
import '../nightly-header-small-page/nightly-header-small-page'
@customElement('nightly-main-page')
export class NightlyMainPage extends TailwindElement(style) {
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

  timeoutRef: number | undefined = undefined

  breakpoint: Breakpoint

  constructor() {
    super()
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

    if (this.openWalletConncet) {
      additionalContent = this.renderConnectWallet()
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
    } else {
      additionalContent = html`
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
      `
    }

    return html`
      <div style="display: flex; flex-direction: column; height: 100vh; z-index: 1;">
        <nightly-header-small-page .onClose=${this.onClose}></nightly-header-small-page>
        <div>${additionalContent}</div>
      </div>
    `
  }

  openConnectWallet() {
    this.openWalletConncet = true
    this.requestUpdate()
  }

  backToPage = () => {
    this.openWalletConncet = false
    this.requestUpdate()
  }

  renderConnectWallet() {
    return html`
      <div class="">
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
