// Button.stories.ts
import { Meta, StoryObj } from '@storybook/html'
// import { CommonModule } from "@angular/common";
import { Button, ButtonProps } from './Button'

export default {
  component: Button,
  decorators: [],
  tags: ['autodocs']
} as Meta<ButtonProps>

export const Primary: StoryObj<ButtonProps> = {
  args: {
    backgroundColor: 'red',
    size: 'large',
    onClick: () => console.log('click')
  }
}
