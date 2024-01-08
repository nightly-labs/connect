import { customElement} from 'lit/decorators.js'
import { tailwindElement } from '../../shared/tailwind.element'
import style from './nightly-footer.css'
import { LitElement, html } from 'lit'

@customElement('nightly-footer')
export class NightlyFooter extends LitElement {
  static styles = tailwindElement(style)

  render() {
    return html`
    <div class="nc_modalFooter">
        <div class="nc_modalFooterCover"></div>
        By connecting, you agree to Common's <a>Terms of Service</a> and to its <a>Privacy Policy</a>.
    </div>
    `
  }
}

declare global {
  interface HTMLElementTagNameMap {
    'nightly-footer': NightlyFooter
  }
}
