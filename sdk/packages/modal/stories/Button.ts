import './button.css'

export interface ButtonProps {
  primary?: boolean
  size?: 'small' | 'medium' | 'large'
  backgroundColor?: string
  onClick?: () => void
}
export const Button = ({
  primary = false,
  size = 'medium',
  backgroundColor = 'pink',
  onClick = () => {}
}: ButtonProps) => {
  const btn = document.createElement('button')
  btn.type = 'button'
  btn.innerText = 'sssdds'
  btn.addEventListener('click', onClick)

  const mode = primary ? 'storybook-button--primary' : 'storybook-button--secondary'
  btn.className = ['storybook-button', `storybook-button--${size}`, mode].join(' ')

  btn.style.backgroundColor = backgroundColor

  return btn
}
