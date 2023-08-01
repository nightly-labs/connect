import { createEffect, createSignal, onMount, Show } from 'solid-js'
import { Title } from 'solid-start'
import { NightlyConnectAdapter } from '@nightlylabs/wallet-selector-solana'
import { Connection, PublicKey, SystemProgram, Transaction as SolanaTx } from '@solana/web3.js'
import toast from 'solid-toast'

const connection = new Connection('https://api.devnet.solana.com')

export default function Solana() {
  const [adapter, setAdapter] = createSignal<NightlyConnectAdapter>()
  const [eager, setEager] = createSignal(false)
  const [publicKey, setPublicKey] = createSignal<PublicKey>()
  onMount(async () => {
    NightlyConnectAdapter.build(
      {
        appMetadata: {
          name: 'NCTestSolana',
          description: 'Nightly Connect Test',
          icon: 'https://docs.nightly.app/img/logo.png',
          additionalInfo: 'Courtesy of Nightly Connect team'
        }
      },
      true,
      document.getElementById('modalAnchor')
    ).then((adapter) => {
      adapter.on('connect', (pk) => {
        setPublicKey(pk)
      })

      adapter.on('disconnect', () => {
        setPublicKey(undefined)
      })

      adapter.canEagerConnect().then((canEagerConnect) => {
        setEager(canEagerConnect)
      })

      setAdapter(adapter)
    })
  })

  createEffect(() => {
    if (eager()) {
      adapter()
        ?.connect()
        .then(
          () => {
            console.log('connect resolved successfully')
          },
          () => {
            console.log('connect rejected')
          }
        )
    }
  })

  return (
    <main>
      <Title>Solana Example</Title>
      <div id="modalAnchor" />
      <Show
        when={!!publicKey()}
        fallback={
          <button
            onClick={() => {
              adapter()
                ?.connect()
                .then(
                  () => {
                    console.log('connect resolved successfully')
                  },
                  () => {
                    console.log('connect rejected')
                  }
                )
            }}>
            Connect
          </button>
        }>
        <h1>Current public key: {publicKey()!.toString()}</h1>
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
          }}>
          Disconnect
        </button>
      </Show>
    </main>
  )
}
