import {
  AccountAuthenticator,
  AccountAuthenticatorEd25519,
  AnyRawTransaction,
  Aptos,
  AccountPublicKey,
  Network,
  AptosConfig,
  InputGenerateTransactionPayloadData
} from '@aptos-labs/ts-sdk'
import {
  AccountInfo,
  AptosSignAndSubmitTransactionInput,
  AptosSignMessageInput,
  UserResponseStatus
} from '@aptos-labs/wallet-standard'
import { NightlyConnectAptosAdapter } from '@nightlylabs/wallet-selector-aptos'
import { createEffect, createSignal, onMount, Show } from 'solid-js'
import { Title } from '@solidjs/meta'
import toast from 'solid-toast'

const aptosConfig = new AptosConfig({
  network: Network.MAINNET
})
const aptos = new Aptos(aptosConfig)

export default function AptosPage() {
  const [adapter, setAdapter] = createSignal<NightlyConnectAptosAdapter>()
  const [eager, setEager] = createSignal(false)
  const [accountInfo, setAccountInfo] = createSignal<AccountInfo>()
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
      {},

      document.getElementById('modalAnchor'),
      {
        variablesOverride: {
          '--nc-color-primary': 'white',
          '--nc-img-logo': 'url(https://alephzero.org/aleph-design/brand-elements/logo-day.svg)'
        },
        stylesOverride: `
  .nc_headerWrapper {
    background-color: blue;
  }

  .nc_headerLogo {
    width: 200px;
  }

  .nc_modalContent {
    border-radius: 0;
    border: 3px dashed var(--nc-color-primary);
  }
  `,
        qrConfigOverride: {
          dotsOptions: {
            color: 'purple'
          }
        },
        networkDataOverride: {
          name: 'Custom name',
          icon: 'https://cdn.pixabay.com/photo/2016/04/01/00/28/face-1298202_640.png'
        }
      }
    ).then(async (adapter) => {
      adapter.canEagerConnect().then((canEagerConnect) => {
        setEager(canEagerConnect)
      })

      adapter.on('connect', (accInfo) => {
        if (accInfo && 'address' in accInfo) {
          setAccountInfo(accInfo)
        }
      })

      adapter.on('disconnect', () => {
        setAccountInfo(undefined)
        console.log('adapter disconnected')
      })

      adapter.on('accountChange', (accInfo) => {
        if (accInfo && 'address' in accInfo) {
          setAccountInfo(accInfo)
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
      <Title>Aptos Example</Title>
      <div id="modalAnchor" />
      <Show
        when={!!accountInfo()?.address}
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
        <h1>Current address: {accountInfo()?.address?.toString()}</h1>
        <button
          onClick={async () => {
            try {
              const payload: InputGenerateTransactionPayloadData = {
                function: '0x1::coin::transfer',
                typeArguments: ['0x1::aptos_coin::AptosCoin'],
                functionArguments: [
                  '0x960dbc655b847cad38b6dd056913086e5e0475abc27152b81570fd302cb10c38',
                  100
                ]
              }
              const signedTx = await adapter()!.signAndSubmitTransaction({ payload })

              // Verify the transaction was signed
              if (signedTx.status !== UserResponseStatus.APPROVED) {
                toast.error('Transaction was not approved')
                return
              }
              console.log('signedTx', signedTx)
              toast.success('Transaction was signed!')
            } catch (e) {
              toast.error("Error: couldn't sign and send transaction!")
              console.log(e)
            }
          }}>
          sign and submit tx
        </button>
        <button
          onClick={async () => {
            try {
              const transaction = await aptos.transaction.build.simple({
                sender: accountInfo()!.address?.toString(),
                data: {
                  function: '0x1::coin::transfer',
                  typeArguments: ['0x1::aptos_coin::AptosCoin'],
                  functionArguments: [
                    '0x960dbc655b847cad38b6dd056913086e5e0475abc27152b81570fd302cb10c38',
                    100
                  ]
                }
              })
              const signedTx = await adapter()!.signTransaction(transaction)
              // Verify the transaction was signed
              if (signedTx.status !== UserResponseStatus.APPROVED) {
                toast.error('Transaction was not approved')
                return
              }
              console.log('signedTx', signedTx)
              console.log(signedTx.args instanceof AccountAuthenticatorEd25519)
              console.log(signedTx.args.isEd25519())
              // @ts-expect-error sdsdsd
              console.log(signedTx.args.public_key)
              // @ts-expect-error sdsdsd
              console.log(signedTx.args.signature)
              const sig = new AccountAuthenticatorEd25519(
                // @ts-expect-error sdsdsd
                signedTx.args.public_key,
                // @ts-expect-error sdsdsd
                signedTx.args.signature
              )
              console.log(sig)
              await aptos.transaction.submit.simple({
                senderAuthenticator: sig,
                transaction: transaction
              })
              toast.success('Transaction was signed!')
            } catch (e) {
              toast.error("Error: couldn't sign and send transaction!")
              console.log(e)
            }
          }}>
          sign tx
        </button>
        <button
          onClick={async () => {
            try {
              const msgToSign: AptosSignMessageInput = {
                message: 'I love Nightly',
                address: true,
                nonce: 'YOLO'
              }
              const signed = await adapter()!.signMessage(msgToSign)

              if ('signature' in signed) {
                if (!signed.signature) throw new Error('Message was not approved')
              } else {
                if (signed.status !== UserResponseStatus.APPROVED)
                  throw new Error('Message was not approved')
              }
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
            setAccountInfo(undefined)
          }}>
          Disconnect
        </button>
      </Show>
    </main>
  )
}
