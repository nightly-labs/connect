import { Meta, StoryObj } from '@storybook/web-components'
import { html } from 'lit'
import { NightlyMainPage } from './nightly-main-page'
import './nightly-main-page'
import Phantom from '../../static/svg/PhantomIcon.svg'
import MetaMask from '../../static/svg/MetaMaskIcon.svg'
import Coinbase from '../../static/svg/CoinbaseIcon.svg'
import Glow from '../../static/svg/GlowIcon.svg'
import ZenGO from '../../static/svg/ZenGOIcon.svg'
import Trust from '../../static/svg/TrustIcon.svg'
import Binance from '../../static/svg/BinanceIcon.svg'
import Sollet from '../../static/svg/SolletIcon.svg'
import NightlyIcon from '../../static/svg/NightlyIcon.svg'
import ChainIcon from '../../static/svg/ChainIcon.svg'

const meta = {
  title: 'nightly-main-page',
  parameters: {
    layout: 'centered'
  },

  render: (args) => {
    return html`
      <nightly-main-page
        .onClose=${args.onClose}
        .selectorItems=${args.selectorItems}
        .onWalletClick=${args.onWalletClick}
        .chainIcon=${args.chainIcon}
        .chainName=${args.chainName}
        .sessionId=${args.sessionId}
        .network=${args.network}
        nameLink=${args.nameLink}
        ?connecting=${args.connecting}
        ?connected=${args.connected}
        .tryAgainClick=${args.tryAgainClick}
        .onClose=${args.onClose}
        link=${args.link}
        .relay=${args.relay}
      ></nightly-main-page>
    `
  }
} satisfies Meta<NightlyMainPage>

export default meta

interface WalletSelectorItem {
  name: string
  icon: string
  status: string
}

interface NightlyModalArgs {
  onClose: () => void
  selectorItems: WalletSelectorItem[]
  onWalletClick: (name: string) => void
  chainIcon: string
  chainName: string
  sessionId: string
  network: string
  nameLink: string
  connecting: boolean
  connected: boolean
  tryAgainClick: () => void
  link: string
  relay: string
}
type Story = StoryObj<NightlyModalArgs>

export const Default: Story = {
  name: 'Default',
  args: {
    onClose: () => console.log('close'),
    selectorItems: [
      { name: 'Phantom', icon: Phantom, status: 'recent' },
      { name: 'Nightly Wallet', icon: NightlyIcon, status: 'recent' },
      { name: 'MetaMask', icon: MetaMask, status: '' },
      { name: 'Glow', icon: Glow, status: '' },
      { name: 'ZenGO', icon: ZenGO, status: 'detected' },
      { name: 'Trust', icon: Trust, status: '' },
      { name: 'Binance', icon: Binance, status: '' },
      { name: 'Sollet', icon: Sollet, status: '' },
      { name: 'Phantom2', icon: Phantom, status: '' },
      { name: 'MetaMask2', icon: MetaMask, status: 'recent' },
      { name: 'Coinbase', icon: Coinbase, status: '' },
      { name: 'ZenGO2', icon: ZenGO, status: '' },
      { name: 'Trust2', icon: Trust, status: 'detected' },
      { name: 'Binance2', icon: Binance, status: '' },
      { name: 'Phantom3', icon: Phantom, status: 'recent' },
      { name: 'Nightly Wallet2', icon: NightlyIcon, status: 'recent' },
      { name: 'MetaMask2', icon: MetaMask, status: '' },
      { name: 'Glow2', icon: Glow, status: '' },
      { name: 'ZenGO3', icon: ZenGO, status: 'detected' },
      { name: 'Trust3', icon: Trust, status: '' },
      { name: 'Binance3', icon: Binance, status: '' },
      { name: 'Sollet2', icon: Sollet, status: '' },
      { name: 'Phantom4', icon: Phantom, status: '' },
      { name: 'MetaMask3', icon: MetaMask, status: 'recent' },
      { name: 'Coinbase2', icon: Coinbase, status: '' },
      { name: 'ZenGO4', icon: ZenGO, status: '' },
      { name: 'Trust4', icon: Trust, status: 'detected' },
      { name: 'Binance4', icon: Binance, status: '' }
    ],
    onWalletClick: (name: string) => {
      console.log('Item clicked:', name)
    },
    chainIcon: ChainIcon,
    chainName: 'Solana',
    sessionId:
      'fsdhfdzfsdhgfzghggdfhbgchgbdfnvfbxhncvfjhzxdhgbhghfgfvzhfgjhgszdhgzxdfhgfzxdjfuhdfhgd',
    network: 'SOLANA',
    nameLink: 'Binance Wallet website',
    connecting: true,
    connected: false,
    tryAgainClick: () => console.log('try again click'),
    link: `https://www.binance.com/en`,
    relay: 'https://relay.nightly.app'
  }
}
