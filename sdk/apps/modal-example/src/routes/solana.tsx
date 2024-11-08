import { createEffect, createSignal, onMount, Show } from 'solid-js'
import { Title } from '@solidjs/meta'
import { NightlyConnectAdapter } from '@nightlylabs/wallet-selector-solana'
import { Connection, PublicKey, SystemProgram, Transaction as SolanaTx } from '@solana/web3.js'
import toast from 'solid-toast'
import { DropdownButton } from '~/components/DropdownButton'

const connection = new Connection('https://api.devnet.solana.com')

export interface INetwork {
  name: string
  genesisHash: string
  url: string | undefined
}
const CUSTOM_NETWORK: INetwork[] = [
  {
    name: 'Solana Mainnet',
    genesisHash: '5eykt4UsFv8P8NJdTREpY1vzqKqZKvdpKuc147dw2N9d',
    url: undefined
  },
  {
    name: 'Solana Testnet',
    genesisHash: '4uhcVJyU9pJkvQyS88uRDiswHXSCkY3zQawwpjk2NsNY',
    url: undefined
  },
  {
    name: 'Solana Devnet',
    genesisHash: 'EtWTRABZaYq6iMfeYKouRu166VU2xqa1wcaWoxPkrZBG',
    url: undefined
  },
  {
    name: 'Eclipse Mainnet',
    genesisHash: 'EAQLJCV2mh23BsK2P9oYpV5CHVLDNHTxYss3URrNmg3s',
    url: undefined
  },
  {
    name: 'Eclipse Testnet',
    genesisHash: 'CX4huckiV9QNAkKNVKi5Tj8nxzBive5kQimd94viMKsU',
    url: undefined
  },
  {
    name: 'Eclipse Devnet',
    genesisHash: '8axJLKAqQU9oyULRunGrZTLDEXhn17VWxoH5F7MCmdXG',
    url: undefined
  },
  {
    name: 'Sonic Testnet',
    genesisHash: 'Ep5wb4kbMk8yHqV4jMXNqDiMWnNtnTh8jX6WY59Y8Qvj',
    url: undefined
  },
  {
    name: 'Solana Mainnet with url',
    genesisHash: '5eykt4UsFv8P8NJdTREpY1vzqKqZKvdpKuc147dw2N9d',
    url: 'https://solana-mainnet.rpc.extrnode.com/85c27167-63a1-4fa3-9971-fc1df7b132dc'
  }
]

export default function Solana() {
  const [adapter, setAdapter] = createSignal<NightlyConnectAdapter>()
  const [eager, setEager] = createSignal(false)
  const [publicKey, setPublicKey] = createSignal<PublicKey>()
  onMount(() => {
    NightlyConnectAdapter.build(
      {
        appMetadata: {
          name: 'NCTestSolana',
          description: 'Nightly Connect Test',
          icon: 'https://docs.nightly.app/img/logo.png',
          additionalInfo: 'Courtesy of Nightly Connect team'
        }
      },
      {},
      document.getElementById('modalAnchor')
    ).then((adapter) => {
      adapter.on('connect', (pk) => {
        setPublicKey(pk)
      })

      adapter.on('disconnect', () => {
        setPublicKey(undefined)
      })
      adapter.on('change', (a) => {
        if (!!a.accounts?.length && a.accounts[0].publicKey) {
          setPublicKey(new PublicKey(a.accounts[0].publicKey))
        }
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
        <DropdownButton
          label="Change network"
          options={CUSTOM_NETWORK}
          onClickOption={async (network) => {
            adapter()!.changeNetwork({
              genesisHash: network.genesisHash,
              url: network.url
            })
          }}
        />

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
