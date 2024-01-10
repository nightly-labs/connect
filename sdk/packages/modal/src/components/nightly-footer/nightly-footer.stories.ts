import { Meta, StoryObj } from '@storybook/web-components'
import { html } from 'lit/static-html.js'
import './nightly-footer'
import { NightlyFooter } from './nightly-footer'

const meta = {
  title: 'nightly-footer',
  parameters: {
    layout: 'centered'
  },
  render: (args) => {
    return html`<nightly-footer .content=${args.content}></nightly-footer>`
  }
} satisfies Meta<NightlyFooter>

export default meta
type Story = StoryObj<NightlyFooter>

export const Default: Story = {
  name: 'Default',
  args: {
    content: html`By connecting, you agree to Common's <a href="#">Terms of Service</a> and to its
      <a href="#">Privacy Policy</a>.`
  }
}
