import { createEffect, createMemo, createSignal } from 'solid-js'

export const END_TIME = 1699893952 //timestamp
// const currentTime = () => Math.floor(Date.now() / 1000);
const [currentTime, setCurrentTime] = createSignal(Math.floor(Date.now() / 1000))
createEffect(() => {
  const interval = setInterval(() => {
    setCurrentTime(Math.floor(Date.now() / 1000))
  }, 1000)
  return () => {
    clearInterval(interval)
  }
})

export const timeLeft = createMemo(() => {
  const time = END_TIME - currentTime()
  return time > 0 ? time : 0
})
