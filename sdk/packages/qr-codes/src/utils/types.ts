export enum ErrorCorrectionLevel {
  L = 'L',
  M = 'M',
  Q = 'Q',
  H = 'H'
}

export enum Mode {
  Numeric = 'Numeric',
  Alphanumeric ='Alphanumeric',
  Byte = 'Byte',
  Kanji = 'Kanji'
}

export interface ImageSizeResult {
  height: number
  width: number
  hideYDots: number
  hideXDots: number
}

type Enumerate<N extends number, Acc extends number[] = []> = Acc['length'] extends N
  ? Acc[number]
  : Enumerate<N, [...Acc, Acc['length']]>

type Range<F extends number, T extends number> = Exclude<Enumerate<T>, Enumerate<F>>

type TypeNum = Range<0, 41>

export interface XMLOptions {
  image?: string
  imageWidth?: number
  imageHeight?: number
  width: number
  height: number
  margin: number
  data: string
  qrOptions: {
    typeNumber: TypeNum
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
