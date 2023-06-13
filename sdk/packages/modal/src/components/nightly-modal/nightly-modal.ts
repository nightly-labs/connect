import { customElement, property } from 'lit/decorators.js'
import { TailwindElement } from '../../shared/tailwind.element'
import style from './nightly-modal.css?inline'
import { html } from 'lit'
import copy from '../../static/svg/copy.svg'
import scan from '../../static/svg/scan.svg'
import { svgToBase64 } from '../../utils/images'
import { generateQrCodeXml } from '@nightlylabs/qr-code'

@customElement('nightly-modal')
export class NightlyModal extends TailwindElement(style) {
  @property()
  // eslint-disable-next-line @typescript-eslint/no-empty-function
  onClose = () => {}

  @property({ type: Array })
  selectorItems = []

  @property()
  // eslint-disable-next-line @typescript-eslint/no-empty-function
  onWalletClick = () => {}

  @property({ type: String }) chainIcon = ''

  @property({ type: String }) chainName = ''

  /**
   * Id of current session
   */
  @property({ type: String })
  sessionId = ''

  /**
   * Network of current session
   */
  @property({ type: String })
  network = ''

  onCopy = () => {
    navigator.clipboard.writeText('nightlyconnect:' + this.sessionId + '?network=' + this.network)
  }

  render() {
    return html`
      <nightly-header .onClose=${this.onClose}></nightly-header>
      <div class="bottomContainer">
        <div class="qrContainer">
          <div class="qrTop">
            <div class="scan"><img class="scanImg" src=${scan} /> Scan QR code</div>
            <div class="copy" @click=${this.onCopy}><img class="copyImg" src=${copy} /> Copy</div>
          </div>
          <img
            class="code"
            src=${svgToBase64(
              generateQrCodeXml('nightlyconnect:' + this.sessionId + '?network=' + this.network, {
                width: 400,
                height: 400,
                margin: 10
              })
            )}
          />
        </div>
        <nightly-wallet-selector-page
          .selectorItems=${this.selectorItems}
          .onWalletClick=${this.onWalletClick}
          .chainIcon=${this.chainIcon}
          .chainName=${this.chainName}
        ></nightly-wallet-selector-page>
      </div>
    `
  }
}

declare global {
  interface HTMLElementTagNameMap {
    'nightly-modal': NightlyModal
  }
}