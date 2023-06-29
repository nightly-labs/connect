import { customElement, property } from 'lit/decorators.js'
import { html } from 'lit/static-html.js'
import { tailwindElement } from '../../../shared/tailwind.element'
import '../nightly-all-wallets-selector/nightly-all-wallets-selector'
import '../nightly-qrCode/nightly-qrCode'
import '../nightly-wallet-wrapper/nightly-wallet-wrapper'
import style from './nightly-wallet-selector-small-page.css'
import { LitElement } from 'lit'
@customElement('nightly-wallet-selector-small-page')
export class NightlyWalletSelectorSmallPage extends LitElement {
  static styles = tailwindElement(style)

  @property({})
  // eslint-disable-next-line @typescript-eslint/no-empty-function
  onClose = () => {}

  @property({ type: Boolean })
  showAll = false

  @property({ type: Boolean })
  isTopWalletsView = true

  @property({ type: Boolean })
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
      <div style="">
        ${(() => {
          if (!this.isTopWalletsView && this.showAll) {
            return html`
              <nightly-all-wallets-selector
                .showAllWallets=${this.showAllWallets.bind(this)}
                .onWalletClick=${this.onWalletClick.bind(this)}
                .selectorItems=${this.selectorItems}
              ></nightly-all-wallets-selector>
            `
          }

          if (!this.isTopWalletsView && this.isQrPageVisible) {
            return html`
              <nightly-qr-code
                .network=${this.network}
                .sessionId=${this.sessionId}
                .showAllWallets=${this.showAllWallets.bind(this)}
              ></nightly-qr-code>
            `
          }

          return html`
            <nightly-wallet-wrapper
              .network=${this.network}
              .sessionId=${this.sessionId}
              .showAllWallets=${this.showAllWallets.bind(this)}
              .onWalletClick=${this.onWalletClick.bind(this)}
              .openQrPage=${() => this.openQrPage()}
              .selectorItems=${this.selectorItems}
            ></nightly-wallet-wrapper>
          `
        })()}
      </div>
    `
  }
}

declare global {
  interface HTMLElementTagNameMap {
    'nightly-wallet-selector-small-page': NightlyWalletSelectorSmallPage
  }
}
