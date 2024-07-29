import { useArgs } from '@storybook/client-api'
import { Meta, StoryObj } from '@storybook/web-components'
import { html } from 'lit'
import Binance from '../../static/svg/BinanceIcon.svg'
import ChainIcon from '../../static/svg/ChainIcon.svg'
import Coinbase from '../../static/svg/CoinbaseIcon.svg'
import Glow from '../../static/svg/GlowIcon.svg'
import MetaMask from '../../static/svg/MetaMaskIcon.svg'
import NightlyIcon from '../../static/svg/NightlyIcon.svg'
import Phantom from '../../static/svg/PhantomIcon.svg'
import Sollet from '../../static/svg/SolletIcon.svg'
import Trust from '../../static/svg/TrustIcon.svg'
import ZenGO from '../../static/svg/ZenGOIcon.svg'
import { WalletSelectorItem } from '../../utils/types'
import './nightly-selector'
import { NightlySelector } from './nightly-selector'

const meta = {
  title: 'nightly-selector',
  parameters: {
    layout: 'centered'
  },
  argTypes: {
    open: {
      control: 'boolean'
    }
  }
} satisfies Meta<NightlySelector & { open: boolean }>

export default meta

interface NightlyModalArgs {
  onClose: () => void
  selectorItems: WalletSelectorItem[]
  onWalletClick: (name: string) => void
  chainIcon: string
  chainName: string
  sessionId?: string
  connecting: boolean
  relay: string
}
type Story = StoryObj<NightlyModalArgs & { open: boolean }>

export const Default: Story = (args: NightlyModalArgs) => {
  const [{ open }, updateArgs] = useArgs()

  const handleClose = () => {
    updateArgs({ open: false })
    args.onClose()
  }

  return open
    ? html`
        <nightly-selector
          .onClose=${handleClose}
          .selectorItems=${args.selectorItems}
          .onWalletClick=${args.onWalletClick}
          .chainIcon=${args.chainIcon}
          .chainName=${args.chainName}
          .sessionId=${args.sessionId}
          ?connecting=${args.connecting}
          .relay=${args.relay}
        ></nightly-selector>
      `
    : html``
}

Default.args = {
  onClose: () => console.log('close'),
  selectorItems: [
    { name: 'Phantom', icon: Phantom, recent: true, link: `https://www.binance.com/en` },
    {
      name: 'Nightly Wallet',
      icon: NightlyIcon,
      link: `https://www.binance.com/en`
    },
    { name: 'MetaMask', icon: MetaMask, link: `https://www.binance.com/en` },
    { name: 'Glow', icon: Glow, link: `https://www.binance.com/en` },
    { name: 'ZenGO', icon: ZenGO, detected: true, link: `https://www.binance.com/en` },
    { name: 'Trust', icon: Trust, link: `https://www.binance.com/en` },
    { name: 'Binance', icon: Binance, link: `https://www.binance.com/en` },
    { name: 'Sollet', icon: Sollet, link: `https://www.binance.com/en` },
    { name: 'Phantom2', icon: Phantom, link: `https://www.binance.com/en` },
    { name: 'MetaMask2', icon: MetaMask, link: `https://www.binance.com/en` },
    { name: 'Coinbase', icon: Coinbase, link: `https://www.binance.com/en` },
    { name: 'ZenGO2', icon: ZenGO, link: `https://www.binance.com/en` },
    { name: 'Trust2', icon: Trust, detected: true, link: `https://www.binance.com/en` },
    { name: 'Binance2', icon: Binance, link: `https://www.binance.com/en` },
    { name: 'Phantom3', icon: Phantom, link: `https://www.binance.com/en` },
    {
      name: 'Nightly Wallet2',
      icon: NightlyIcon,
      link: `https://www.binance.com/en`
    },
    { name: 'MetaMask2', icon: MetaMask, link: `https://www.binance.com/en` },
    { name: 'Glow2', icon: Glow, link: `https://www.binance.com/en` },
    { name: 'ZenGO3', icon: ZenGO, detected: true, link: `https://www.binance.com/en` },
    { name: 'Trust3', icon: Trust, link: `https://www.binance.com/en` },
    { name: 'Binance3', icon: Binance, link: `https://www.binance.com/en` },
    { name: 'Sollet2', icon: Sollet, link: `https://www.binance.com/en` },
    { name: 'Phantom4', icon: Phantom, link: `https://www.binance.com/en` },
    { name: 'MetaMask3', icon: MetaMask, link: `https://www.binance.com/en` },
    { name: 'Coinbase2', icon: Coinbase, link: `https://www.binance.com/en` },
    { name: 'ZenGO4', icon: ZenGO, link: `https://www.binance.com/en` },
    { name: 'Trust4', icon: Trust, detected: true, link: `https://www.binance.com/en` },
    { name: 'Binance4', icon: Binance, link: `https://www.binance.com/en` }
  ],
  onWalletClick: (name: string) => {
    console.log('Item clicked:', name)
  },
  chainIcon: ChainIcon,
  chainName: 'Solana',
  sessionId:
    'fsdhfdzfsdhgfzghggdfhbgchgbdfnvfbxhncvfjhzxdhgbhghfgfvzhfgjhgszdhgzxdfhgfzxdjfuhdfhgd',
  connecting: true,
  relay: 'https://nc2.nightly.app',
  open: true
}

