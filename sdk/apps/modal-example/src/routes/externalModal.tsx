import { createEffect, createSignal, onMount, Show } from 'solid-js'
import { Title } from 'solid-start'
import { NightlyConnectAdapter } from '@nightlylabs/wallet-selector-solana'
import { Connection, PublicKey, SystemProgram, Transaction as SolanaTx } from '@solana/web3.js'
import toast from 'solid-toast'
import { AppInitData, NightlyConnectSelectorModal } from '@nightlylabs/wallet-selector-base'
import { SOLANA_NETWORK } from '@nightlylabs/nightly-connect-solana'

const connection = new Connection('https://api.devnet.solana.com')

export default function SolanaExternalModal() {
  const [adapter, setAdapter] = createSignal<NightlyConnectAdapter>()
  const [modal, setModal] = createSignal<NightlyConnectSelectorModal>()
  const [eager, setEager] = createSignal(false)
  const [publicKey, setPublicKey] = createSignal<PublicKey>()
  onMount(async () => {
    const appInitData: AppInitData = {
      appMetadata: {
        name: 'NCTestSolana',
        description: 'Nightly Connect Test',
        icon: 'https://docs.nightly.app/img/logo.png',
        additionalInfo: 'Courtesy of Nightly Connect team'
      },
      persistent: true
    }

    const adapter = await NightlyConnectAdapter.build(
      appInitData,
      { disableModal: true },
      document.getElementById('modalAnchor')
    )

    const modal = new NightlyConnectSelectorModal(
      adapter.walletsList,
      appInitData.url ?? 'https://nc2.nightly.app',
      {
        name: SOLANA_NETWORK,
        icon: 'https://assets.coingecko.com/coins/images/4128/small/solana.png'
      },
      document.getElementById('modalAnchor')
    )

    setModal(modal)

    adapter.on('connect', (pk) => {
      modal.closeModal()
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
      <Title>Solana with External Modal Example</Title>
      <div id="modalAnchor" />
      <Show
        when={!!publicKey()}
        fallback={
          <button
            onClick={() => {
              if (adapter()?.connecting) {
                console.log('Cannot connect while connecting')
                return
              }

              if (adapter()?.connected) {
                return
              }

              modal()?.openModal(adapter()?.sessionId ?? undefined, async (walletName) => {
                try {
                  modal()?.setStandardWalletConnectProgress(true)
                  await adapter()?.connectToWallet(walletName)
                } catch (err) {
                  modal()?.setStandardWalletConnectProgress(false)
                  console.log('error')
                  modal()?.closeModal()
                }
              })
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
