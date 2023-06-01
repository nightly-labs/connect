import { LitElement, css, html } from 'lit'
import { customElement, property } from 'lit/decorators.js'
import litLogo from './assets/lit.svg'
import viteLogo from '/vite.svg'
import { TailwindElement } from './shared/tailwind.element'
import style from './my-element.css?inline'
/**
 * An example element.
 *
 * @slot - This element has a slot
 * @csspart button - The button
 */
@customElement('my-element')
export class MyElement extends TailwindElement(style) {
  /**
   * Copy for the read the docs hint.
   */
  @property()
  docsHint = 'Click on the Vite and Lit logos to learn more'

  /**
   * The number of times the button has been clicked.
   */
  @property({ type: Number })
  aNumber = 0

  render() {
    return html`
      <div class="bg-gray-200 big-dick">
        <button @click=${this._onClick}>aNumber is ${this.aNumber}</button>
        <img src="${viteLogo}" />
      </div>
    `
  }

  private _onClick() {
    this.aNumber++
  }
}

declare global {
  interface HTMLElementTagNameMap {
    'my-element': MyElement
  }
}
