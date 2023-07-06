import { Meta, StoryObj } from '@storybook/web-components'
import { html } from 'lit'
import { NightlySelector, WalletSelectorItem } from './nightly-selector'
import './nightly-selector'
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
  title: 'nightly-selector',
  parameters: {
    layout: 'centered'
  },

  render: (args) => {
    return html`
      <nightly-selector
        .onClose=${args.onClose}
        .selectorItems=${args.selectorItems}
        .onWalletClick=${args.onWalletClick}
        .chainIcon=${args.chainIcon}
        .chainName=${args.chainName}
        .sessionId=${args.sessionId}
        .network=${args.network}
        ?connecting=${args.connecting}
        .relay=${args.relay}
      ></nightly-selector>
    `
  }
} satisfies Meta<NightlySelector>

export default meta

interface NightlyModalArgs {
  onClose: () => void
  selectorItems: WalletSelectorItem[]
  onWalletClick: (name: string) => void
  chainIcon: string
  chainName: string
  sessionId: string
  network: string
  connecting: boolean
  relay: string
}
type Story = StoryObj<NightlyModalArgs & { open: boolean }>

export const Default: Story = {
  name: 'Default',
  args: {
    onClose: () => console.log('close'),
    selectorItems: [
      { name: 'Phantom', icon: Phantom, status: 'recent', link: `https://www.binance.com/en` },
      {
        name: 'Nightly Wallet',
        icon: NightlyIcon,
        status: 'recent',
        link: `https://www.binance.com/en`
      },
      { name: 'MetaMask', icon: MetaMask, status: '', link: `https://www.binance.com/en` },
      { name: 'Glow', icon: Glow, status: '', link: `https://www.binance.com/en` },
      { name: 'ZenGO', icon: ZenGO, status: 'detected', link: `https://www.binance.com/en` },
      { name: 'Trust', icon: Trust, status: '', link: `https://www.binance.com/en` },
      { name: 'Binance', icon: Binance, status: '', link: `https://www.binance.com/en` },
      { name: 'Sollet', icon: Sollet, status: '', link: `https://www.binance.com/en` },
      { name: 'Phantom2', icon: Phantom, status: '', link: `https://www.binance.com/en` },
      { name: 'MetaMask2', icon: MetaMask, status: 'recent', link: `https://www.binance.com/en` },
      { name: 'Coinbase', icon: Coinbase, status: '', link: `https://www.binance.com/en` },
      { name: 'ZenGO2', icon: ZenGO, status: '', link: `https://www.binance.com/en` },
      { name: 'Trust2', icon: Trust, status: 'detected', link: `https://www.binance.com/en` },
      { name: 'Binance2', icon: Binance, status: '', link: `https://www.binance.com/en` },
      { name: 'Phantom3', icon: Phantom, status: 'recent', link: `https://www.binance.com/en` },
      {
        name: 'Nightly Wallet2',
        icon: NightlyIcon,
        status: 'recent',
        link: `https://www.binance.com/en`
      },
      { name: 'MetaMask2', icon: MetaMask, status: '', link: `https://www.binance.com/en` },
      { name: 'Glow2', icon: Glow, status: '', link: `https://www.binance.com/en` },
      { name: 'ZenGO3', icon: ZenGO, status: 'detected', link: `https://www.binance.com/en` },
      { name: 'Trust3', icon: Trust, status: '', link: `https://www.binance.com/en` },
      { name: 'Binance3', icon: Binance, status: '', link: `https://www.binance.com/en` },
      { name: 'Sollet2', icon: Sollet, status: '', link: `https://www.binance.com/en` },
      { name: 'Phantom4', icon: Phantom, status: '', link: `https://www.binance.com/en` },
      { name: 'MetaMask3', icon: MetaMask, status: 'recent', link: `https://www.binance.com/en` },
      { name: 'Coinbase2', icon: Coinbase, status: '', link: `https://www.binance.com/en` },
      { name: 'ZenGO4', icon: ZenGO, status: '', link: `https://www.binance.com/en` },
      { name: 'Trust4', icon: Trust, status: 'detected', link: `https://www.binance.com/en` },
      { name: 'Binance4', icon: Binance, status: '', link: `https://www.binance.com/en` }
    ],
    onWalletClick: (name: string) => {
      console.log('Item clicked:', name)
    },
    chainIcon: ChainIcon,
    chainName: 'Solana',
    sessionId:
      'fsdhfdzfsdhgfzghggdfhbgchgbdfnvfbxhncvfjhzxdhgbhghfgfvzhfgjhgszdhgzxdfhgfzxdjfuhdfhgd',
    network: 'SOLANA',
    connecting: true,
    relay: 'https://relay.nightly.app'
  }
}
