import { Meta, StoryObj } from '@storybook/web-components'
import { html } from 'lit'
import { nightlyWalletSelectorItem } from './nightly-wallet-selector-item'
import PhantomIcon from '../../static/svg/PhantomIcon.svg'

import './nightly-wallet-selector-item'

const meta = {
  title: 'nightly-wallet-selector-item',
  parameters: {
    layout: 'centered'
  },
  render: (args) => {
    return html`
      <nightly-wallet-selector-item
        name=${args.name}
        icon=${args.icon}
        recent=${args.recent}
      ></nightly-wallet-selector-item>
    `
  }
} satisfies Meta<nightlyWalletSelectorItem>

export default meta
type Story = StoryObj<nightlyWalletSelectorItem>

export const Default: Story = {
  name: 'Default',
  args: {
    name: 'Phantom',
    icon: PhantomIcon,
    recent: 'Recent'
    // action: true
  }
}
