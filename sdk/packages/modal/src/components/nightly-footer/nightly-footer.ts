import { customElement, property } from 'lit/decorators.js'
import { tailwindElement } from '../../shared/tailwind.element'
import style from './nightly-footer.css'
import { LitElement, html } from 'lit'

@customElement('nightly-footer')
export class NightlyFooter extends LitElement {
  static styles = tailwindElement(style)

  @property({ type: String })
  termsOfService = '#'

  @property({ type: String })
  privacyPolicy = '#'

  render() {
    return html`
        <div class="nc_footerWrapper">
            <p class="nc_footerParagraph">By connecting, you agree to Common's 
                <a href=${this.termsOfService} class="nc_footerLink">Terms of Service</a>
            and to its
                <a href=${this.privacyPolicy} class="nc_footerLink">Privacy Policy</a>.
            </p>
        </div>
    `
  }
}

declare global {
  interface HTMLElementTagNameMap {
    'nightly-footer': NightlyFooter
  }
}
