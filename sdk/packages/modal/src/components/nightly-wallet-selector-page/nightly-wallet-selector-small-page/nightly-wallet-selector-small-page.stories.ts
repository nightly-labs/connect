import { Meta } from '@storybook/web-components'
import { NightlyWalletSelectorSmallPage } from './nightly-wallet-selector-small-page'
import { html } from 'lit'
import Phantom from '../../../static/svg/PhantomIcon.svg'
import MetaMask from '../../../static/svg/MetaMaskIcon.svg'
import Coinbase from '../../../static/svg/CoinbaseIcon.svg'
import Glow from '../../../static/svg/GlowIcon.svg'
import ZenGO from '../../../static/svg/ZenGOIcon.svg'
import Trust from '../../../static/svg/TrustIcon.svg'
import Binance from '../../../static/svg/BinanceIcon.svg'
import Sollet from '../../../static/svg/SolletIcon.svg'
import NightlyIcon from '../../../static/svg/NightlyIcon.svg'
import './nightly-wallet-selector-small-page'
import '../../nightly-chain-menu-item/nightly-chain-menu-item'
// import '../../nightly-wallet-selector-item/nightly-wallet-selector-item'

interface WalletSelectorItem {
  name: string
  icon: string
  status: string
}

interface NightlyWalletSelectorListArgs {
  walletSelectorItems: WalletSelectorItem[]
  onWalletClick: (event: Event) => void
  // chainIcon: string
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
  console.log(args.walletSelectorItems)

  return html`
    <nightly-wallet-selector-small-page
      .selectorItems=${args.walletSelectorItems}
      .onWalletClick=${args.onWalletClick}
    ></nightly-wallet-selector-small-page>
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
  onWalletClick: (event: Event) => {
    const target = event.target as HTMLElement
    console.log('Item clicked:', target.getAttribute('name'))
  }
}
