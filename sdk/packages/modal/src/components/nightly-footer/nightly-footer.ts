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
        By connecting, you agree to Common's
        <a href="#" target="_blank" class="highlight"> Terms of Service</a> and to its
        <a href="#" target="_blank" class="highlight"> Privacy Policy</a>.
      </div>
    `
  }
}

declare global {
  interface HTMLElementTagNameMap {
    'nightly-footer': NightlyFooter
  }
}
