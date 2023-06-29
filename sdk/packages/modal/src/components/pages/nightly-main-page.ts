import { LitElement, html } from 'lit'
import { customElement, property } from 'lit/decorators.js'
import { tailwindElement } from '../../shared/tailwind.element'
import { Breakpoint, getBreakpointFromWidthInMainPage } from '../../utils/utils'
import '../nightly-modal/nightly-modal'
import style from './nightly-main-page.css'
import '../nightly-connect-wallet/nightly-connect-wallet'
import '../nightly-wallet-selector-page/nightly-wallet-selector-small-page/nightly-wallet-selector-small-page'
import '../nightly-header/nightly-header'
import { styleMap } from 'lit/directives/style-map.js'
import { animate } from '@lit-labs/motion'
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
    return html`
      <div class="nightlyModal">
        <nightly-header .onClose=${this.onClose}></nightly-header>
        <div class="contentWrapper">
          <nightly-connect-wallet
            class="fade-in"
            style=${styleMap({
              visibility: this.openWalletConncet ? 'visible' : 'hidden',
              height: this.openWalletConncet ? 'auto' : '0',
              opacity: this.openWalletConncet ? '1' : '0'
            })}
            .breakpoint=${this.breakpoint}
            .coinName=${this.coinName}
            .connecting=${this.connecting}
            .tryAgainClick=${this.tryAgainClick}
            .fallback=${this.backToPage}
            .link=${this.link}
            .nameLink=${this.nameLink}
            .walletIcon=${this.walletIcon}
            ${animate()}
          ></nightly-connect-wallet>
          <nightly-wallet-selector-small-page
            class="fade-in"
            style=${styleMap({
              visibility:
                this.breakpoint === 'xs' && !this.openWalletConncet ? 'visible' : 'hidden',
              height: this.breakpoint === 'xs' && !this.openWalletConncet ? 'auto' : '0',
              opacity: this.breakpoint === 'xs' && !this.openWalletConncet ? '1' : '0'
            })}
            .breakpoint=${this.breakpoint}
            .hasUpdated=${this.hasUpdated}
            .isUpdatePending=${this.isUpdatePending}
            .network=${this.network}
            .onWalletClick=${this.openConnectWallet}
            .onClose=${this.onClose}
            .selectorItems=${this.selectorItems}
            .sessionId=${this.sessionId}
            ${animate()}
          ></nightly-wallet-selector-small-page>
          <nightly-modal
            class="fade-in"
            style=${styleMap({
              visibility:
                this.breakpoint !== 'xs' && !this.openWalletConncet ? 'visible' : 'hidden',
              height: this.breakpoint !== 'xs' && !this.openWalletConncet ? 'auto' : '0',
              opacity: this.breakpoint !== 'xs' && !this.openWalletConncet ? '1' : '0'
            })}
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
            ${animate()}
          ></nightly-modal>
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
