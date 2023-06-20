import { customElement, property } from 'lit/decorators.js'
import { TailwindElement } from '../../shared/tailwind.element'
import style from './nightly-modal.css?inline'
import { html } from 'lit'
import copy from '../../static/svg/copy.svg'
import scan from '../../static/svg/scan.svg'
import { svgToBase64 } from '../../utils/images'
import { generateQrCodeXml } from '@nightlylabs/qr-code'
import '../nightly-wallet-selector-page/nightly-wallet-selector-page'
import '../nightly-header/nightly-header'

@customElement('nightly-modal')
export class NightlyModal extends TailwindElement(style) {
  @property()
  // eslint-disable-next-line @typescript-eslint/no-empty-function
  onClose = () => {}

  @property({ type: Array })
  selectorItems = []

  @property({ type: Function })
  // eslint-disable-next-line @typescript-eslint/no-empty-function
  onWalletClick: (name: string) => void = () => {}

  @property({ type: String })
  chainIcon = ''

  @property({ type: String })
  chainName = ''

  @property({ type: String })
  sessionId = ''

  @property({ type: String })
  network = ''

  @property({ type: String })
  copyMessage = 'Copy'

  timeoutRef: number | undefined = undefined

  onCopy = () => {
    navigator.clipboard.writeText('nightlyconnect:' + this.sessionId + '?network=' + this.network)
    this.copyMessage = 'Copied!'
    clearTimeout(this.timeoutRef)
    this.timeoutRef = setTimeout(() => {
      this.copyMessage = 'Copy'
    }, 1000) as unknown as number
  }

  render() {
    return html`
      <div class="mainContainer">
        <nightly-header .onClose=${this.onClose}></nightly-header>
        <div class="bottomContainer">
          <div class="qrContainer">
            <div class="qrTop">
              <div class="scan"><img class="scanImg" src=${scan} />Scan QR code</div>
              <div class="copy" @click=${this.onCopy}>
                <img class="copyImg" src=${copy} />${this.copyMessage}
              </div>
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
      </div>
    `
  }
}

declare global {
  interface HTMLElementTagNameMap {
    'nightly-modal': NightlyModal
  }
}
