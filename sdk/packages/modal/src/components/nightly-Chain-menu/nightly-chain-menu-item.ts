import { html } from 'lit'
import style from './nightly-chain-menu-item.css?inline'
import { customElement, property } from 'lit/decorators.js'
import { TailwindElement } from '../../shared/tailwind.element'

@customElement('nightly-chain-menu-item')
export class NightlyChainMenuItem extends TailwindElement(style) {
  @property({ type: String })
  name = ''

  @property({ type: String })
  icon = ''

  @property()
  onClick = () => console.log('')

  render() {
    return html`
      <button class="chainSingleButton" onClick=${this.onClick}>
        <img class="chainIcon" src=${this.icon} />
        <span class="nameButtonChain"> ${this.name} </span>
      </button>
    `
  }
}

declare global {
  interface HTMLElementTagNameMap {
    'nightly-chain-menu-item': NightlyChainMenuItem
  }
}
