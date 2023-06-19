import { createSignal, onMount } from 'solid-js'
import { Title } from 'solid-start'
import { NCSolanaSelector } from '@nightlylabs/wallet-selector-solana'
import { StandardWalletAdapter } from '@solana/wallet-standard'
import { Connection, PublicKey, SystemProgram, Transaction as SolanaTx } from '@solana/web3.js'

let selector: NCSolanaSelector

const connection = new Connection('https://api.devnet.solana.com')

export default function Home() {
  const [adapter, setAdapter] = createSignal<StandardWalletAdapter>()
  onMount(async () => {
    if (selector) {
      return
    }
    selector = await NCSolanaSelector.build({
      appInitData: {
        appMetadata: {
          name: 'Test application',
          description: 'If you see this message, you will be soon testing new Nightly Connect',
          icon: 'https://docs.nightly.app/img/logo.png',
          additionalInfo: 'Courtesy of Nightly Connect team'
        },
        url: 'https://nc2.nightly.app'
      }
    })
    selector.onSelectWallet = (newAdapter) => {
      newAdapter.connect().then(() => {
        setAdapter(newAdapter)
      })
    }
  })
  return (
    <main>
      <Title>Hello World</Title>
      <button
        onClick={() => {
          selector.openModal()
        }}>
        Connect
      </button>
      <button
        onClick={async () => {
          try {
            const adapt = adapter()

            if (!adapt || adapt.publicKey === null) {
              return
            }

            const ix = SystemProgram.transfer({
              fromPubkey: adapt.publicKey,
              lamports: 1_000_000,
              toPubkey: new PublicKey('147oKbjwGDHEthw7sRKNrzYiRiGqYksk1ravTMFkpAnv')
            })
            const tx = new SolanaTx().add(ix).add(ix).add(ix).add(ix).add(ix)
            const a = await connection.getRecentBlockhash()
            tx.recentBlockhash = a.blockhash
            tx.feePayer = adapt.publicKey
            const signedTx = await adapt.signTransaction!(tx)
            const id = await connection.sendRawTransaction(signedTx!.serialize())
            console.log(id)
            console.log(id)
          } catch (e) {
            console.log(e)
          }
        }}>
        Test send
      </button>
    </main>
  )
}
