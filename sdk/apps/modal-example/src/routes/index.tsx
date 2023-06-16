import { createSignal, onMount } from 'solid-js'
import { Title } from 'solid-start'
import Counter from '~/components/Counter'
import { NCSolanaSelector } from '@nightlylabs/wallet-selector-solana'
import { StandardWalletAdapter } from '@solana/wallet-standard'

const selector = new NCSolanaSelector({
  appInitData: {
    appMetadata: {
      name: 'Test application',
      description: 'If you see this message, you will be soon testing new Nightly Connect',
      icon: 'https://pbs.twimg.com/profile_images/1509999443692687367/T5-8VrZq_400x400.jpg',
      additionalInfo: 'Courtesy of Nightly Connect team'
    }
  }
})

export default function Home() {
  const [adapter, setAdapter] = createSignal<StandardWalletAdapter>()
  onMount(() => {
    selector.onSelectWallet = (newAdapter) => {
      setAdapter(newAdapter)
    }
  })
  return (
    <main>
      <Title>Hello World</Title>
      <button
        onClick={() => {
          selector.openModal()
        }}>
        Connect
      </button>
    </main>
  )
}
