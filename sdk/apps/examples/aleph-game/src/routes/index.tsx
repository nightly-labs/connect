import { Show, createEffect, createSignal } from 'solid-js'
import toast from 'solid-toast'
import { MainPage } from '~/components/MainPage/MainPage'
import { ResolvePage } from '~/components/ResolvePage/ResolvePage'
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
  const [accountData, setAccountData] = createSignal<{ publicKey: string; accountId: string }>()
  const [user, setUser] = createSignal({ address: '', tickets: {}, loaded: false })
  const [isWinner, setIsWinner] = createSignal(false)

  const connectWallet = async () => {
    // @ts-expect-error ignore
    if (window?.nightly?.near) {
      // @ts-expect-error ignore
      await window.nightly.near.connect().then((res: any) => {
        console.log(res)
        setAccountData(res)
        toast.success('Wallet connected')
      })
    }
  }

  createEffect(() => {
    connectWallet().catch((err) => {
      console.log(err)
      toast.error('Connect rejected')
    })
  })
  createEffect(async () => {
    if (accountData()) {
      const tickets = await getUserTickets(accountData()?.accountId ?? '')
      setUser({ address: accountData()?.accountId ?? '', tickets, loaded: true })
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
        setFirstAllWinner(accountData()?.accountId ?? '')
      }
      if (Object.values(user().tickets).length >= 3) {
        setFirstThreeWinner(accountData()?.accountId ?? '')
      }
    }
  })
  createEffect(async () => {
    if (user().loaded) {
      const firstWinner = await getFirstAllWinner()
      const firstThreeWinner = await getFirstThreeWinner()
      const randomWinner = await getRandomWinner()
      const allWinners = new Set([...firstWinner, ...firstThreeWinner, ...randomWinner])
      if (allWinners.has(accountData()?.accountId ?? '')) {
        setIsWinner(true)
      }
    }
  })
  return (
    <Show when={timeLeft() !== 0} fallback={<ResolvePage resolve={isWinner()} />}>
      //
      <MainPage
        connected={user().loaded}
        onConnect={async () => {
          await connectWallet().catch((err) => {
            console.log(err)
            toast.error('Connect rejected')
          })
        }}
        counter={Object.values(user().tickets).length.toString()}
        id={Object.values(user().tickets)}
        time={timeLeft()}></MainPage>
    </Show>
  )
}
