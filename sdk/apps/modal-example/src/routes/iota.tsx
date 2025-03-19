import { createEffect, createSignal, onMount, Show } from 'solid-js'
import { Title } from '@solidjs/meta'
import { NightlyConnectIotaAdapter } from '@nightlylabs/wallet-selector-iota'
import { Transaction } from '@iota/iota-sdk/transactions'
import toast from 'solid-toast'

export default function Iota() {
  const [adapter, setAdapter] = createSignal<NightlyConnectIotaAdapter>()
  const [eager, setEager] = createSignal(false)
  const [publicKey, setPublicKey] = createSignal<string>()
  onMount(() => {
    NightlyConnectIotaAdapter.build(
      {
        appMetadata: {
          name: 'NCTestIOTA',
          description: 'Nightly Connect Test',
          icon: 'https://docs.nightly.app/img/logo.png',
          additionalInfo: 'Courtesy of Nightly Connect team'
        }
      },
      {},
      document.getElementById('modalAnchor')
    ).then((adapter) => {
      adapter.canEagerConnect().then((canEagerConnect) => {
        setEager(canEagerConnect)
      })

      adapter.on('connect', (accounts) => {
        setPublicKey(accounts[0].address)
      })

      adapter.on('disconnect', () => {
        setPublicKey(undefined)
        console.log('adapter disconnected')
      })

      adapter.on('change', (a) => {
        if (!!a.accounts?.length && a.accounts[0].address) {
          setPublicKey(a.accounts[0].address)
        }
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
      <Title>IOTA Example</Title>
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
        <h1>Current address: {publicKey()}</h1>
        <button
          onClick={async () => {
            try {
              const transactionBlock = new Transaction()
              const coin = transactionBlock.splitCoins(transactionBlock.gas, [
                transactionBlock.pure.u64(50000000)
              ])
              transactionBlock.transferObjects(
                [coin],
                transactionBlock.pure.address(
                  '0x62248df36a0f520bac63a54301079eb62b45c0c3374211a53fa0f57de5d8c415'
                )
              )
              const accounts = await adapter()!.getAccounts()
              await adapter()!.signAndExecuteTransaction({
                transaction: transactionBlock,
                chain: 'iota:testnet',
                account: accounts[0]
              })

              toast.success('Transaction was signed and sent!')
            } catch (e) {
              toast.error("Error: couldn't sign and send transaction!")
              console.log(e)
            }
          }}>
          Send 0.05 IOTA
        </button>
        <button
          onClick={async () => {
            try {
              const accounts = await adapter()!.getAccounts()
              await adapter()!.signPersonalMessage!({
                message: new TextEncoder().encode('I love Nightly'),
                account: accounts[0]
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
