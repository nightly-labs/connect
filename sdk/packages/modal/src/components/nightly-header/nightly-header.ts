import { customElement, property } from 'lit/decorators.js'
import { tailwindElement } from '../../shared/tailwind.element'
import style from './nightly-header.css'
import { LitElement, html } from 'lit'
import Logo from '../../static/svg/Logo.svg'
import Close from '../../static/svg/Close.svg'
import Clouds from '../../static/svg/Clouds.svg'
import Stars from '../../static/svg/Stars.svg'

@customElement('nightly-header')
export class NightlyHeader extends LitElement {
  static styles = tailwindElement(style)

  @property()
  // eslint-disable-next-line @typescript-eslint/no-empty-function
  onClose = () => {}

  render() {
    return html`
      <div class="mainContainer-header">
        <div class="logoContainer">
          <img class="header-logo" src=${Logo} />
          <button class="closeButton" @click=${this.onClose}>
            <img src=${Close} />
          </button>
        </div>
        <img src=${Stars} class="starsAnimationHeader"></img>
        <img src=${Clouds} class="cloudsAnimationHeader"></img>
      </div>
    `
  }
}

declare global {
  interface HTMLElementTagNameMap {
    'nightly-header': NightlyHeader
  }
}
