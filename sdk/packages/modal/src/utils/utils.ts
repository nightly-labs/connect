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

export function getNumberOfItems(breakpoint: Breakpoint) {
  switch (breakpoint) {
    case 'xs':
      return 5
    case 'sm':
      return 7
    case 'lg':
    default:
      return 9
  }
}
