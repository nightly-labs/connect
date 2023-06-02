import { Meta, StoryObj } from '@storybook/web-components'
import { html } from 'lit'

import './my-element'
import { MyElement } from './my-element'
const meta = {
  title: 'My Element',
  parameters: {
    layout: 'centered'
  },
  render: (args) => {
    return html`<my-element aNumber=${args.aNumber}></my-element>`
  }
} satisfies Meta<MyElement>

export default meta
type Story = StoryObj<MyElement>

export const Default: Story = {
  name: 'Default',
  args: {
    aNumber: 121
  }
}
