import { WalletSelectorItem } from './types'

export type Breakpoint = 'xs' | 'sm' | 'lg'

export function getBreakpointFromWidth(screenWidth: number): Breakpoint {
  if (screenWidth < 374) {
    return 'xs'
  } else if (screenWidth <= 485) {
    return 'sm'
  } else {
    return 'lg'
  }
}

export function getBreakpointFromWidthInConnectWallet(screenWidth: number): Breakpoint {
  if (screenWidth < 560) {
    return 'xs'
  } else {
    return 'sm'
  }
}

export function getBreakpointFromWidthInMainPage(screenWidth: number): Breakpoint {
  if (screenWidth < 640) {
    return 'xs'
  } else {
    return 'sm'
  }
}

export function getNumberOfItems(breakpoint: Breakpoint) {
  switch (breakpoint) {
    case 'xs':
      return 2
    case 'sm':
      return 3
    case 'lg':
    default:
      return 4
  }
}

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
