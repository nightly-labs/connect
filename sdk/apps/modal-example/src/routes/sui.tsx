import { createEffect, createSignal, onMount, Show } from 'solid-js'
import { Title } from '@solidjs/meta'
import { NightlyConnectSuiAdapter } from '@nightlylabs/wallet-selector-sui'
import { Transaction } from '@mysten/sui/transactions'
import toast from 'solid-toast'

export default function Sui() {
  const [adapter, setAdapter] = createSignal<NightlyConnectSuiAdapter>()
  const [eager, setEager] = createSignal(false)
  const [publicKey, setPublicKey] = createSignal<string>()
  onMount(() => {
    NightlyConnectSuiAdapter.build(
      {
        appMetadata: {
          name: 'NCTestSui',
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
      <Title>Sui Example</Title>
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
                  '0xd85c7ad90905e0bd49b72420deb5f4077cab62840fb3917ca2945e41d8854013'
                )
              )
              const accounts = await adapter()!.getAccounts()
              await adapter()!.signAndExecuteTransactionBlock({
                transactionBlock,
                chain: 'sui:testnet',
                account: accounts[0]
              })

              toast.success('Transaction was signed and sent!')
            } catch (e) {
              toast.error("Error: couldn't sign and send transaction!")
              console.log(e)
            }
          }}>
          Send 0.05 SUI
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
