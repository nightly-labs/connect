import { onMount } from 'solid-js'
import { Title } from 'solid-start'
import Counter from '~/components/Counter'
import '@nightlylabs/wallet-selector-modal'

export default function Home() {
  onMount(() => {
    if (typeof window !== 'undefined') {
      const modal = document.createElement('nightly-modal')
      modal.onClose = () => {
        console.log('test')
      }
      document.body.insertAdjacentElement('beforeend', modal)
    }
  })
  return (
    <main>
      <Title>Hello World</Title>
      <h1>Hello world!</h1>
      <Counter />
      <p>
        Visit{' '}
        <a href="https://start.solidjs.com" target="_blank">
          start.solidjs.com
        </a>{' '}
        to learn how to build SolidStart apps.
      </p>
    </main>
  )
}
