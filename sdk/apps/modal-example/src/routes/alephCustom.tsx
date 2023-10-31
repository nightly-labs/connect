import { getPolkadotWallets, NightlyConnectAdapter } from '@nightlylabs/wallet-selector-polkadot'
import { ApiPromise, WsProvider } from '@polkadot/api'
import { createEffect, createSignal, onMount, Show } from 'solid-js'
import { Title } from 'solid-start'
import toast from 'solid-toast'

const RECEIVER = '5CFRopxy991HCJj1HYtUQjaaBMw9iRLE9jxPndBsgdCjeJj5'

export const customFennecXml = `<svg viewBox="0 0 151 166" fill="none"
xmlns="http://www.w3.org/2000/svg">
<path d="M141.713 0C129.202 16.9595 113.518 28.7139 95.0161 36.5919C88.5913 34.8881 81.9893 33.9815 75.5 34.0441C69.0107 33.9815 62.4087 34.8881 55.9839 36.5919C37.4822 28.7139 21.7985 16.9751 9.28692 0C5.48675 9.23785 -9.05372 41.0936 8.41739 85.5947C8.41739 85.5947 2.82985 108.838 13.0871 128.798C13.0871 128.798 27.9657 122.265 39.7527 131.456C52.1032 141.194 48.1742 150.541 56.8695 158.575C64.341 166 75.5 166 75.5 166C75.5 166 86.659 166 94.1305 158.591C102.826 150.557 98.9129 141.209 111.247 131.471C123.034 122.28 137.913 128.814 137.913 128.814C148.17 108.853 142.583 85.6104 142.583 85.6104C160.054 41.0936 145.513 9.23785 141.713 0ZM17.7246 79.4362C8.20805 60.5072 5.59947 34.4974 11.6218 13.9584C19.5441 33.4345 30.3328 42.2034 43.1664 51.4412C37.7077 62.3829 27.4826 72.7305 17.7246 79.4362ZM45.0665 112.808C37.5949 109.604 35.9847 103.242 35.9847 103.242C46.2097 97.0053 61.2172 101.773 61.6841 116.544C53.7779 111.901 51.1532 115.403 45.0665 112.808ZM75.5 165.187C70.1379 165.187 65.7903 161.467 65.7903 156.887C65.7903 152.307 70.1379 148.587 75.5 148.587C80.8621 148.587 85.2097 152.307 85.2097 156.887C85.2097 161.483 80.8621 165.187 75.5 165.187ZM105.934 112.808C99.8468 115.403 97.2221 111.901 89.3159 116.544C89.7989 101.773 104.79 97.0053 115.015 103.242C115.015 103.242 113.405 109.604 105.934 112.808ZM133.275 79.4362C123.517 72.7305 113.292 62.3829 107.834 51.4256C120.667 42.2034 131.456 33.4345 139.378 13.9584C145.401 34.513 142.792 60.5228 133.275 79.4362Z" fill="#FF00FF"/>
</svg>`

export default function Polkadot() {
  const [adapter, setAdapter] = createSignal<NightlyConnectAdapter>()
  const [eager, setEager] = createSignal(false)
  const [publicKey, setPublicKey] = createSignal<string>()
  const [api, setApi] = createSignal<ApiPromise>()
  const provider = new WsProvider('wss://ws.test.azero.dev/')

  onMount(async () => {
    const adapter = NightlyConnectAdapter.buildLazy(
      {
        appMetadata: {
          name: 'NC TEST AlephZero',
          description: 'Nightly Connect Test',
          icon: 'https://docs.nightly.app/img/logo.png',
          additionalInfo: 'Courtesy of Nightly Connect team'
        },
        network: 'AlephZero'
      },
      true, // change this to false to test disabling eager connect
      document.getElementById('modalAnchor'),
      {
        variablesOverride: {
          '--nc-color-primary': 'green',
          '--nc-img-logo': 'url(https://alephzero.org/aleph-design/brand-elements/logo-day.svg)'
        },
        stylesOverride: `
        .nc_headerWrapper {
          background-color: red;
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
          image: customFennecXml,
          dotsOptions: {
            color: 'gold'
          }
        }
      }
    )

    adapter.canEagerConnect().then((canEagerConnect) => {
      setEager(canEagerConnect)
    })
    setAdapter(adapter)

    ApiPromise.create({
      provider
    }).then((api) => {
      setApi(api)
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
              console.log(getPolkadotWallets())
              await adapter()!.connect()
              const accounts = await adapter()!.accounts.get()
              console.log(accounts)
              setPublicKey(accounts[0].address)
              console.log('adapter', adapter())
            }}>
            Connect
          </button>
        }>
        <h1>Current address: {publicKey()}</h1>
        <button
          onClick={async () => {
            try {
              const payload = api()!.tx.balances.transfer(RECEIVER, 5000000000000)
              const signed = await payload.signAsync(publicKey()!, { signer: adapter()!.signer })
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
        {/* <button
          onClick={async () => {
            try {
              const accounts = await adapter()!.getAccounts()
              await adapter()!.signMessage!({
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
        </button> */}
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
