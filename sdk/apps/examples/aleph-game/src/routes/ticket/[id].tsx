import { NightlyConnectAdapter } from '@nightlylabs/wallet-selector-polkadot'
import { Show, createEffect, createSignal, onMount } from 'solid-js'
import { useNavigate, useParams } from 'solid-start'
import toast from 'solid-toast'
import { LandingPage } from '../../components/LandingPage/LandingPage'
import { getAdapter } from '~/store/adapter'
import { addUserTicket, getUserTickets, setRandomWinner } from '~/store/dbClient'
import { TicketId, TICKETS_MAP } from '~/store/ticketsMap'
import { stringToU8a, u8aToHex } from '@polkadot/util'

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
      console.log(tickets)
      // @ts-expect-error ignore
      console.log(!!tickets[params.id])
      // @ts-expect-error ignore
      setIsTicketClaimed(!!tickets[params.id])
      setUser({ address: publicKey()!, tickets, loaded: true })
    }
  })
  createEffect(async () => {
    if (isTicketClaimed()) {
      setTimeout(() => {
        navigate('/main')
      }, 1000)
    }
  })
  return (
    <Show when={loaded()}>
      <LandingPage
        hasTicketsToClaim={!isTicketClaimed()}
        isConnected={user().loaded && loaded()}
        onAddTickets={async () => {
          try {
            const message = stringToU8a(ticketId)
            const signed = await adapter()!.signer!.signRaw!({
              address: publicKey()!,
              data: u8aToHex(message),
              type: 'bytes'
            })
            // verify the message using Alice's address
            // Ignore verification for now
            // const { isValid } = signatureVerify(message, signed.signature, publicKey()!)
            setIsTicketClaimed(true)
            await addUserTicket(publicKey()!, ticketId)
            toast.success('Transaction was signed and sent!')
            // tigger with probability of 5%
            if (Math.random() < 0.05) {
              toast.success('You won a random price!')
              await setRandomWinner(publicKey()!)
            }
          } catch (e) {
            toast.error("Error: couldn't sign and send transaction!")
            console.log(e)
          }
        }}
        onClaimTickets={async () => {
          navigate('/main')
        }}
        onConnectWallet={async () => {
          await adapter()!.connect()
          const accounts = await adapter()!.accounts.get()
          setPublicKey(accounts[0].address)
        }}
      />
    </Show>
  )
}
