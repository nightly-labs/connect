import { LitElement, PropertyValueMap, html } from 'lit'
import { tailwindElement } from '../../shared/tailwind.element'
import { customElement, property, state } from 'lit/decorators.js'
import { svgToBase64 } from '../../utils/images'
import { XMLOptions, generateQrCodeXml } from '@nightlylabs/qr-code'
import style from './nightly-mobile-qr.css'

@customElement('nightly-mobile-qr')
export class NightlyMobileQr extends LitElement {
  static styles = tailwindElement(style)

  @property({ type: String })
  sessionId = ''

  @property({ type: String })
  relay = ''

  @property({ type: String })
  chainName = ''

  @property({ type: Function })
  showAllWallets!: () => void

  @property({ type: Object })
  qrConfigOverride: Partial<XMLOptions> = {}

  @state()
  qrSource: string | undefined = undefined

  @state()
  timeoutError: boolean = false

  @state()
  isSessionIdImmediatelyDefined: boolean = false

  loadingTimeout: number | undefined = undefined

  private updateQrSource = () => {
    if (this.sessionId) {
      clearTimeout(this.loadingTimeout)

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
  }

  connectedCallback(): void {
    super.connectedCallback()

    this.loadingTimeout = setTimeout(() => {
      if (this.sessionId) clearTimeout(this.loadingTimeout)
      // timeout error when sessionId takes longer than 5 seconds to arrive
      else {
        clearTimeout(this.loadingTimeout)
        this.timeoutError = true
      }
    }, 5000) as unknown as number

    this.updateQrSource()
    if (this.sessionId) this.isSessionIdImmediatelyDefined = true
  }

  disconnectedCallback(): void {
    super.disconnectedCallback()

    clearTimeout(this.loadingTimeout)
  }

  protected updated(_changedProperties: PropertyValueMap<any> | Map<PropertyKey, unknown>): void {
    super.updated(_changedProperties)

    this.updateQrSource()
  }

  render() {
    return html`
      <div class="nc_mobileQrWrapper">
        <div class="nc_mobileQrTopBar">
          <button class="nc_mobileQrBackButton" @click=${this.showAllWallets}></button>
          <span class="nc_mobileQrTitle"> QR Code </span>
          <div class="nc_mobileQrTopJustify"></div>
        </div>
        <img class="nc_mobileQrCode" src=${this.qrSource} />

        ${!this.isSessionIdImmediatelyDefined &&
        html`<div
          class="nc_mobileQrLoaderOverlay ${this.qrSource ? 'nc_mobileQrLoadedOverlayFadeOut' : ''}"
        >
          <button class="nc_mobileQrBackButtonLoader" @click=${this.showAllWallets}></button>
          <img
            src="https://registry.nightly.app/gifs/loading.gif"
            alt="Loading"
            class="nc_mobileQrLoader"
          />
          <h3 class="nc_mobileQrLoaderLabel">Generating QR code...</h3>
        </div>`}

        <div
          class="nc_mobileQrTimeoutErrorOverlay ${this.timeoutError &&
          'nc_mobileQrTimeoutErrorOverlayFadeIn'}"
        >
          <button class="nc_mobileQrBackButtonTimeoutError" @click=${this.showAllWallets}></button>
          <img
            src="https://registry.nightly.app/images/fox_sad.gif"
            alt="Timeout error"
            class="nc_mobileQrTimeoutError"
          />
          <h3 class="nc_mobileQrTimeoutErrorLabel">QR code couldn’t be generated...</h3>
          <p class="nc_mobileQrTimeoutErrorLabelDescription">
            Make sure you have stable internet connection.
          </p>
        </div>
      </div>
    `
  }
}

declare global {
  interface HTMLElementTagNameMap {
    'nightly-mobile-qr': NightlyMobileQr
  }
}
