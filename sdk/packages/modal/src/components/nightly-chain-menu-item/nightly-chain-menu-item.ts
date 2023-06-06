import { LitElement, css, html } from 'lit'
import { customElement, property } from 'lit/decorators.js'

@customElement('nightly-chain-menu-item')
export class NightlyChainMenuItem extends LitElement {
  static styles = css`
    .chainSingleButton {
      display: flex;
      border: none;
      margin-bottom: 4px;
      align-items: center;
      width: 241px;
      height: 30px;
      background-color: #343551;
      border-radius: 4px;
      padding: 4px 8px;
    }

    .chainSingleButton:hover {
      background-color: #47486a;
    }
    .chainSingleButton:focus {
      background-color: #6067f9;
    }

    .chainIcon {
      width: 16px;
      height: 16px;
    }

    .nameButtonChain {
      font-family: 'Prompt';
      margin-left: 4px;
      font-size: 12px;
      line-height: 22px;
      font-weight: 400;
      color: #f7f7f7;
    }
  `
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
