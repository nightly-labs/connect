import { Mode, TypeNumber } from './types'

interface Modes {
  [key: string]: Mode
}

export const modes: Modes = {
  numeric: 'Numeric',
  alphanumeric: 'Alphanumeric',
  byte: 'Byte',
  kanji: 'Kanji'
}

interface TypesMap {
  [key: number]: TypeNumber
}

export const qrTypes: TypesMap = {}

for (let type = 0; type <= 40; type++) {
  qrTypes[type] = type as TypeNumber
}

interface ErrorCorrectionPercents {
  [key: string]: number
}

export const errorCorrectionPercents: ErrorCorrectionPercents = {
  L: 0.07,
  M: 0.15,
  Q: 0.25,
  H: 0.3
}
