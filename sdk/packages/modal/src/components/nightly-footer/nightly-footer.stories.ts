import { Meta, StoryObj } from '@storybook/web-components'
import { html } from 'lit/static-html.js'
import './nightly-footer'
import { NightlyFooter } from './nightly-footer'

const meta = {
  title: 'nightly-footer',
  parameters: {
    layout: 'centered'
  },
  render: () => {
    return html`<nightly-footer></nightly-footer>`
  }
} satisfies Meta<NightlyFooter>

export default meta
type Story = StoryObj<NightlyFooter>

export const Default: Story = {
  name: 'Default',

}
