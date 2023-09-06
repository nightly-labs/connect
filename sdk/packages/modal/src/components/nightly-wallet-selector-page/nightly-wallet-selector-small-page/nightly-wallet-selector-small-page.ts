import { customElement, property, state } from 'lit/decorators.js'
import { html } from 'lit/static-html.js'
import { tailwindElement } from '../../../shared/tailwind.element'
import '../nightly-all-wallets-selector/nightly-all-wallets-selector'
import '../nightly-qrCode/nightly-qrCode'
import '../nightly-wallet-wrapper/nightly-wallet-wrapper'
import style from './nightly-wallet-selector-small-page.css'
import { LitElement } from 'lit'
import { WalletSelectorItem } from '../../../utils/types'

@customElement('nightly-wallet-selector-small-page')
export class NightlyWalletSelectorSmallPage extends LitElement {
  static styles = tailwindElement(style)

  @state()
  showAll = false

  @state()
  isTopWalletsView = true

  @state()
  isQrPageVisible = false

  @property({ type: String })
  chainIcon = ''

  @property({ type: String })
  sessionId = ''

  @property({ type: String })
  chainName = ''

  @property({ type: String })
  relay = ''

  @property({ type: Function })
  // eslint-disable-next-line @typescript-eslint/no-empty-function
  onWalletClick: (name: string) => void = () => {}

  @property({ type: Array })
  selectorItems: WalletSelectorItem[] = []

  showAllWallets() {
    if (this.isQrPageVisible) {
      this.isQrPageVisible = false
      this.isTopWalletsView = true
      this.showAll = false
    } else {
      const oppositeState = this.showAll
      this.isTopWalletsView = oppositeState
      this.showAll = !oppositeState
    }
  }

  openQrPage() {
    this.isQrPageVisible = true
    this.isTopWalletsView = false
  }

  disconnectedCallback(): void {
    super.disconnectedCallback()
    this.isTopWalletsView = true
    this.isQrPageVisible = false
    this.showAll = false
  }

  renderView() {
    if (this.isTopWalletsView) {
      return html`
        <nightly-wallet-wrapper
          class="selectorView"
          .sessionId=${this.sessionId}
          .showAllWallets=${this.showAllWallets.bind(this)}
          .onWalletClick=${this.onWalletClick.bind(this)}
          .openQrPage=${() => this.openQrPage()}
          .selectorItems=${this.selectorItems}
        ></nightly-wallet-wrapper>
      `
    }

    if (this.showAll) {
      return html` <nightly-all-wallets-selector
        class="selectorView"
        .showAllWallets=${this.showAllWallets.bind(this)}
        .onWalletClick=${this.onWalletClick.bind(this)}
        .selectorItems=${this.selectorItems}
      ></nightly-all-wallets-selector>`
    }

    return html`
      <nightly-qr-code
        class="selectorView"
        .chainName=${this.chainName}
        .sessionId=${this.sessionId}
        .relay=${this.relay}
        .showAllWallets=${this.showAllWallets.bind(this)}
      ></nightly-qr-code>
    `
  }

  render() {
    return html` <div
      class="selectorWrapper ${this.isTopWalletsView
        ? 'topView'
        : this.isQrPageVisible
        ? 'qrView'
        : 'walletsView'}"
    >
      ${this.renderView()}
    </div>`
  }
}

declare global {
  interface HTMLElementTagNameMap {
    'nightly-wallet-selector-small-page': NightlyWalletSelectorSmallPage
  }
}
