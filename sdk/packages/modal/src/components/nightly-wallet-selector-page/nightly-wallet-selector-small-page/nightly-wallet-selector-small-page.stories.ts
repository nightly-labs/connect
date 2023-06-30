import { Meta } from '@storybook/web-components'
import { html } from 'lit'
import Binance from '../../../static/svg/BinanceIcon.svg'
import Coinbase from '../../../static/svg/CoinbaseIcon.svg'
import Glow from '../../../static/svg/GlowIcon.svg'
import MetaMask from '../../../static/svg/MetaMaskIcon.svg'
import NightlyIcon from '../../../static/svg/NightlyIcon.svg'
import Phantom from '../../../static/svg/PhantomIcon.svg'
import Sollet from '../../../static/svg/SolletIcon.svg'
import Trust from '../../../static/svg/TrustIcon.svg'
import ZenGO from '../../../static/svg/ZenGOIcon.svg'
import '../../nightly-wallet-selector-item/nightly-wallet-selector-item'
import './nightly-wallet-selector-small-page'
import { NightlyWalletSelectorSmallPage } from './nightly-wallet-selector-small-page'

interface WalletSelectorItem {
  name: string
  icon: string
  status: string
}

interface NightlyWalletSelectorListArgs {
  walletSelectorItems: WalletSelectorItem[]
  onWalletClick: (name: string) => void
  sessionId: string
  network: string
  relay: string
  onClose: () => void
}

const meta: Meta<NightlyWalletSelectorSmallPage> = {
  title: 'Nightly wallet selector small page',
  parameters: {
    layout: 'centered'
  },
  component: 'nightly-wallet-selector-small-page'
}
export default meta

export const Default = (args: NightlyWalletSelectorListArgs) => {
  return html`
    <div style="100%">
      <nightly-wallet-selector-small-page
        .selectorItems=${args.walletSelectorItems}
        .onWalletClick=${args.onWalletClick}
        .sessionId=${args.sessionId}
        .network=${args.network}
        .onClose=${args.onClose}
        .relay=${args.relay}
      ></nightly-wallet-selector-small-page>
    </div>
  `
}
Default.args = {
  walletSelectorItems: [
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
    console.log(name)
  },
  sessionId:
    'fsdhfdzfsdhgfzghggdfhbgchgbdfnvfbxhncvfjhzxdhgbhghfgfvzhfgjhgszdhgzxdfhgfzxdjfuhdfhgd',
  network: 'SOLANA',
  relay: 'https://relay.nightly.app'
}
