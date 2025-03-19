import { Title } from '@solidjs/meta'
import { A } from '@solidjs/router'

export default function Home() {
  return (
    <main>
      <Title>Nightly Connect Examples</Title>

      <A href="/solana">
        <button>Solana</button>
      </A>
      <A href="/solanaLazy">
        <button>Solana - lazy adapter build</button>
      </A>
      <A href="/solanaInitOnConnect">
        <button>Solana - init app on connect</button>
      </A>
      <A href="/externalModal">
        <button>Solana - external modal</button>
      </A>
      <A href="/sui">
        <button>Sui</button>
      </A>
      <A href="/suiInitOnConnect">
        <button>Sui - init app on connect</button>
      </A>
      <A href="/iota">
        <button>IOTA</button>
      </A>
      <A href="/aleph">
        <button>Aleph</button>
      </A>
      <A href="/alephInitOnConnect">
        <button>Aleph - init app on connect</button>
      </A>
      <A href="/alephCustom">
        <button>Aleph - customized modal</button>
      </A>
      <A href="/aptos">
        <button>Aptos</button>
      </A>
      <A href="/aptosCustom">
        <button>Aptos custom</button>
      </A>
    </main>
  )
}
