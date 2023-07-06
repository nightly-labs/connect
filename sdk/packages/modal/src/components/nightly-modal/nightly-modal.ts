import { customElement, property, state } from 'lit/decorators.js'
import { tailwindElement } from '../../shared/tailwind.element'
import style from './nightly-modal.css'
import { LitElement, html } from 'lit'
import copy from '../../static/svg/copy.svg'
import scan from '../../static/png/scan.png'
import { svgToBase64 } from '../../utils/images'
import { generateQrCodeXml } from '@nightlylabs/qr-code'
import '../nightly-wallet-selector-page/nightly-wallet-selector-page'

@customElement('nightly-modal')
export class NightlyModal extends LitElement {
  static styles = tailwindElement(style)

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
  relay = ''

  @state()
  copyMessage = 'Copy'

  timeoutRef: number | undefined = undefined

  onCopy = () => {
    navigator.clipboard.writeText(
      'nc:' + this.sessionId + '?network=' + this.network + '&relay=' + this.relay
    )
    this.copyMessage = 'Copied!'
    clearTimeout(this.timeoutRef)
    this.timeoutRef = setTimeout(() => {
      this.copyMessage = 'Copy'
    }, 1000) as unknown as number
  }

  render() {
    return html`
      <div class="mainContainer">
        <div class="bottomContainer">
          <div class="qrContainer">
            <div class="qrTop">
              <div class="scan"><img class="scanImg" src=${scan} />Scan QR code</div>
              <button id="nightly-modal-copy-button" class="copy" @click=${this.onCopy}>
                <img class="copyImg" src=${copy} />${this.copyMessage}
              </button>
            </div>
            <img
              class="code"
              src=${svgToBase64(
                generateQrCodeXml(
                  'nc:' + this.sessionId + '?network=' + this.network + '&relay=' + this.relay,
                  {
                    width: 400,
                    height: 400,
                    margin: 10
                  }
                )
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
