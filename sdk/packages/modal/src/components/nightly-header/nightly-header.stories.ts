import { Meta, StoryObj } from '@storybook/web-components'
import { html } from 'lit/static-html.js'
import './nightly-header'
import { NightlyHeader } from './nightly-header'

const meta = {
  title: 'nightly-header',
  parameters: {
    layout: 'centered'
  },
  render: (args) => {
    return html`<nightly-header .onClose=${args.onClose}></nightly-header>`
  }
} satisfies Meta<NightlyHeader>

export default meta
type Story = StoryObj<NightlyHeader>

export const Default: Story = {
  name: 'Default',
  args: {
    onClose: () => console.log('close')
  }
}
