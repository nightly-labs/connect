import { createSignal, onMount, Show } from 'solid-js'
import { Title } from 'solid-start'
import { NCSolanaSelector } from '@nightlylabs/wallet-selector-solana'
import { StandardWalletAdapter } from '@solana/wallet-standard'
import { Connection, PublicKey, SystemProgram, Transaction as SolanaTx } from '@solana/web3.js'
import toast from 'solid-toast'

let selector: NCSolanaSelector

const connection = new Connection('https://api.devnet.solana.com')

export default function Solana() {
  const [adapter, setAdapter] = createSignal<StandardWalletAdapter>()
  onMount(async () => {
    if (selector) {
      return
    }
    selector = await NCSolanaSelector.build(
      {
        appMetadata: {
          name: 'NCTestSolana',
          description: 'Nightly Connect Test',
          icon: 'https://docs.nightly.app/img/logo.png',
          additionalInfo: 'Courtesy of Nightly Connect team'
        },
        url: 'https://nc2.nightly.app'
      },
      (newAdapter) => {
        setAdapter(newAdapter)
      },
      document.getElementById('modalAnchor') ?? undefined
    )
  })
  return (
    <main>
      <Title>Solana Example</Title>
      <div id="modalAnchor" />
      <Show
        when={!!adapter() && adapter()?.publicKey !== null}
        fallback={
          <button
            onClick={() => {
              selector.openModal()
            }}>
            Connect
          </button>
        }>
        <h1>Current public key: {adapter()?.publicKey?.toString()}</h1>
        <button
          onClick={async () => {
            try {
              const ix = SystemProgram.transfer({
                fromPubkey: adapter()!.publicKey!,
                lamports: 1e6,
                toPubkey: new PublicKey('147oKbjwGDHEthw7sRKNrzYiRiGqYksk1ravTMFkpAnv')
              })
              const tx = new SolanaTx().add(ix).add(ix).add(ix).add(ix).add(ix)
              const a = await connection.getRecentBlockhash()
              tx.recentBlockhash = a.blockhash
              tx.feePayer = adapter()!.publicKey!
              const signedTx = await adapter()!.signTransaction!(tx)
              await connection.sendRawTransaction(signedTx!.serialize())

              toast.success('Transaction was signed and sent!')
            } catch (e) {
              toast.error("Error: couldn't sign and send transaction!")
              console.log(e)
            }
          }}>
          Send 0.005 SOL
        </button>
        <button
          onClick={async () => {
            try {
              await adapter()!.signMessage!(new TextEncoder().encode('I love Nightly'))

              toast.success('Message was signed!')
            } catch (e) {
              toast.error("Error: couldn't sign message!")
              console.log(e)
            }
          }}>
          Sign message
        </button>
        <button
          onClick={() => {
            adapter()?.disconnect()
            setAdapter(undefined)
          }}>
          Disconnect
        </button>
      </Show>
    </main>
  )
}
