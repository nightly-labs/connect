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
import ChainIcon from '../../static/svg/ChainIcon.svg'

import './nightly-wallet-selector-page'
import '../nightly-chain-menu-item/nightly-chain-menu-item'

interface WalletSelectorItem {
  name: string
  icon: string
  status: string
}

interface NightlyWalletSelectorListArgs {
  walletSelectorItems: WalletSelectorItem[]
  onWalletClick: (name: string) => void
  chainIcon: string
  chainName: string
}

const meta: Meta<NightlyWalletSelectorPage> = {
  title: 'Nightly wallet selector page',
  parameters: {
    layout: 'centered'
  },
  component: 'nightly-wallet-selector-page'
}

export default meta

export const Default = (args: NightlyWalletSelectorListArgs) => {
  console.log(args.walletSelectorItems)

  return html`
    <nightly-wallet-selector-page
      .selectorItems=${args.walletSelectorItems}
      .onWalletClick=${args.onWalletClick}
      .chainIcon=${args.chainIcon}
      .chainName=${args.chainName}
    ></nightly-wallet-selector-page>
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
  chainIcon: ChainIcon,
  chainName: 'Solana'
}
