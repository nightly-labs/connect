import { Meta, StoryObj } from '@storybook/web-components'
import { html } from 'lit'
import ChainIcon from '../static/svg/ChainIcon.svg'

import './nightly-chain-menu-item'
import { NightlyChainMenuItem } from './nightly-chain-menu-item'

const meta = {
  title: 'Chain menu item',
  parameters: {
    layout: 'centered'
  },
  render: (args) => {
    return html`<nightly-chain-menu-item
      name=${args.name}
      icon=${args.icon}
      onClick=${args.onClick}
    ></nightly-chain-menu-item>`
  }
} satisfies Meta<NightlyChainMenuItem>

export default meta
type Story = StoryObj<NightlyChainMenuItem>

export const Default: Story = {
  name: 'Default',
  args: {
    name: 'Solana',
    icon: `${ChainIcon}`,
    onClick: () => console.log('Solana')
  }
}
