import { Meta, StoryObj } from '@storybook/web-components'
import { html } from 'lit'
import './nightly-connect-wallet'
import { NightlyConnectWallet } from './nightly-connect-wallet'
import binance from '../../static/svg/BinanceIcon.svg'

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
        ?connecting=${args.connecting}
        walletIcon=${args.walletIcon}
        .tryAgainClick=${args.tryAgainClick}
        .goBack=${args.goBack}
        link=${args.link}
      ></nightly-connect-wallet>
    `
  }
} satisfies Meta<NightlyConnectWallet>

export default meta
type Story = StoryObj<NightlyConnectWallet>

export const Connecting: Story = {
  name: 'Connecting',
  args: {
    coinName: 'Binance Wallet',
    nameLink: 'Binance Wallet website',
    connecting: true,
    walletIcon: binance,
    tryAgainClick: () => console.log('try again click'),
    goBack: () => console.log('back to main page'),
    link: `https://www.binance.com/en`
  }
}
export const ConnectingFailed: Story = {
  name: 'ConnectingFailed',
  args: {
    coinName: 'Binance Wallet',
    nameLink: 'Binance Wallet website',
    connecting: false,
    walletIcon: binance,
    tryAgainClick: () => console.log('try again click'),
    goBack: () => console.log('back to main page')
  }
}
