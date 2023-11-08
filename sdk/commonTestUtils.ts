export function sleep(ms: number) {
  return new Promise((resolve) => setTimeout(resolve, ms))
}
export const TEST_RELAY_ENDPOINT = process.env.PRODUCTION
  ? 'https://nc2.nightly.app'
  : 'http://127.0.0.1:6969'

export const smartDelay = async (ms?: number) => {
  if (process.env.PRODUCTION) {
    await sleep(ms || 100)
  } else {
    if (process.env.IS_CI) {
      await sleep(ms || 100)
    } else {
      await sleep(ms || 5)
    }
  }
}
