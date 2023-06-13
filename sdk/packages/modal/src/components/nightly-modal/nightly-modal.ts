import { customElement, property } from 'lit/decorators.js'
import { TailwindElement } from '../../shared/tailwind.element'
import style from './nightly-modal.css?inline'
import { html } from 'lit'

@customElement('nightly-modal')
export class NightlyModal extends TailwindElement(style) {
  @property()
  // eslint-disable-next-line @typescript-eslint/no-empty-function
  onClose = () => {}

  render() {
    return html`<nightly-header .onClose=${this.onClose}></nightly-header>`
  }
}

declare global {
  interface HTMLElementTagNameMap {
    'nightly-modal': NightlyModal
  }
}
