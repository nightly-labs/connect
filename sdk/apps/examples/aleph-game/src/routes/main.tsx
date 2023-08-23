import { NightlyConnectAdapter } from '@nightlylabs/wallet-selector-polkadot'
import { createEffect, createSignal, onMount, Show } from 'solid-js'
import { Title } from 'solid-start'
import toast from 'solid-toast'
import { LandingPage } from '~/components/LandingPage/LandingPage'
import { MainPage } from '~/components/MainPage/MainPage'
import { getAdapter } from '~/store/adapter'
import { getUserTickets } from '~/store/dbClient'
import { TICKETS_MAP } from '~/store/ticketsMap'

export default function Polkadot() {
  const [adapter, setAdapter] = createSignal<NightlyConnectAdapter>()
  const [eager, setEager] = createSignal(false)
  const [publicKey, setPublicKey] = createSignal<string>()
  const [loaded, setLoaded] = createSignal(false)
  const [user, setUser] = createSignal({ address: '', tickets: {}, loaded: false })

  onMount(async () => {
    const _adapter = await getAdapter()
    setAdapter(_adapter)
    if (await _adapter.canEagerConnect()) {
      setEager(true)
    }
    setLoaded(true)
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
      console.log(user())
    }
  })
  const tableData = Object.entries(TICKETS_MAP)
  const matrix: Array<Array<[string, number]>> = [[], [], []]

  for (let i = 0; i < tableData.length; i++) {
    const row = Math.floor(i / 3)
    matrix[row].push(tableData[i])
  }
  console.log(user())
  return (
    <MainPage
      collectedTicket={true}
      counter={Object.values(user().tickets).length.toString()}
      id={Object.values(user().tickets)}
      time={9238974312734}></MainPage>
  )
}
