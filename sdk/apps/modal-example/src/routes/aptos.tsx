import { createEffect, createSignal, onMount, Show } from 'solid-js'
import { Title } from 'solid-start'
import { NightlyConnectAptosAdapter } from '@nightlylabs/wallet-selector-aptos'
import toast from 'solid-toast'
import { AptosClient } from 'aptos'
// Create an AptosClient to interact with devnet.
const client = new AptosClient('https://fullnode.devnet.aptoslabs.com/v1')
export default function Aptos() {
  const [adapter, setAdapter] = createSignal<NightlyConnectAptosAdapter>()
  const [eager, setEager] = createSignal(false)
  const [publicKey, setPublicKey] = createSignal<string>()
  onMount(async () => {
    NightlyConnectAptosAdapter.build(
      {
        appMetadata: {
          name: 'NCTestAptos',
          description: 'Nightly Connect Test',
          icon: 'https://docs.nightly.app/img/logo.png',
          additionalInfo: 'Courtesy of Nightly Connect team'
        }
      },
      true,
      document.getElementById('modalAnchor')
    ).then(async (adapter) => {
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
          async (a) => {
            console.log(a)

            // const accounts = await adapter()!.account()
            // console.log(accounts)

            setPublicKey(a.address)
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
      <Title>Aptos Example</Title>
      <div id="modalAnchor" />
      <Show
        when={!!publicKey()}
        fallback={
          <button
            onClick={() => {
              adapter()
                ?.connect()
                .then(
                  async (a) => {
                    // const accounts = await adapter()!.account()
                    // console.log(accounts)
                    setPublicKey(a.address)
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
        <h1>Current address: {publicKey()}</h1>
        <button
          onClick={async () => {
            try {
              const tx = {
                type: 'entry_function_payload',
                arguments: [
                  '0x66fe4f72f8306d0e463fbbc51e400160effc9818f2ca08426229b06f1c4e8942',
                  1000
                ],
                function: '0x1::coin::transfer',
                type_arguments: ['0x1::aptos_coin::AptosCoin']
              }
              const a = await adapter()!.signAndSubmitTransaction(tx)

              console.log(a)
              toast.success('Transaction was signed and sent!')
            } catch (e) {
              toast.error("Error: couldn't sign and send transaction!")
              console.log(e)
            }
          }}>
          Send 0.005 Aptos
        </button>
        <button
          onClick={async () => {
            try {
              const accounts = await adapter()!.account()
              await adapter()!.signMessage!({
                message: 'Hello world!',
                nonce: '121'
              })

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
            setPublicKey(undefined)
          }}>
          Disconnect
        </button>
      </Show>
    </main>
  )
}
