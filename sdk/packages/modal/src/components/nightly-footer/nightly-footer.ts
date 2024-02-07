import { customElement, property } from 'lit/decorators.js'
import { tailwindElement } from '../../shared/tailwind.element'
import style from './nightly-footer.css'
import { LitElement, html } from 'lit'

@customElement('nightly-footer')
export class NightlyFooter extends LitElement {
  static styles = tailwindElement(style)

  @property({ type: HTMLElement || String })
  content: unknown = ''

  render() {
    return (
      this.content &&
      html`
        <div class="nc_modalFooter">
          <div class="nc_modalFooterCover"></div>

          ${this.content}
        </div>
      `
    )
  }
}

declare global {
  interface HTMLElementTagNameMap {
    'nightly-footer': NightlyFooter
  }
}
