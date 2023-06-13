import { Meta, StoryObj } from '@storybook/web-components'
import { html } from 'lit/static-html.js'
import './nightly-modal'
import { NightlyModal } from './nightly-modal'

const meta = {
  title: 'nightly-modal',
  parameters: {
    layout: 'centered'
  },
  render: (args) => {
    return html`<nightly-modal .onClose=${args.onClose}></nightly-modal>`
  }
} satisfies Meta<NightlyModal>

export default meta
type Story = StoryObj<NightlyModal>

export const Default: Story = {
  name: 'Default',
  args: {
    onClose: () => console.log('close')
  }
}