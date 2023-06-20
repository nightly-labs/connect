import { createSignal, onMount, Show } from 'solid-js'
import { Title } from 'solid-start'
import { NCSolanaSelector } from '@nightlylabs/wallet-selector-solana'
import { StandardWalletAdapter } from '@solana/wallet-standard'
import { Connection, PublicKey, SystemProgram, Transaction as SolanaTx } from '@solana/web3.js'

let selector: NCSolanaSelector

const connection = new Connection('https://api.devnet.solana.com')

export default function Solana() {
  const [adapter, setAdapter] = createSignal<StandardWalletAdapter>()
  onMount(async () => {
    if (selector) {
      return
    }
    selector = await NCSolanaSelector.build({
        appMetadata: {
          name: 'NCTest',
          description: 'Nightly Connect Test',
          icon: 'https://docs.nightly.app/img/logo.png',
          additionalInfo: 'Courtesy of Nightly Connect team'
        },
        url: 'https://nc2.nightly.app'
      }
    )
    selector.onConnected = (newAdapter) => {
      setAdapter(newAdapter)
    }
  })
  return (
    <main>
      <Title>Solana Example</Title>
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

              window.alert('Transaction was signed and sent!')
            } catch (e) {
              console.log(e)
            }
          }}>
          Send 0.005 SOL
        </button>
        <button
          onClick={() => {
            adapter()?.disconnect()
          }}>
          Disconnect
        </button>
      </Show>
    </main>
  )
}
