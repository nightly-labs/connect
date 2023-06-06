import { ImageSizeResult } from '../utils/types'

export const rotationString = ({
  x,
  y,
  size,
  rotation = 0
}: {
  x: number
  y: number
  size: number
  rotation?: number
}) => {
  const cx = x + size / 2
  const cy = y + size / 2

  return 'transform=' + `"rotate(${(180 * rotation) / Math.PI},${cx},${cy})"`
}

export const tearCornerDot = ({
  x,
  y,
  size,
  rotation = 0,
  color
}: {
  x: number
  y: number
  size: number
  rotation?: number
  color: string
}) => {
  const dotSize = size / 7

  const d =
    `M ${x} ${y + 2.5 * dotSize}` +
    `v ${2 * dotSize}` +
    `a ${2.5 * dotSize} ${2.5 * dotSize}, 0, 0, 0, ${dotSize * 2.5} ${dotSize * 2.5}` +
    `h ${4.5 * dotSize}` +
    `v ${-4.5 * dotSize}` +
    `a ${2.5 * dotSize} ${2.5 * dotSize}, 0, 0, 0, ${-dotSize * 2.5} ${-dotSize * 2.5}` +
    `h ${-2 * dotSize}` +
    `a ${2.5 * dotSize} ${2.5 * dotSize}, 0, 0, 0, ${-dotSize * 2.5} ${dotSize * 2.5}`

  return `<path clip-rule="evenodd" fill="${color}" d="${d}" ${rotationString({
    x,
    y,
    size,
    rotation
  })}></path>`
}

export const tearCornerSquare = ({
  x,
  y,
  size,
  rotation = 0,
  color
}: {
  x: number
  y: number
  size: number
  rotation?: number
  color: string
}) => {
  const dotSize = size / 7

  const d =
    `M ${x} ${y + 2.5 * dotSize}` +
    `v ${2 * dotSize}` +
    `a ${2.5 * dotSize} ${2.5 * dotSize}, 0, 0, 0, ${dotSize * 2.5} ${dotSize * 2.5}` +
    `h ${4.5 * dotSize}` +
    `v ${-4.5 * dotSize}` +
    `a ${2.5 * dotSize} ${2.5 * dotSize}, 0, 0, 0, ${-dotSize * 2.5} ${-dotSize * 2.5}` +
    `h ${-2 * dotSize}` +
    `a ${2.5 * dotSize} ${2.5 * dotSize}, 0, 0, 0, ${-dotSize * 2.5} ${dotSize * 2.5}` +
    `M ${x + 2.5 * dotSize} ${y + dotSize}` +
    `h ${2 * dotSize}` +
    `a ${1.5 * dotSize} ${1.5 * dotSize}, 0, 0, 1, ${dotSize * 1.5} ${dotSize * 1.5}` +
    `v ${3.5 * dotSize}` +
    `h ${-3.5 * dotSize}` +
    `a ${1.5 * dotSize} ${1.5 * dotSize}, 0, 0, 1, ${-dotSize * 1.5} ${-dotSize * 1.5}` +
    `v ${-2 * dotSize}` +
    `a ${1.5 * dotSize} ${1.5 * dotSize}, 0, 0, 1, ${dotSize * 1.5} ${-dotSize * 1.5}`

  return `<path clip-rule="evenodd" fill="${color}" d="${d}" ${rotationString({
    x,
    y,
    size,
    rotation
  })}></path>`
}

export const distRoundedDot = ({
  x,
  y,
  size,
  rotation = 0,
  color
}: {
  x: number
  y: number
  size: number
  rotation?: number
  color: string
}) => {
  const dotSize = size / 8

  const d =
    `M ${x} ${y + 2.5 * dotSize}` +
    `v ${2 * dotSize}` +
    `a ${2.5 * dotSize} ${2.5 * dotSize}, 0, 0, 0, ${dotSize * 2.5} ${dotSize * 2.5}` +
    `h ${2 * dotSize}` +
    `a ${2.5 * dotSize} ${2.5 * dotSize}, 0, 0, 0, ${dotSize * 2.5} ${-dotSize * 2.5}` +
    `v ${-2 * dotSize}` +
    `a ${2.5 * dotSize} ${2.5 * dotSize}, 0, 0, 0, ${-dotSize * 2.5} ${-dotSize * 2.5}` +
    `h ${-2 * dotSize}` +
    `a ${2.5 * dotSize} ${2.5 * dotSize}, 0, 0, 0, ${-dotSize * 2.5} ${dotSize * 2.5}`

  return `<path clip-rule="evenodd" fill="${color}" d="${d}" ${rotationString({
    x,
    y,
    size,
    rotation
  })}></path>`
}

