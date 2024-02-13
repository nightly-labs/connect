import { customElement } from 'lit/decorators.js'
import { tailwindElement } from '../../shared/tailwind.element'
import style from './nightly-footer.css'
import { LitElement, html } from 'lit'

@customElement('nightly-footer')
export class NightlyFooter extends LitElement {
  static styles = tailwindElement(style)

  render() {
    return html`
      <div class="nc_footerWrapper">
        <div class="nc_footerLogo"></div>
        <div class="nc_footerContent">
          <p class="nc_footerText">
            By connecting, you agree to Common's

            <a href="" target="_blank" class="nc_footerLink">Terms of Service</a>
            and to its
            <a href="" target="_blank" class="nc_footerLink">Privacy Policy</a>
          </p>
        </div>
      </div>
    `
  }
}

declare global {
  interface HTMLElementTagNameMap {
    'nightly-footer': NightlyFooter
  }
}
