import { getPolkadotWallets, NightlyConnectAdapter } from '@nightlylabs/wallet-selector-polkadot'
import { stringToU8a, u8aToHex } from '@polkadot/util'
import { signatureVerify } from '@polkadot/util-crypto'
import { createEffect, createSignal, onMount, Show } from 'solid-js'
import { useNavigate, useParams } from 'solid-start'
import toast from 'solid-toast'
import { getAdapter } from '~/store/adapter'
import { addUserTicket, getUserTickets } from '~/store/dbClient'
import { TicketId, TICKETS_MAP } from '~/store/ticketsMap'

export default function Polkadot() {
  const [adapter, setAdapter] = createSignal<NightlyConnectAdapter>()
  const [eager, setEager] = createSignal(false)
  const [publicKey, setPublicKey] = createSignal<string>()
  const [loaded, setLoaded] = createSignal(false)
  const [isTicketClaimed, setIsTicketClaimed] = createSignal(false)
  const [user, setUser] = createSignal({ address: '', tickets: {}, loaded: false })
  const params = useParams<{ id: string }>()
  const navigate = useNavigate()
  // @ts-expect-error ignore
  if (!TICKETS_MAP[params.id]) {
    navigate('/aleph')
  }
  const ticketId = params.id as TicketId
  onMount(async () => {
    const _adapter = await getAdapter()
    setAdapter(_adapter)
    if (await _adapter.canEagerConnect()) {
      setEager(true)
    }
    setLoaded(true)
  })
  createEffect(() => {
    if (adapter()) {
      setLoaded(true)
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
      // @ts-expect-error ignore
      setIsTicketClaimed(!!tickets[params.id])
    }
  })
  createEffect(async () => {
    if (isTicketClaimed()) {
      setTimeout(() => {
        navigate('/aleph')
      }, 300)
    }
  })
  return (
    <Show when={loaded()} fallback={<div>Loading...</div>}>
      <main>
        <Show
          when={!!publicKey()}
          fallback={
            <button
              onClick={async () => {
                console.log(getPolkadotWallets())
                await adapter()!.connect()
                const accounts = await adapter()!.accounts.get()
                console.log(accounts)
                setPublicKey(accounts[0].address)
                console.log('adapter', adapter())
              }}>
              Connect
            </button>
          }>
          <Show when={!isTicketClaimed()} fallback={<div>Ticket claimed</div>}>
            <button
              onClick={async () => {
                try {
                  const message = stringToU8a('this is our message')
                  const signed = await adapter()!.signer!.signRaw!({
                    address: publicKey()!,
                    data: u8aToHex(message),
                    type: 'bytes'
                  })
                  // verify the message using Alice's address
                  const { isValid } = signatureVerify(message, signed.signature, publicKey()!)
                  if (isValid) {
                    setIsTicketClaimed(true)
                    await addUserTicket(publicKey()!, ticketId)
                    toast.success('Transaction was signed and sent!')
                  } else {
                    toast.error('Invalid signature')
                  }
                } catch (e) {
                  toast.error("Error: couldn't sign and send transaction!")
                  console.log(e)
                }
              }}>
              Sign test transfer
            </button>
          </Show>
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
