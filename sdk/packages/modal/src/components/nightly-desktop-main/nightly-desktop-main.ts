import { customElement, property, state } from 'lit/decorators.js'
import { tailwindElement } from '../../shared/tailwind.element'
import style from './nightly-desktop-main.css'
import { LitElement, PropertyValueMap, html } from 'lit'
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
  onWalletClick: (name: string) => void = () => { }

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

  @property({ type: String })
  timeoutError = ''

  @state()
  copyMessage = 'Copy'

  @state()
  qrSource: string | undefined = undefined

  @state()
  isSessionIdImmediatelyDefined: boolean = false

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

  private updateQrSource = () => {
    if (this.sessionId)
      this.qrSource = svgToBase64(
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
      )
  }

  connectedCallback(): void {
    super.connectedCallback()

    this.updateQrSource()
    if (this.sessionId) this.isSessionIdImmediatelyDefined = true
  }

  protected updated(_changedProperties: PropertyValueMap<any> | Map<PropertyKey, unknown>): void {
    super.updated(_changedProperties)

    this.updateQrSource()
  }

  // TODO: change loading animation

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
          <img id="qrCode" class="nc_desktopMainQrCode" src=${this.qrSource} />

          ${!this.isSessionIdImmediatelyDefined ?
        html`<div
            class="nc_desktopQrLoaderOverlay ${this.qrSource && !this.timeoutError
            ? 'nc_desktopQrLoadedOverlayFadeOut'
            : ''}"
          >
            <img
              src="https://registry.nightly.app/gifs/loading.gif"
              alt="Loading"
              class="nc_desktopQrLoader"
            />
            <h3 class="nc_desktopQrLoaderLabel">Generating QR code...</h3>
          </div>` : html``}

          <div
            class="nc_desktopQrTimeoutErrorOverlay ${this.timeoutError &&
      'nc_desktopQrTimeoutErrorOverlayFadeIn'}"
          >
            <img
              src="https://registry.nightly.app/images/fox_sad.gif"
              alt="Timeout error"
              class="nc_desktopQrTimeoutError"
            />
            <h3 class="nc_desktopQrTimeoutErrorLabel">QR code couldn’t be generated...</h3>
            <p class="nc_desktopQrTimeoutErrorLabelDescription">
              Make sure you have stable internet connection.
            </p>
          </div>
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
