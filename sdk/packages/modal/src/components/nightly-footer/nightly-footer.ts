import { customElement, property } from 'lit/decorators.js'
import { tailwindElement } from '../../shared/tailwind.element'
import style from './nightly-footer.css'
import { LitElement, html } from 'lit'

@customElement('nightly-footer')
export class NightlyFooter extends LitElement {
  static styles = tailwindElement(style)

  @property({ type: String }) termsOfServiceUrl = '';
  @property({ type: String }) privacyPolicyUrl = '';

  render() {
    return html`
      <div class="nc_footerWrapper">
        <div class="nc_footerText">
          By connecting, you agree to
          <a href="${this.termsOfServiceUrl}" target="_blank">Common's Terms of Service</a>
          and to its
          <a href="${this.privacyPolicyUrl}" target="_blank">Privacy Policy</a>
        </div>
      </div>
    `;
  }
}

declare global {
  interface HTMLElementTagNameMap {
    'nightly-footer': NightlyFooter
  }
}
