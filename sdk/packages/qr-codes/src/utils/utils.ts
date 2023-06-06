import { modes } from './consts'
import { ImageSizeResult, Mode } from './types'

export const getMode = (data: string): Mode => {
  switch (true) {
    case /^[0-9]*$/.test(data):
      return modes.numeric
    case /^[0-9A-Z $%*+\-./:]*$/.test(data):
      return modes.alphanumeric
    default:
      return modes.byte
  }
}

export const calculateImageSize = ({
  originalHeight,
  originalWidth,
  maxHiddenDots,
  maxHiddenAxisDots,
  dotSize
}: {
  originalHeight: number
  originalWidth: number
  maxHiddenDots: number
  maxHiddenAxisDots?: number
  dotSize: number
}
): ImageSizeResult => {
  const hideDots = { x: 0, y: 0 }
  const imageSize = { x: 0, y: 0 }

  if (originalHeight <= 0 || originalWidth <= 0 || maxHiddenDots <= 0 || dotSize <= 0) {
    return {
      height: 0,
      width: 0,
      hideYDots: 0,
      hideXDots: 0
    }
  }

  const k = originalHeight / originalWidth

  //Getting the maximum possible axis hidden dots
  hideDots.x = Math.floor(Math.sqrt(maxHiddenDots / k))
  //The count of hidden dot's can't be less than 1
  if (hideDots.x <= 0) hideDots.x = 1
  //Check the limit of the maximum allowed axis hidden dots
  if (maxHiddenAxisDots && maxHiddenAxisDots < hideDots.x) hideDots.x = maxHiddenAxisDots
  //The count of dots should be odd
  if (hideDots.x % 2 === 0) hideDots.x--
  imageSize.x = hideDots.x * dotSize
  //Calculate opposite axis hidden dots based on axis value.
  //The value will be odd.
  //We use ceil to prevent dots covering by the image.
  hideDots.y = 1 + 2 * Math.ceil((hideDots.x * k - 1) / 2)
  imageSize.y = Math.round(imageSize.x * k)
  //If the result dots count is bigger than max - then decrease size and calculate again
  if (
    hideDots.y * hideDots.x > maxHiddenDots ||
    (maxHiddenAxisDots && maxHiddenAxisDots < hideDots.y)
  ) {
    if (maxHiddenAxisDots && maxHiddenAxisDots < hideDots.y) {
      hideDots.y = maxHiddenAxisDots
      if (hideDots.y % 2 === 0) hideDots.x--
    } else {
      hideDots.y -= 2
    }
    imageSize.y = hideDots.y * dotSize
    hideDots.x = 1 + 2 * Math.ceil((hideDots.y / k - 1) / 2)
    imageSize.x = Math.round(imageSize.y / k)
  }

  return {
    height: imageSize.y,
    width: imageSize.x,
    hideYDots: hideDots.y,
    hideXDots: hideDots.x
  }
}
