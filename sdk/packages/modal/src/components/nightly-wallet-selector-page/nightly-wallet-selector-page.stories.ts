import { Meta } from '@storybook/web-components'
import { NightlyWalletSelectorPage } from './nightly-wallet-selector-page'
import { html } from 'lit'
import Phantom from '../../static/svg/PhantomIcon.svg'
import MetaMask from '../../static/svg/MetaMaskIcon.svg'
import Coinbase from '../../static/svg/CoinbaseIcon.svg'
import Glow from '../../static/svg/GlowIcon.svg'
import ZenGO from '../../static/svg/ZenGOIcon.svg'
import Trust from '../../static/svg/TrustIcon.svg'
import Binance from '../../static/svg/BinanceIcon.svg'
import Sollet from '../../static/svg/SolletIcon.svg'
import NightlyIcon from '../../static/svg/NightlyIcon.svg'
import './nightly-wallet-selector-page'
import '../nightly-chain-menu-item/nightly-chain-menu-item'

interface WalletSelectorItem {
  name: string
  icon: string
  status: string
}

interface NightlyWalletSelectorListArgs {
  walletSelectorItems: WalletSelectorItem[]
  onWalletClick: (event: Event) => void
}

const meta: Meta<NightlyWalletSelectorPage> = {
  title: 'Nightly wallet selector page',
  parameters: {
    layout: 'centered'
  },
  component: 'nightly-wallet-selector-page'
}

export default meta

export const Default = (args: NightlyWalletSelectorListArgs) => html`
  <nightly-wallet-selector-page
    .walletSelectorItems=${args.walletSelectorItems}
    .onWalletClick=${args.onWalletClick}
  ></nightly-wallet-selector-page>
`
Default.args = {
  walletSelectorItems: [
    { name: 'Phantom', icon: Phantom, status: '' },
    { name: 'MetaMask', icon: MetaMask, status: '' },
    { name: 'Coinbase', icon: Coinbase, status: '' },
    { name: 'Nightly Wallet', icon: NightlyIcon, status: '' },
    { name: 'Glow Wallet', icon: Glow, status: '' },
    { name: 'ZenGO', icon: ZenGO, status: '' },
    { name: 'Trust', icon: Trust, status: '' },
    { name: 'Binance Wallet', icon: Binance, status: '' },
    { name: 'Sollet', icon: Sollet, status: '' }
  ],
  onWalletClick: (event: Event) => {
    const target = event.target as HTMLElement
    console.log('Item clicked:', target.getAttribute('name'))
  }
}
