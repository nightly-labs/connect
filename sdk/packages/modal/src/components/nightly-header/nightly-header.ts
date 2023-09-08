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
  static styles = tailwindElement(
    style,
    `
    .nc_headerLogo {
      background-image: url("${Logo}");
    }

    .nc_headerCloseButton {
      background-image: url("${Close}");
    }

      .nc_headerAnimatedBgBackground {
        background-image: url("${Stars}");
      }

      .nc_headerAnimatedBgForeground {
        background-image: url("${Clouds}");
      }
    `
  )

  @property()
  // eslint-disable-next-line @typescript-eslint/no-empty-function
  onClose = () => {}

  render() {
    return html`
      <div class="nc_headerWrapper">
        <div class="nc_headerLogo"></div>
        <button class="nc_headerCloseButton" @click=${this.onClose}></button>
        <div class="nc_headerAnimatedBgWrapper">
          <div class="nc_headerAnimatedBgBackground"></div>
          <div class="nc_headerAnimatedBgForeground"></div>
        </div>
      </div>
    `
  }
}

declare global {
  interface HTMLElementTagNameMap {
    'nightly-header': NightlyHeader
  }
}
