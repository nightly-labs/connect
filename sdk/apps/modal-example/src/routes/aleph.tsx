import { getPolkadotWallets, NightlyConnectAdapter } from '@nightlylabs/wallet-selector-polkadot'
import { ApiPromise, WsProvider } from '@polkadot/api'
import { createEffect, createSignal, onMount, Show } from 'solid-js'
import { Title } from '@solidjs/meta'
import toast from 'solid-toast'
import { Signer } from '@polkadot/api/types'

const RECEIVER = '5CFRopxy991HCJj1HYtUQjaaBMw9iRLE9jxPndBsgdCjeJj5'

export default function Polkadot() {
  const [adapter, setAdapter] = createSignal<NightlyConnectAdapter>()
  const [eager, setEager] = createSignal(false)
  const [publicKey, setPublicKey] = createSignal<string>()
  const [api, setApi] = createSignal<ApiPromise>()
  const provider = new WsProvider('wss://ws.test.azero.dev/')

  onMount(() => {
    NightlyConnectAdapter.build(
      {
        appMetadata: {
          name: 'NC TEST AlephZero',
          description: 'Nightly Connect Test',
          icon: 'https://docs.nightly.app/img/logo.png',
          additionalInfo: 'Courtesy of Nightly Connect team'
        },
        network: 'AlephZero'
      },
      {},
      document.getElementById('modalAnchor')
    ).then((adapter) => {
      adapter.canEagerConnect().then((canEagerConnect) => {
        setEager(canEagerConnect)
      })

      adapter.on('disconnect', () => {
        setPublicKey(undefined)
      })

      setAdapter(adapter)

      ApiPromise.create({
        provider
      }).then((api) => {
        setApi(api)
      })
    })
  })

  createEffect(() => {
    if (eager()) {
      adapter()
        ?.connect()
        .then(
          async () => {
            const accounts = await adapter()!.accounts.get()
            console.log(accounts)
            setPublicKey(accounts[0].address)
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
      <Title>Aleph Zero Example</Title>
      <div id="modalAnchor" />
      <Show
        when={!!publicKey()}
        fallback={
          <button
            onClick={async () => {
              try {
                console.log(getPolkadotWallets())

                await adapter()!.connect()
                const accounts = await adapter()!.accounts.get()
                console.log(accounts)
                setPublicKey(accounts[0].address)
                console.log('adapter', adapter())
              } catch (err) {
                console.log(err)
              }
            }}>
            Connect
          </button>
        }>
        <h1>Current address: {publicKey()}</h1>
        <button
          onClick={async () => {
            try {
              const payload = api()!.tx.balances.transfer(RECEIVER, 5000000000000)
              const signed = await payload.signAsync(publicKey()!, {
                signer: adapter()!.signer as Signer
              })
              console.log({ signed })
              await signed.send()
              toast.success('Transaction was signed and sent!')
            } catch (e) {
              toast.error("Error: couldn't sign and send transaction!")
              console.log(e)
            }
          }}>
          Sign test transfer
        </button>
        <button
          onClick={async () => {
            try {
              const message = 'I love Nightly 🦊'
              const _signed = await adapter()!.signer.signRaw!({
                address: publicKey()!,
                data: message,
                type: 'bytes'
              })

              toast.success('Message was signed!')
            } catch (e) {
              toast.error("Error: couldn't sign message!")
              console.log(e)
            }
          }}>
          Sign message
        </button>
      </Show>
      <button
        onClick={() => {
          adapter()?.disconnect()
          setPublicKey(undefined)
        }}>
        Disconnect
      </button>
    </main>
  )
}
