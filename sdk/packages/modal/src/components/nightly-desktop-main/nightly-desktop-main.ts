import { customElement, property, state } from 'lit/decorators.js'
import { tailwindElement } from '../../shared/tailwind.element'
import style from './nightly-desktop-main.css'
import { LitElement, html } from 'lit'
import { svgToBase64 } from '../../utils/images'
import { XMLOptions, generateQrCodeXml } from '@nightlylabs/qr-code'
import '../nightly-wallet-selector-page/nightly-wallet-selector-page'

@customElement('nightly-desktop-main')
export class NightlyDesktopMain extends LitElement {
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
  relay = ''

  @property({ type: Object })
  qrConfigOverride: Partial<XMLOptions> = {}

  @state()
  copyMessage = 'Copy'

  timeoutRef: number | undefined = undefined

  onCopy = () => {
    navigator.clipboard.writeText(
      'nc:' +
        this.sessionId +
        '?network=' +
        this.chainName.replace(/\s/g, '') +
        '&relay=' +
        this.relay
    )
    this.copyMessage = 'Copied!'
    clearTimeout(this.timeoutRef)
    this.timeoutRef = setTimeout(() => {
      this.copyMessage = 'Copy'
    }, 1000) as unknown as number
  }

  render() {
    return html`
      <div class="nc_desktopMainWrapper">
        <div class="nc_desktopMainQrWrapper">
          <div class="nc_desktopMainQrTopBar">
            <div class="nc_desktopMainQrScanInfo">
              <div class="nc_desktopMainQrScanInfoIcon"></div>
              Scan QR code
            </div>
            <button class="nc_desktopMainQrCopyInfo" @click=${this.onCopy}>
              <div class="nc_desktopMainQrCopyInfoIcon"></div>
              ${this.copyMessage}
            </button>
          </div>
          <img
            class="nc_desktopMainQrCode"
            src=${svgToBase64(
              generateQrCodeXml(
                'nc:' +
                  this.sessionId +
                  '?network=' +
                  this.chainName.replace(/\s/g, '') +
                  '&relay=' +
                  this.relay,
                {
                  width: 500,
                  height: 500,
                  margin: 10,
                  ...this.qrConfigOverride
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
    `
  }
}

declare global {
  interface HTMLElementTagNameMap {
    'nightly-desktop-main': NightlyDesktopMain
  }
}
