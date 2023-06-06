import * as parts from './parts'
import qrcode from 'qrcode-generator'
import { getMode, calculateImageSize } from '../utils/utils'
import { defaultXMLOptions, errorCorrectionPercents } from '../utils/consts'
import { XMLOptions } from '../utils/types'

export const generateQrCodeXml = (address: string, options?: Partial<XMLOptions>) => {
  const codeOptions: XMLOptions = {
    ...defaultXMLOptions,
    data: address,
    ...(options ?? {})
  }

  const qr = qrcode(codeOptions.qrOptions.typeNumber, codeOptions.qrOptions.errorCorrectionLevel)
  qr.addData(codeOptions.data, codeOptions.qrOptions.mode || getMode(codeOptions.data))
  qr.make()

  const count = qr.getModuleCount()
  const minSize = Math.min(codeOptions.width, codeOptions.height) - codeOptions.margin * 2
  const dotSize = Math.floor(minSize / count)

  let drawImageSize = {
    hideXDots: 0,
    hideYDots: 0,
    width: 0,
    height: 0
  }

  if (codeOptions.image && codeOptions.imageHeight && codeOptions.imageWidth) {
    const { imageOptions, qrOptions } = codeOptions
    const coverLevel =
      imageOptions.imageSize * errorCorrectionPercents[qrOptions.errorCorrectionLevel]
    const maxHiddenDots = Math.floor(coverLevel * count * count)

    drawImageSize = calculateImageSize({
      originalWidth: codeOptions.imageWidth,
      originalHeight: codeOptions.imageHeight,
      maxHiddenDots,
      maxHiddenAxisDots: count - 14,
      dotSize
    })
  }

  const backgroundNode = parts.background({
    color: codeOptions.backgroundOptions.color,
    x: 0,
    y: 0,
    width: codeOptions.width,
    height: codeOptions.height
  })

  const dotPaths = parts.drawDots(
    qr,
    codeOptions.width,
    codeOptions.height,
    codeOptions.margin,
    codeOptions.dotsOptions.color,
    codeOptions.imageOptions.hideBackgroundDots,
    count,
    drawImageSize
  )

  const cornerPaths = parts.drawCorners(
    codeOptions.width,
    codeOptions.height,
    codeOptions.margin,
    codeOptions.cornersSquareOptions.color,
    codeOptions.cornersDotOptions.color,
    count
  )

  const image = parts.centerImage({
    qrWidth: codeOptions.width,
    qrHeight: codeOptions.height,
    width: drawImageSize.width,
    height: drawImageSize.height,
    count,
    dotSize,
    image: codeOptions.image,
    imageMargin: codeOptions.imageOptions.margin
  })

  const paths = [backgroundNode, ...dotPaths, ...cornerPaths, image]

  return `<svg width="${codeOptions.width}" height="${
    codeOptions.height
  }" xmlns="http://www.w3.org/2000/svg" xmlns:xlink= "http://www.w3.org/1999/xlink">
    ${paths.join('')}
  </svg>`
}

export const generateQrCodeSvgElement = (address: string, options?: Partial<XMLOptions>) => {
  const xml = generateQrCodeXml(address, options)

  const element = document.createElement('div')
  element.innerHTML = xml

  return element.firstChild as SVGElement
}
