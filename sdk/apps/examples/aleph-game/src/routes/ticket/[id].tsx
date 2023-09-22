import { createEffect, createSignal } from 'solid-js'
import { useNavigate, useParams } from 'solid-start'
import toast from 'solid-toast'
import { LandingPage } from '../../components/LandingPage/LandingPage'

import { addUserTicket, getUserTickets } from '~/store/dbClient'
import { TicketId, TICKETS_MAP } from '~/store/ticketsMap'
import { START_TIME } from '~/store/timer'

export default function Polkadot() {
  const [accountData, setAccountData] = createSignal<{ publicKey: string; accountId: string }>()
  const [isTicketClaimed, setIsTicketClaimed] = createSignal(false)
  const [user, setUser] = createSignal({ address: '', tickets: {}, loaded: false })
  const params = useParams<{ id: string }>()
  const navigate = useNavigate()
  // @ts-expect-error ignore
  if (!TICKETS_MAP[params.id]) {
    navigate('/near')
  }

  // const actualDate = new Date().getTime() / 1000
  // if (actualDate < START_TIME) {
  //   navigate('/chilling')
  // }

  const ticketId = params.id as TicketId

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
    })
  })

  createEffect(async () => {
    if (accountData()) {
      const tickets = await getUserTickets(accountData()?.accountId ?? '')
      console.log(tickets)
      // @ts-expect-error ignore
      console.log(!!tickets[params.id])
      // @ts-expect-error ignore
      setIsTicketClaimed(!!tickets[params.id])
      setUser({ address: accountData()?.accountId ?? '', tickets, loaded: true })
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
    <LandingPage
      hasTicketsToClaim={!isTicketClaimed()}
      isConnected={user().loaded}
      onAddTickets={async () => {
        try {
          const message = ticketId
          const nonce = Buffer.from(Array.from(Array(32).keys()))
          //  @ts-expect-error ignore
          const signed = await window?.nightly?.near.signMessage({
            message,
            recipient: accountData()?.accountId ?? '',
            nonce
          })

          // Ignore verification for now
          // const { isValid } = signatureVerify(message, signed.signature, publicKey()!)
          setIsTicketClaimed(true)
          await addUserTicket(accountData()?.accountId ?? '', ticketId)
          toast.success('Transaction was signed and sent!')
          // tigger with probability of 5%
          // if (Math.random() < 0.02) {
          //   toast.success('You won a random price!')
          //   await setRandomWinner(accountData()!)
          // }
        } catch (e) {
          toast.error("Error: couldn't sign and send transaction!")
          console.log(e)
        }
      }}
      onClaimTickets={async () => {
        navigate('/main')
      }}
      onConnectWallet={async () => {
        await connectWallet()
      }}
    />
  )
}
