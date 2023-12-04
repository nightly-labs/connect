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
          async () => {
            const accounts = await adapter()!.account()
            setPublicKey(accounts.address)
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
                  async () => {
                    const accounts = await adapter()!.account()
                    setPublicKey(accounts.address)
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
              // const transactionBlock = new TransactionBlock()
              // const coin = transactionBlock.splitCoins(transactionBlock.gas, [
              //   transactionBlock.pure(50000000)
              // ])
              // transactionBlock.transferObjects(
              //   [coin],
              //   transactionBlock.pure(
              //     '0xd85c7ad90905e0bd49b72420deb5f4077cab62840fb3917ca2945e41d8854013'
              //   )
              // )
              // const accounts = await adapter()!.account()
              // await adapter()!.signAndSubmitTransaction({
              //   transactionBlock,
              //   chain: 'sui:testnet',
              //   account: accounts
              // })

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
