import { LitElement, html } from 'lit'
import { tailwindElement } from '../../../shared/tailwind.element'
import { customElement, property } from 'lit/decorators.js'
import { svgToBase64 } from '../../../utils/images'
import { generateQrCodeXml } from '@nightlylabs/qr-code'
import vector from '../../../static/svg/backButton.svg'
import style from './nightly-qrCode.css'

@customElement('nightly-qr-code')
export class NightlyQrCode extends LitElement {
  static styles = tailwindElement(style)

  @property({ type: String })
  sessionId = ''

  @property({ type: String })
  network = ''

  @property({ type: Function })
  showAllWallets!: () => void

  render() {
    return html`
      <div class="headerQrCodeWrapper">
        <div class="headerContainer">
          <div class="buttonContainer">
            <button @click=${this.showAllWallets}>
              <img src=${vector} />
            </button>
          </div>
          <div class="textContainer">
            <span> QR Code </span>
          </div>
        </div>
        <div class="qrCodeWrapper">
          <img
            class="code"
            src=${svgToBase64(
              generateQrCodeXml('nightlyconnect:' + this.sessionId + '?network=' + this.network, {
                width: 432,
                height: 432,
                margin: 5
              })
            )}
          />
        </div>
      </div>
    `
  }
}

declare global {
  interface HTMLElementTagNameMap {
    'nightly-qr-code': NightlyQrCode
  }
}
