import { generateNightlyQRCodeXML } from '@nightlylabs/qr-code'
import { html } from 'lit'
import { customElement, property } from 'lit/decorators.js'
import { TailwindElement } from '../shared/tailwind.element'
import style from './qr-code.css?inline'
import { svgToBase64 } from '../utils/images'

@customElement('qr-code')
export class QrCode extends TailwindElement(style) {
  /**
   * Id of current session
   */
  @property({ type: String })
  sessionId = ''

  /**
   * Network of current session
   */
  @property({ type: String })
  network =''

  render() {
    return html`
      <div class="qr-wrapper">
        <img class="code" src=${svgToBase64(generateNightlyQRCodeXML('nightlyconnect:' + this.sessionId + '?network=' + this.network, { width: 400, height: 400, margin: 4 }))} />
      </div>
    `
  }
}

declare global {
  interface HTMLElementTagNameMap {
    'qr-code': QrCode
  }
}