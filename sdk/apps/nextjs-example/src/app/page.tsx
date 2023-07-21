'use client'

import styles from './page.module.css'
import { NightlyConnectAdapter } from '@nightlylabs/wallet-selector-solana'

import { useEffect, useState } from 'react'
import { Connection, PublicKey, SystemProgram, Transaction as SolanaTx } from '@solana/web3.js'

export default function Home() {
  const [adapter, setAdapter] = useState<NightlyConnectAdapter>()
  const [eager, setEager] = useState(false)
  const [publicKey, setPublicKey] = useState<PublicKey>()

  const connection = new Connection('https://api.devnet.solana.com')
  useEffect(() => {
    NightlyConnectAdapter.build(
      {
        appMetadata: {
          name: 'NCTestSolana',
          description: 'Nightly Connect Test',
          icon: 'https://docs.nightly.app/img/logo.png',
          additionalInfo: 'Courtesy of Nightly Connect team'
        },
        url: 'https://nc2.nightly.app'
      },
      true
    ).then((adapter) => {
      adapter.on('connect', (pk) => {
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
  }, [])

  useEffect(() => {
    if (eager) {
      adapter?.connect().then(
        () => {
          console.log('connect resolved successfully')
        },
        () => {
          console.log('connect rejected')
        }
      )
    }
  }, [eager])

  return (
    <main className={styles.main}>
      {publicKey ? (
        <>
          <h1>Current public key: {publicKey.toString()}</h1>
          <button
            onClick={async () => {
              try {
                const ix = SystemProgram.transfer({
                  fromPubkey: adapter!.publicKey!,
                  lamports: 1e6,
                  toPubkey: new PublicKey('147oKbjwGDHEthw7sRKNrzYiRiGqYksk1ravTMFkpAnv')
                })
                const tx = new SolanaTx().add(ix).add(ix).add(ix).add(ix).add(ix)
                const a = await connection.getRecentBlockhash()
                tx.recentBlockhash = a.blockhash
                tx.feePayer = adapter!.publicKey!
                const signedTx = await adapter!.signTransaction!(tx)
                await connection.sendRawTransaction(signedTx!.serialize())

                window.alert('Transaction was signed and sent!')
              } catch (e) {
                window.alert("Error: couldn't sign and send transaction!")
                console.log(e)
              }
            }}>
            Send 0.005 SOL
          </button>
          <button
            onClick={async () => {
              try {
                await adapter?.signMessage!(new TextEncoder().encode('I love Nightly'))

                window.alert('Message was signed!')
              } catch (e) {
                window.alert("Error: couldn't sign message!")
                console.log(e)
              }
            }}>
            Sign message
          </button>
          <button
            onClick={() => {
              adapter?.disconnect()
            }}>
            Disconnect
          </button>
        </>
      ) : (
        <button
          onClick={() => {
            adapter?.connect().then(
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
      )}
    </main>
  )
}
