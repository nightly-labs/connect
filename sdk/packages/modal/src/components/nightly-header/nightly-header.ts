import { customElement, property } from 'lit/decorators.js'
import { tailwindElement } from '../../shared/tailwind.element'
import style from './nightly-header.css'
import { LitElement, html } from 'lit'
import Logo from '../../static/svg/Logo.svg'
import Close from '../../static/svg/Close.svg'
import Clouds from '../../static/svg/Clouds.svg'
import Stars from '../../static/svg/Stars.svg'
import { styleMap } from 'lit/directives/style-map.js'

@customElement('nightly-header')
export class NightlyHeader extends LitElement {
  static styles = tailwindElement(style)

  @property()
  // eslint-disable-next-line @typescript-eslint/no-empty-function
  onClose = () => {}

  @property({ type: String })
  height = '56px'

  render() {
    const styles = { height: this.height }
    return html`
      <div class="mainContainer-header" style=${styleMap(styles)}>
        <div class="logoContainer">
          <img class="header-logo" src=${Logo} />
          <button class="closeButton" @click=${this.onClose}>
            <img src=${Close} />
          </button>
        </div>
        <div
          class="starsAnimationHeader"
          style=${styleMap({ 'background-image': `url("${Stars}")` })}
        ></div>
        <div
          class="cloudsAnimationHeader"
          style=${styleMap({ 'background-image': `url("${Clouds}")` })}
        ></div>
      </div>
    `
  }
}

declare global {
  interface HTMLElementTagNameMap {
    'nightly-header': NightlyHeader
  }
}
