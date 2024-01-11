import { Meta, StoryObj } from '@storybook/web-components'
import { html } from 'lit/static-html.js'
import { NightlyFooter } from './nightly-footer'
import './nightly-footer'

const meta = {
  title: 'nightly-footer',
  parameters: {
    layout: 'centered'
  },
  render: (args) => {
    return html`<nightly-footer
      .termsOfService=${args.termsOfService}
      .privacyPolicy=${args.privacyPolicy}
    ></nightly-footer>`
  }
} satisfies Meta<NightlyFooter>

export default meta
type Story = StoryObj<NightlyFooter>

export const Default: Story = {
  name: 'Default',
}
