import { LitElement, html } from 'lit'
import { customElement, property } from 'lit/decorators.js'
import { tailwindElement } from '../../shared/tailwind.element'
import style from './nightly-selector.css'
import '../pages/nightly-main-page'
import { QueryNetwork, WalletSelectorItem } from '../../utils/types'

@customElement('nightly-selector')
export class NightlySelector extends LitElement {
  static styles = tailwindElement(style)

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
  network: QueryNetwork = QueryNetwork.SOLANA

  @property({ type: String })
  relay = ''

  @property({ type: Boolean })
  connecting = false

  render() {
    return html`
      <div class="nightlySelectorOverlay">
        <nightly-main-page
          class="nightlySelector"
          .onClose=${this.onClose}
          .selectorItems=${this.selectorItems}
          .onWalletClick=${this.onWalletClick}
          .chainIcon=${this.chainIcon}
          .chainName=${this.chainName}
          .sessionId=${this.sessionId}
          .network=${this.network}
          ?connecting=${this.connecting}
          .relay=${this.relay}
        ></nightly-main-page>
      </div>
    `
  }
}

declare global {
  interface HTMLElementTagNameMap {
    'nightly-selector': NightlySelector
  }
}
