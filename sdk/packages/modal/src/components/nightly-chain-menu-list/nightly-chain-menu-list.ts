import { css, html } from 'lit'
import { customElement } from 'lit/decorators.js'
import { NightlyChainMenuItem } from '../nightly-chain-menu-item/nightly-chain-menu-item'
import ChainIcon from '../static/svg/ChainIcon.svg'

@customElement('nightly-chain-menu-list')
export class NightlyChainMenuList extends NightlyChainMenuItem {
  static styles = css`
    .mainContainer {
      padding: 8px 8px 4px 8px;
      border-radius: 8px;
      background-color: #202137;
      border: 1px solid #343551;
    }
  `
  render() {
    return html`<div class="mainContainer">${this.renderMenuItems()}</div>`
  }

  renderMenuItems() {
    const menuItems = [
      { name: 'Solana', icon: `${ChainIcon}` },
      { name: 'NEAR', icon: `${ChainIcon}` },
      { name: 'Bitcoin', icon: `${ChainIcon}` },
      { name: 'Ethereum', icon: `${ChainIcon}` },
      { name: 'SUI', icon: `${ChainIcon}` }
    ]

    return menuItems.map((item) => {
      return html`
        <nightly-chain-menu-item
          name=${item.name}
          icon=${item.icon}
          @click=${this.handleMenuItemClick}
        ></nightly-chain-menu-item>
      `
    })
  }

  handleMenuItemClick(event: { target: { name: string } }) {
    console.log('Kliknięto element menu:', event.target.name)
  }
}
