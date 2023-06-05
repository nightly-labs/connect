import { customElement, property } from 'lit/decorators.js'
import { TailwindElement } from '../../shared/tailwind.element'
import style from './nightly-header.css?inline'
import { html } from 'lit'
import Logo from '../../static/svg/Logo.svg'
import Close from '../../static/svg/Close.svg'

@customElement('nightly-header')
export class NightlyHeader extends TailwindElement(style) {
  @property()
  onClose = () => console.log()

  render() {
    return html`
      <div class="mainContainer-header">
        <div class="logoContainer">
          <img class="header-logo" src=${Logo} />
          <button class="closeButton">
            <img src=${Close} this.onClose />
          </button>
        </div>
        <div class="starsAnimationHeader"></div>
        <div class="cloudsAnimationHeader"></div>
      </div>
    `
  }
}

declare global {
  interface HTMLElementTagNameMap {
    'nightly-header': NightlyHeader
  }
}
