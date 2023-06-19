import { Meta, StoryObj } from '@storybook/web-components'
import { html } from 'lit'
import './nightly-connect-wallet'
import { NightlyConnectWallet } from './nightly-connect-wallet'

const meta = {
  title: 'nightly-connect-wallet',
  parameters: {
    layout: 'centered'
  },

  render: (args) => {
    return html`
      <nightly-connect-wallet
        coinName=${args.coinName}
        nameLink=${args.nameLink}
      ></nightly-connect-wallet>
    `
  }
} satisfies Meta<NightlyConnectWallet>

export default meta
type Story = StoryObj<NightlyConnectWallet>

export const Default: Story = {
  name: 'Default',
  args: {
    coinName: 'Binance Wallet',
    nameLink: 'Binance Wallet website'
  }
}
