import { NightlyConnectAdapter } from '@nightlylabs/wallet-selector-polkadot'
import { Show, createEffect, createSignal, onMount } from 'solid-js'
import toast from 'solid-toast'
import { MainPage } from '~/components/MainPage/MainPage'
import { ResolvePage } from '~/components/ResolvePage/ResolvePage'
import { getAdapter } from '~/store/adapter'
import {
  getFirstAllWinner,
  getFirstThreeWinner,
  getRandomWinner,
  getUserTickets,
  setFirstAllWinner,
  setFirstThreeWinner
} from '~/store/dbClient'
import { TICKETS_MAP } from '~/store/ticketsMap'
import { timeLeft } from '~/store/timer'

export default function Polkadot() {
  const [adapter, setAdapter] = createSignal<NightlyConnectAdapter>()
  const [eager, setEager] = createSignal(false)
  const [publicKey, setPublicKey] = createSignal<string>()
  const [loaded, setLoaded] = createSignal(false)
  const [user, setUser] = createSignal({ address: '', tickets: {}, loaded: false })
  const [isWinner, setIsWinner] = createSignal(false)
  onMount(async () => {
    try {
      const _adapter = await getAdapter()
      setAdapter(_adapter)
      if (await _adapter.canEagerConnect()) {
        setEager(true)
      }
      setLoaded(true)
    } catch {
      toast.error('Failed to connect please restart page')
    }
  })
  createEffect(() => {
    if (eager()) {
      adapter()
        ?.connect()
        .then(
          async () => {
            const accounts = await adapter()!.accounts.get()
            setPublicKey(accounts[0].address)
            toast.success('Wallet connected')
          },
          () => {
            toast.error('Connect rejected')
          }
        )
    }
  })
  createEffect(async () => {
    if (publicKey()) {
      const tickets = await getUserTickets(publicKey()!)
      setUser({ address: publicKey()!, tickets, loaded: true })
    }
  })
  const tableData = Object.entries(TICKETS_MAP)
  const matrix: Array<Array<[string, number]>> = [[], [], []]

  for (let i = 0; i < tableData.length; i++) {
    const row = Math.floor(i / 3)
    matrix[row].push(tableData[i])
  }
  createEffect(() => {
    if (user().loaded) {
      if (Object.values(user().tickets).length === 9) {
        toast.success('You have collected all tickets')
        setFirstAllWinner(publicKey()!)
      }
      if (Object.values(user().tickets).length >= 3) {
        setFirstThreeWinner(publicKey()!)
      }
    }
  })
  createEffect(async () => {
    if (user().loaded) {
      const firstWinner = await getFirstAllWinner()
      const firstThreeWinner = await getFirstThreeWinner()
      const randomWinner = await getRandomWinner()
      const allWinners = new Set([...firstWinner, ...firstThreeWinner, ...randomWinner])
      if (allWinners.has(publicKey()!)) {
        setIsWinner(true)
      }
    }
  })
  return (
    <Show when={timeLeft() !== 0} fallback={<ResolvePage resolve={true} />}>
      //
      <MainPage
        connected={user().loaded}
        onConnect={async () => {
          await adapter()!.connect()
          const accounts = await adapter()!.accounts.get()
          setPublicKey(accounts[0].address)
        }}
        counter={Object.values(user().tickets).length.toString()}
        id={Object.values(user().tickets)}
        time={timeLeft()}></MainPage>
    </Show>
  )
}
