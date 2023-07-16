import { triggerDeeplink } from '@nightlylabs/nightly-connect-base'

export const isMobileBrowser = () =>
  /Android|iPhone|iPad|iPod|Opera Mini/i.test(navigator.userAgent)

export const triggerConnect = (path: string, sessionId: string, relay: string) => {
  triggerDeeplink({
    path,
    deeplinkParams: { sessionId, relay }
  })
}

export const sleep = (timeout: number) =>
  new Promise<void>((resolve) => {
    setTimeout(() => {
      resolve()
    }, timeout)
  })
