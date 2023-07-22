'use client'

import SuiProvider from '@/providers/sui-provider'
import { ConnectModal, useWalletKit } from '@mysten/wallet-kit'
import { useState } from 'react'
import styles from '../page.module.css'
export default function Home() {
  return (
    <main className={styles.main}>
      <SuiProvider>
        <Content />
      </SuiProvider>
    </main>
  )
}

export function Content() {
  const { currentAccount, isConnected, disconnect, signMessage, wallets } = useWalletKit()
  const [connectModalOpen, setConnectModalOpen] = useState(false)
  return (
    <>
      <ConnectModal
        open={connectModalOpen}
        onClose={() => {
          setConnectModalOpen(false)
        }}
      />
      {isConnected ? (
        <div>
          <div>Connected: {currentAccount?.address.toString().substring(0, 10)}</div>
          <div>
            <button
              onClick={() => {
                disconnect()
              }}>
              Disconnect
            </button>
          </div>
          <div>
            <button
              onClick={async () => {
                console.log('try sign')
                try {
                  await signMessage({
                    account: currentAccount!,
                    message: Buffer.from('Hello Nighlty')
                  })
                  console.log('signed')
                } catch (e) {
                  console.log('error ', e)
                }
              }}>
              sign message
            </button>
          </div>
        </div>
      ) : (
        <>
          {' '}
          <button
            onClick={async () => {
              setConnectModalOpen(true)
            }}>
            Connect
          </button>
        </>
      )}
    </>
  )
}
