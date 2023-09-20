import { WalletSelectorItem } from './types'

export const walletsSort = (a: WalletSelectorItem, b: WalletSelectorItem) => {
  if (a.recent) {
    return -1
  }

  if (b.recent) {
    return 1
  }

  if (a.detected) {
    return -1
  }

  if (b.detected) {
    return 1
  }

  return 0
}
