import { LitElement, html } from 'lit'
import { customElement } from 'lit/decorators.js'
import { tailwindElement } from '../../shared/tailwind.element'
import style from './nightly-footer.css'

@customElement('nightly-footer')
export class NightlyFooter extends LitElement {
  static styles = tailwindElement(style)

  render() {
    return html`
      <div class="nc_footer">
          By connecting, you agree to Common's <span class="highlight"> Terms of Service</span> and
          to its <span class="highlight"> Privacy Policy</span>.
      </div>
    `
  }
}

declare global {
  interface HTMLElementTagNameMap {
    'nightly-footer': NightlyFooter
  }
}
