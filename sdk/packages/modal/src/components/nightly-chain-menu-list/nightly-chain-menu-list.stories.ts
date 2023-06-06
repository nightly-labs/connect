import { Meta } from '@storybook/web-components'
import { html } from 'lit'
import ChainIcon from '../../static/svg/ChainIcon.svg'

import '../nightly-chain-menu-item/nightly-chain-menu-item'
import './nightly-chain-menu-list'
import { NightlyChainMenuList } from './nightly-chain-menu-list'

interface MenuItem {
  name: string
  icon: string
}

interface NightlyChainMenuListArgs {
  menuItems: MenuItem[]
  onItemClick: (event: Event) => void
}

const meta: Meta<NightlyChainMenuList> = {
  title: 'Chain menu list',
  parameters: {
    layout: 'centered'
  },
  component: 'nightly-chain-menu-list'
}

export default meta

export const Default = (args: NightlyChainMenuListArgs) => html`
  <nightly-chain-menu-list
    .menuItems=${args.menuItems}
    .onItemClick=${args.onItemClick}
  ></nightly-chain-menu-list>
`

Default.args = {
  menuItems: [
    { name: 'Solana', icon: ChainIcon },
    { name: 'Ethereum', icon: ChainIcon },
    { name: 'Bitcoin', icon: ChainIcon }
  ],
  onItemClick: (event: Event) => {
    const target = event.target as HTMLElement
    console.log('Item clicked:', target.getAttribute('name'))
  }
}
