import { triggerDeeplink } from '@nightlylabs/nightly-connect-base'

export const modalStyle = `
  @import url('https://fonts.googleapis.com/css2?family=Prompt&display=swap');

  .nightlyConnectSelectorOverlay {
    position: fixed;
    top: 0;
    left: 0;
    width: 100vw;
    height: 100vh;
    background: rgba(15, 15, 26, 0.4);
    backdrop-filter: blur(10px);
    z-index: 100;
  }

  .nightlyConnectSelector {
    margin: auto;
    position: relative;
    top: 50%;
    transform: translateY(-50%);
    width: fit-content;
    display: block;
  }
`

export const isMobileBrowser = () =>
  /Android|iPhone|iPad|iPod|Opera Mini/i.test(navigator.userAgent)

export const triggerConnect = (path: string, sessionId: string, relay: string) => {
  triggerDeeplink({
    path,
    deeplinkParams: { sessionId, relay }
  })
}
