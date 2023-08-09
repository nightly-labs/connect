import { LitElement, html } from 'lit'
import { customElement, property, state } from 'lit/decorators.js'
import { tailwindElement } from '../../shared/tailwind.element'
import style from './nightly-selector.css'
import '../pages/nightly-main-page'
import { WalletSelectorItem } from '../../utils/types'

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
  relay = ''

  @property({ type: Boolean })
  connecting = false

  @state()
  fireClosingAnimation = false

  handleClose = () => {
    this.fireClosingAnimation = true
    setTimeout(
      () => {
        this.onClose()
      },
      window.matchMedia('(max-width: 640px)') ? 240 : 80
    )
  }

  constructor() {
    super()
    this.handleClose = this.handleClose.bind(this)
  }

  disconnectedCallback(): void {
    super.disconnectedCallback()
    this.fireClosingAnimation = false
  }

  render() {
    return html`
      <div
        class="nightlySelectorOverlay ${this.fireClosingAnimation ? 'fadeOutOpacity' : ''}"
        @click=${this.handleClose}
      >
        <nightly-main-page
          @click=${(e: MouseEvent) => {
            e.stopPropagation()
          }}
          class="nightlySelector"
          .onClose=${this.handleClose}
          .selectorItems=${this.selectorItems}
          .onWalletClick=${this.onWalletClick}
          .chainIcon=${this.chainIcon}
          .chainName=${this.chainName}
          .sessionId=${this.sessionId}
          ?connecting=${this.connecting}
          .relay=${this.relay}
          ?fireClosingAnimation=${this.fireClosingAnimation}
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
