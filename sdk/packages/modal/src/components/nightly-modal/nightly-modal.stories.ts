import { Meta, StoryObj } from '@storybook/web-components'
import { html } from 'lit/static-html.js'
import './nightly-modal'
import { NightlyModal } from './nightly-modal'
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
  title: 'nightly-modal',
  parameters: {
    layout: 'centered'
  },
  render: (args) => {
    return html`<nightly-modal 
    .onClose=${args.onClose} 
    .selectorItems=${args.selectorItems}
    .onWalletClick=${args.onWalletClick}
    .chainIcon=${args.chainIcon}
    .chainName=${args.chainName}
    .sessionId=${args.sessionId}
    .network=${args.network}
    ></nightly-modal>`
  }
} satisfies Meta<NightlyModal>

export default meta

interface WalletSelectorItem {
  name: string
  icon: string
  status: string
}

interface NightlyModalArgs {
  onClose: () => void
  selectorItems: WalletSelectorItem[]
  onWalletClick: () => void
  chainIcon: string
  chainName: string,
  sessionId: string,
  network: string
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
    onWalletClick: () => {
      console.log('Item clicked')
    },
    chainIcon: ChainIcon,
    chainName: 'Solana',
    sessionId: 'fsdhfdzfsdhgfzghggdfhbgchgbdfnvfbxhncvfjhzxdhgbhghfgfvzhfgjhgszdhgzxdfhgfzxdjfuhdfhgd',
    network: 'SOLANA'
  }
}