export const Loading: Story = (args: NightlyModalArgs) => {
  const [{ open, sessionId }, updateArgs] = useArgs()

  const handleClose = () => {
    updateArgs({ open: false })
    args.onClose()
  }

  open &&
    setTimeout(() => {
      updateArgs({ sessionId: '1234' })
    }, 2000)

  return open
    ? html`
        <nightly-selector
          .onClose=${handleClose}
          .selectorItems=${args.selectorItems}
          .onWalletClick=${args.onWalletClick}
          .chainIcon=${args.chainIcon}
          .chainName=${args.chainName}
          .sessionId=${sessionId}
          ?connecting=${args.connecting}
          .relay=${args.relay}
        ></nightly-selector>
      `
    : html``
}

let { sessionId: _, ...rest } = Default.args
Loading.args = { ...rest }

export const Error: Story = (args: NightlyModalArgs) => {
  const [{ open, timeoutError }, updateArgs] = useArgs()

  const handleClose = () => {
    updateArgs({ open: false })
    args.onClose()
  }

  if (!args.sessionId)
    setTimeout(() => {
      updateArgs({ timeoutError: 'error' })
    }, 5000)

  return open
    ? html`
        <nightly-selector
          .onClose=${handleClose}
          .selectorItems=${args.selectorItems}
          .onWalletClick=${args.onWalletClick}
          .chainIcon=${args.chainIcon}
          .chainName=${args.chainName}
          ?connecting=${args.connecting}
          .relay=${args.relay}
          .timeoutError=${timeoutError}
        ></nightly-selector>
      `
    : html``
}

Error.args = { ...rest }

export const CustomBlockchain: Story = (args: NightlyModalArgs) => {
  const [{ open }, updateArgs] = useArgs()

  const handleClose = () => {
    updateArgs({ open: false })
    args.onClose()
  }

  return open
    ? html`
        <nightly-selector
          .onClose=${handleClose}
          .selectorItems=${args.selectorItems}
          .onWalletClick=${args.onWalletClick}
          .chainIcon=${args.chainIcon}
          .chainName=${args.chainName}
          .sessionId=${args.sessionId}
          ?connecting=${args.connecting}
          .relay=${args.relay}
        ></nightly-selector>
      `
    : html``
}

CustomBlockchain.args = {
  ...rest,
  chainIcon: 'https://cdn.pixabay.com/photo/2016/04/01/00/28/face-1298202_640.png',
  chainName: 'Custom name'
}
