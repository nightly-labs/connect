import { WalletSelectorItem } from './types'

export const walletsSort = (a: WalletSelectorItem, b: WalletSelectorItem) => {
  if (a.recent && b.recent) {
    return 0;
  }

  if (a.recent) {
    return -1
  }

  if (b.recent) {
    return 1
  }

  if (a.detected && b.detected) {
    return 0;
  }

  if (a.detected) {
    return -1
  }

  if (b.detected) {
    return 1
  }

  return 0
}