export const centerImage = ({
  qrWidth,
  qrHeight,
  width,
  height,
  count,
  dotSize,
  image = '',
  imageMargin
}: {
  qrWidth: number
  qrHeight: number
  width: number
  height: number
  count: number
  dotSize: number
  image?: string
  imageMargin: number
}) => {
  const xBeginning = (qrWidth - count * dotSize) / 2
  const yBeginning = (qrHeight - count * dotSize) / 2
  const dx = xBeginning + imageMargin + (count * dotSize - width) / 2
  const dy = yBeginning + imageMargin + (count * dotSize - height) / 2
  const dw = width - imageMargin * 2
  const dh = height - imageMargin * 2

  return /^<svg/.test(image)
    ? `<g transform="translate(${dx} ${dy}) scale(${(dw/qrWidth).toFixed(4)} ${(dh/qrHeight).toFixed(4)})">${image}</g>`
    : `<image href="${image}" x="${dx}" y="${dy}" width="${dw}px" height="${dh}px"></image>`
}

export const background = ({
  color,
  x,
  y,
  height,
  width
}: {
  color?: string
  x: number
  y: number
  height: number
  width: number
}) => {
  const clipPath = `url('#clip-path-bg')`
  const colorString = color ? `fill="${color}"` : ''
  return `<rect clip-path="${clipPath}" ${colorString} x="${x}" y="${y}" width="${width}" height="${height}"></rect>`
}

const squareMask = [
  [1, 1, 1, 1, 1, 1, 1],
  [1, 0, 0, 0, 0, 0, 1],
  [1, 0, 0, 0, 0, 0, 1],
  [1, 0, 0, 0, 0, 0, 1],
  [1, 0, 0, 0, 0, 0, 1],
  [1, 0, 0, 0, 0, 0, 1],
  [1, 1, 1, 1, 1, 1, 1]
]

const dotMask = [
  [0, 0, 0, 0, 0, 0, 0],
  [0, 0, 0, 0, 0, 0, 0],
  [0, 0, 1, 1, 1, 0, 0],
  [0, 0, 1, 1, 1, 0, 0],
  [0, 0, 1, 1, 1, 0, 0],
  [0, 0, 0, 0, 0, 0, 0],
  [0, 0, 0, 0, 0, 0, 0]
]

const dotsFilter =
  (hideBackgroundDots: boolean, count: number, drawImageSize: ImageSizeResult) =>
  (i: number, j: number): boolean => {
    if (hideBackgroundDots) {
      if (
        i >= (count - drawImageSize.hideXDots) / 2 &&
        i < (count + drawImageSize.hideXDots) / 2 &&
        j >= (count - drawImageSize.hideYDots) / 2 &&
        j < (count + drawImageSize.hideYDots) / 2
      ) {
        return false
      }
    }

    if (squareMask[i]?.[j] || squareMask[i - count + 7]?.[j] || squareMask[i]?.[j - count + 7]) {
      return false
    }

    if (dotMask[i]?.[j] || dotMask[i - count + 7]?.[j] || dotMask[i]?.[j - count + 7]) {
      return false
    }

    return true
  }

export const drawDots = (
  qr: QRCode,
  width: number,
  height: number,
  margin: number,
  color: string,
  hideBackgroundDots: boolean,
  count: number,
  drawImageSize: ImageSizeResult
) => {
  const minSize = Math.min(width, height) - margin * 2
  const dotSize = minSize / count
  const xBeginning = (width - count * dotSize) / 2
  const yBeginning = (height - count * dotSize) / 2

  const filter = dotsFilter(hideBackgroundDots, count, drawImageSize)
  const dots: string[] = []

  for (let i = 0; i < count; i++) {
    for (let j = 0; j < count; j++) {
      if (!filter(i, j)) {
        continue
      }
      if (!qr.isDark(i, j)) {
        continue
      }

      dots.push(
        distRoundedDot({
          x: xBeginning + i * dotSize,
          y: yBeginning + j * dotSize,
          size: dotSize,
          color
        })
      )
    }
  }

  return dots
}

export const drawCorners = (
  width: number,
  height: number,
  margin: number,
  squareColor: string,
  dotColor: string,
  count: number
) => {
  const paths: string[] = []

  const minSize = Math.min(width, height) - margin * 2
  const dotSize = minSize / count
  const cornersSquareSize = dotSize * 7
  const cornersDotSize = dotSize * 3
  const xBeginning = (width - count * dotSize) / 2
  const yBeginning = (height - count * dotSize) / 2

  ;[
    [0, 0, 0],
    [1, 0, Math.PI / 2],
    [0, 1, -Math.PI / 2]
  ].forEach(([column, row, rotation]) => {
    const x = xBeginning + column * dotSize * (count - 7)
    const y = yBeginning + row * dotSize * (count - 7)

    paths.push(
      tearCornerSquare({
        x,
        y,
        size: cornersSquareSize,
        rotation,
        color: squareColor
      })
    )

    paths.push(
      tearCornerDot({
        x: x + dotSize * 2,
        y: y + dotSize * 2,
        size: cornersDotSize,
        rotation,
        color: dotColor
      })
    )
  })

  return paths
}
