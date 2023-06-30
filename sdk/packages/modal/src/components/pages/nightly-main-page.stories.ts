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
        coinName=${args.coinName}
        nameLink=${args.nameLink}
        ?connecting=${args.connecting}
        walletIcon=${args.walletIcon}
        ?connected=${args.connected}
        .tryAgainClick=${args.tryAgainClick}
        .onClose=${args.onClose}
        link=${args.link}
        ?openWalletConncet=${args.openWalletConncet}
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
  coinName: string
  nameLink: string
  connecting: boolean
  walletIcon: string
  connected: boolean
  tryAgainClick: () => void
  fallback: () => void
  link: string
  openWalletConncet: boolean
  useSmallHeader: boolean
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
    onWalletClick: (name: string) => {
      console.log('Item clicked:', name)
    },
    chainIcon: ChainIcon,
    chainName: 'Solana',
    sessionId:
      'fsdhfdzfsdhgfzghggdfhbgchgbdfnvfbxhncvfjhzxdhgbhghfgfvzhfgjhgszdhgzxdfhgfzxdjfuhdfhgd',
    network: 'SOLANA',
    coinName: 'Binance Wallet',
    nameLink: 'Binance Wallet website',
    connecting: true,
    walletIcon: Binance,
    connected: false,
    tryAgainClick: () => console.log('try again click'),
    fallback: () => console.log('back to main page'),
    link: `https://www.binance.com/en`,
    openWalletConncet: false,
    useSmallHeader: false
  }
}
