import { customElement, property } from 'lit/decorators.js'
import { TailwindElement } from '../../shared/tailwind.element'
import style from './nightly-header-small-page.css?inline'
import { html } from 'lit'
import Logo from '../../static/svg/Logo.svg'
import Close from '../../static/svg/Close.svg'

@customElement('nightly-header-small-page')
export class NightlyHeaderSmallPage extends TailwindElement(style) {
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
        <div class="starsAnimationHeader"></div>
        <div class="cloudsAnimationHeader"></div>
      </div>
    `
  }
}

declare global {
  interface HTMLElementTagNameMap {
    'nightly-header-small-page': NightlyHeaderSmallPage
  }
}
