import { NightlyConnectAdapter } from '@nightlylabs/wallet-selector-polkadot'
import { createEffect, createSignal, onMount, Show } from 'solid-js'
import { Title } from 'solid-start'
import toast from 'solid-toast'
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

  return (
    <Show when={loaded()} fallback={<div>Loading...</div>}>
      <main>
        <Title>Aleph Zero Example</Title>
        <Show
          when={!!publicKey()}
          fallback={
            <button
              onClick={async () => {
                await adapter()!.connect()
                const accounts = await adapter()!.accounts.get()
                setPublicKey(accounts[0].address)
              }}>
              Connect
            </button>
          }>
          <h1>Current address: {publicKey()}</h1>
          <div>
            <table>
              <tbody>
                {matrix.map((row) => (
                  <tr>
                    {row.map(([key, value]) => {
                      // @ts-expect-error ignore
                      const isClaimed = !!user().tickets[key]
                      return (
                        <>
                          <td>Ticket id: {value} </td>
                          <td>{isClaimed.toString()}</td>
                        </>
                      )
                    })}
                  </tr>
                ))}
              </tbody>
            </table>
          </div>
        </Show>
        <button
          onClick={() => {
            adapter()?.disconnect()
            setPublicKey(undefined)
          }}>
          Disconnect
        </button>
      </main>
    </Show>
  )
}
