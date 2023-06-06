import { customElement, property } from 'lit/decorators.js'
import { TailwindElement } from '../../shared/tailwind.element'
import { html } from 'lit/static-html.js'
import style from './nightly-wallet-selector-item.css?inline'

@customElement('nightly-wallet-selector-item')
export class NightlyWalletSelectorItem extends TailwindElement(style) {
  @property({ type: String })
  name = ''

  @property({ type: String })
  icon = ''

  @property({ type: String })
  recent = ''

  @property({ type: String })
  detected = ''

  @property()
  // eslint-disable-next-line @typescript-eslint/no-empty-function
  onClick = () => {}

  render() {
    return html`
      <button class="walletSelectorItem" onClick=${this.onClick}>
        <img src=${this.icon} />
        <span class="walletSelectorName"> ${this.name} </span>
        ${this.recent ? html`<span class="walletSelectorInfo">${this.recent}</span>` : ''}
        ${this.detected ? html`<span class="walletSelectorInfo">${this.detected}</span>` : ''}
      </button>
    `
  }
}

declare global {
  interface HTMLElementTagNameMap {
    'nightly-wallet-selector-item': NightlyWalletSelectorItem
  }
}
