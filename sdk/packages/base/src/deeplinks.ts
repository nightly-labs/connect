export const PREFIX = 'nc'

export interface DeeplinkParams {
  sessionId: string
  requestId?: string // if deeplink does not have requestId, it is a connect deeplink
  relay: string
}

// export const exampleDeeplink = `https://nightly.app/nc?sessionId=0x123&requestId=0x456&relay=https://nc2.nightly.app`
export interface TriggerDeeplink {
  path: string // https://nightly.app or nightly
  deeplinkParams: DeeplinkParams
}
export const createDeeplinkUrl = ({ deeplinkParams, path }: TriggerDeeplink) => {
  const { sessionId, requestId, relay } = deeplinkParams
  let entry = ''
  if (path.startsWith('https://')) {
    entry = path
  } else {
    entry = path + ':/'
  }
  let url = `${entry}/${PREFIX}?sessionId=${sessionId}&relay=${relay}`
  if (requestId) {
    url = url + `&requestId=${requestId}`
  }
  return url
}
export const triggerDeeplink = ({ deeplinkParams, path }: TriggerDeeplink) => {
  const url = createDeeplinkUrl({ deeplinkParams, path })
  if (window) {
    if (url.startsWith('https://')) {
      window.open(url, '_blank', 'noreferrer noopener')
    } else {
      window.open(url, '_self', 'noreferrer noopener')
    }
  } else {
    console.warn('window is undefined')
  }
}
export const parseDeeplink = (url: string): DeeplinkParams => {
  const urlObj = new URL(url)
  const sessionId = urlObj.searchParams.get('sessionId')
  const requestId = urlObj.searchParams.get('requestId')
  const relay = urlObj.searchParams.get('relay')
  if (sessionId && relay) {
    return {
      sessionId,
      requestId: requestId ?? undefined,
      relay
    }
  }
  throw new Error('Invalid deeplink')
}
