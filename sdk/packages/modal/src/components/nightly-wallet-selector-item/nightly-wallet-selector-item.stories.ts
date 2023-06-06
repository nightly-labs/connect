import { Meta, StoryObj } from '@storybook/web-components'
import { html } from 'lit'
import { NightlyWalletSelectorItem } from './nightly-wallet-selector-item'
import PhantomIcon from '../../static/svg/PhantomIcon.svg'
import MetaMaskIcon from '../../static/svg/MetaMaskIcon.svg'
import BinanceIcon from '../../static/svg/BinanceIcon.svg'

import './nightly-wallet-selector-item'

const meta = {
  title: 'nightly-wallet-selector-item',
  parameters: {
    layout: 'centered'
  },
  render: (args) => {
    return html`
      <div class="width: 100%; height: 100%;  background-color: #17182B;">
        <nightly-wallet-selector-item
          name=${args.name}
          icon=${args.icon}
          status=${args.status}
        ></nightly-wallet-selector-item>
      </div>
    `
  }
} satisfies Meta<NightlyWalletSelectorItem>

export default meta
type Story = StoryObj<NightlyWalletSelectorItem>

export const Default: Story = {
  name: 'Phantom',
  args: {
    name: 'Phantom',
    icon: PhantomIcon,
    status: 'detected'
  }
}

export const MetaMask: Story = {
  name: 'MetaMask',
  args: {
    name: 'MetaMask',
    icon: MetaMaskIcon,
    status: 'recent'
  }
}

export const Binance: Story = {
  name: 'Binance',
  args: {
    name: 'Binance Wallet',
    icon: BinanceIcon,
    status: 'recent'
  }
}
