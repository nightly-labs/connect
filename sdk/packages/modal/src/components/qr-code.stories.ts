import { Meta, StoryObj } from '@storybook/web-components'
import { html } from 'lit'

import './qr-code'
import { QrCode } from './qr-code'

const meta = {
  title: 'QR Code',
  parameters: {
    layout: 'centered'
  },
  render: (args) => {
    return html`<qr-code sessionId=${args.sessionId} network=${args.network}></qr-code>`
  }
} satisfies Meta<QrCode>

export default meta
type Story = StoryObj<QrCode>

export const Default: Story = {
  name: 'Default',
  args: {
    sessionId: 'fsdhfdzfsdhgfzghggdfhbgchgbdfnvfbxhncvfjhzxdhgbhghfgfvzhfgjhgszdhgzxdfhgfzxdjfuhdfhgd',
    network: 'SOLANA'
  }
}