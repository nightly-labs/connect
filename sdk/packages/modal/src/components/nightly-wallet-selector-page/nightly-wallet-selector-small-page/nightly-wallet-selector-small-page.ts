import { customElement, property, state } from 'lit/decorators.js'
import { html } from 'lit/static-html.js'
import { tailwindElement } from '../../../shared/tailwind.element'
import '../nightly-all-wallets-selector/nightly-all-wallets-selector'
import '../nightly-qrCode/nightly-qrCode'
import '../nightly-wallet-wrapper/nightly-wallet-wrapper'
import style from './nightly-wallet-selector-small-page.css'
import { LitElement } from 'lit'
import { animate } from '@lit-labs/motion'

@customElement('nightly-wallet-selector-small-page')
export class NightlyWalletSelectorSmallPage extends LitElement {
  static styles = tailwindElement(style)

  @property({})
  // eslint-disable-next-line @typescript-eslint/no-empty-function
  onClose = () => {}

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
  network = ''

  @property({ type: Function })
  // eslint-disable-next-line @typescript-eslint/no-empty-function
  onWalletClick: (name: string) => void = () => {}

  @property({ type: Array })
  selectorItems: { name: string; icon: string; status: string }[] = []

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
    this.requestUpdate()
  }

  openQrPage() {
    this.isQrPageVisible = true
    this.isTopWalletsView = false
    this.requestUpdate()
  }

  render() {
    return html`
      <nightly-wallet-wrapper
        class="selectorView ${this.isTopWalletsView ? 'fade-in-open' : 'fade-in-closed'}"
        .network=${this.network}
        .sessionId=${this.sessionId}
        .showAllWallets=${this.showAllWallets.bind(this)}
        .onWalletClick=${this.onWalletClick.bind(this)}
        .openQrPage=${() => this.openQrPage()}
        .selectorItems=${this.selectorItems}
        ${animate()}
      ></nightly-wallet-wrapper>
      <nightly-all-wallets-selector
        class="selectorView ${!this.isTopWalletsView && this.showAll
          ? 'fade-in-open'
          : 'fade-in-closed'}"
        .showAllWallets=${this.showAllWallets.bind(this)}
        .onWalletClick=${this.onWalletClick.bind(this)}
        .selectorItems=${this.selectorItems}
        ${animate()}
      ></nightly-all-wallets-selector>
      <nightly-qr-code
        class="selectorView ${!this.isTopWalletsView && this.isQrPageVisible
          ? 'fade-in-open'
          : 'fade-in-closed'}"
        .network=${this.network}
        .sessionId=${this.sessionId}
        .showAllWallets=${this.showAllWallets.bind(this)}
        ${animate()}
      ></nightly-qr-code>
    `
  }
}

declare global {
  interface HTMLElementTagNameMap {
    'nightly-wallet-selector-small-page': NightlyWalletSelectorSmallPage
  }
}
