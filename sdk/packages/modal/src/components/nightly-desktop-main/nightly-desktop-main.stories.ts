import { Meta, StoryObj } from '@storybook/web-components'
import { html } from 'lit/static-html.js'
import './nightly-desktop-main'
import { NightlyDesktopMain } from './nightly-desktop-main'
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
  title: 'nightly-desktop-main',
  parameters: {
    layout: 'centered'
  },
  render: (args) => {
    return html`<nightly-desktop-main
      .selectorItems=${args.selectorItems}
      .onWalletClick=${args.onWalletClick}
      .chainIcon=${args.chainIcon}
      .chainName=${args.chainName}
      .sessionId=${args.sessionId}
      .relay=${args.relay}
    ></nightly-desktop-main>`
  }
} satisfies Meta<NightlyDesktopMain>

export default meta

interface WalletSelectorItem {
  name: string
  icon: string
  status: string
}

interface NightlyModalArgs {
  selectorItems: WalletSelectorItem[]
  onWalletClick: (name: string) => void
  chainIcon: string
  chainName: string
  sessionId: string
  relay: string
}
type Story = StoryObj<NightlyModalArgs>

export const Default: Story = {
  name: 'Default',
  args: {
    selectorItems: [
      { name: 'Phantom', icon: Phantom, status: 'recent' },
      { name: 'Nightly Wallet', icon: NightlyIcon, status: 'recent' },
      { name: 'MetaMask', icon: MetaMask, status: '' },
      { name: 'Glow', icon: Glow, status: '' },
      { name: 'ZenGO', icon: ZenGO, status: 'detected' },
      { name: 'Trust', icon: Trust, status: '' },
      { name: 'Binance', icon: Binance, status: '' },
      { name: 'Sollet', icon: Sollet, status: '' },
      { name: 'Phantom', icon: Phantom, status: '' },
      { name: 'MetaMask', icon: MetaMask, status: 'recent' },
      { name: 'Coinbase', icon: Coinbase, status: '' },
      { name: 'ZenGO', icon: ZenGO, status: '' },
      { name: 'Trust', icon: Trust, status: 'detected' },
      { name: 'Binance', icon: Binance, status: '' },
      { name: 'Phantom', icon: Phantom, status: 'recent' },
      { name: 'Nightly Wallet', icon: NightlyIcon, status: 'recent' },
      { name: 'MetaMask', icon: MetaMask, status: '' },
      { name: 'Glow', icon: Glow, status: '' },
      { name: 'ZenGO', icon: ZenGO, status: 'detected' },
      { name: 'Trust', icon: Trust, status: '' },
      { name: 'Binance', icon: Binance, status: '' },
      { name: 'Sollet', icon: Sollet, status: '' },
      { name: 'Phantom', icon: Phantom, status: '' },
      { name: 'MetaMask', icon: MetaMask, status: 'recent' },
      { name: 'Coinbase', icon: Coinbase, status: '' },
      { name: 'ZenGO', icon: ZenGO, status: '' },
      { name: 'Trust', icon: Trust, status: 'detected' },
      { name: 'Binance', icon: Binance, status: '' }
    ],
    onWalletClick: (name: string) => {
      console.log('Item clicked:', name)
    },
    chainIcon: ChainIcon,
    chainName: 'Solana',
    sessionId: '6a82dc5a-c013-4c17-b6ff-45fe0f45bddb',
    relay: 'https://nc2.nightly.app'
  }
}
