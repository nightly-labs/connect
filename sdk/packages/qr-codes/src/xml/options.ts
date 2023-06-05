import { qrTypes } from '../utils/consts'
import { ErrorCorrectionLevel, Mode, TypeNumber } from '../utils/types'
import { fennecXml } from './consts'

// TODO: I'm gonna add other options later or perhaps even create a whole new lib, it's too much work for now
export interface XMLOptions {
  image?: string
  imageWidth?: number
  imageHeight?: number
  width: number
  height: number
  margin: number
  data: string
  qrOptions: {
    typeNumber: TypeNumber
    mode?: Mode
    errorCorrectionLevel: ErrorCorrectionLevel
  }
  imageOptions: {
    hideBackgroundDots: boolean
    imageSize: number
    crossOrigin?: string
    margin: number
  }
  dotsOptions: {
    color: string
  }
  cornersDotOptions: {
    color: string
  }
  cornersSquareOptions: {
    color: string
  }
  backgroundOptions: {
    color: string
  }
}

export const defaultXMLOptions: XMLOptions = {
  data: '',
  width: 188,
  height: 188,
  margin: 0,
  dotsOptions: {
    color: '#6067F9'
  },
  cornersDotOptions: {
    color: '#6067F9'
  },
  cornersSquareOptions: {
    color: '#FFFFFF'
  },
  backgroundOptions: {
    color: '#17182B'
  },
  imageOptions: {
    margin: 0,
    imageSize: 0.4,
    hideBackgroundDots: true,
    crossOrigin: undefined
  },
  qrOptions: {
    typeNumber: qrTypes[0],
    mode: undefined,
    errorCorrectionLevel: 'Q'
  },
  image: fennecXml,
  imageWidth: 1510, // workaround for problem with getting image size
  imageHeight: 1660 // way used on web is incompatible with one used on native and vice versa
}
