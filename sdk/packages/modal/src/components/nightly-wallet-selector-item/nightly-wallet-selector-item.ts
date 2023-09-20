import { customElement, property } from 'lit/decorators.js'
import { tailwindElement } from '../../shared/tailwind.element'
import { html } from 'lit/static-html.js'
import style from './nightly-wallet-selector-item.css'
import { LitElement } from 'lit'

@customElement('nightly-wallet-selector-item')
export class NightlyWalletSelectorItem extends LitElement {
  static styles = tailwindElement(style)

  @property({ type: String })
  name = ''

  @property({ type: String })
  icon = ''

  @property({ type: String })
  status = ''

  @property({ type: Function })
  // eslint-disable-next-line @typescript-eslint/no-empty-function
  onClick: (event: Event) => void = () => {}

  render() {
    return html`
      <button
        class="nc_walletButton"
        @click=${this.onClick}
      >
        <img class="nc_walletButtonIcon" src=${this.icon} />
        <span class="nc_walletButtonName">${this.name}</span>
        <span class="nc_walletButtonStatus">${this.status}</span>
      </button>
    `
  }
}

declare global {
  interface HTMLElementTagNameMap {
    'nightly-wallet-selector-item': NightlyWalletSelectorItem
  }
}
