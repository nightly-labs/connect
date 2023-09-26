---
title: Sign Transaction
slug: client/sign
---

import Tabs from '@theme/Tabs';
import TabItem from '@theme/TabItem';

Client is listening to the event `client.on('signTransactions')`, which returns data about transactions that are being requested.

When user accepts and signs a transaction with keyPair, the transaction is approved and sent to the blockchain.
To resolve the transaction client needs to pass in requestId and signed transaction.

<Tabs>

<TabItem value="Solana" label="Solana">

```js
import { Keypair, Transaction } from '@solana/web3.js'

interface SignSolanaTransactionEvent {
  requestId: string
  transactions: Array<VersionedTransaction>
  sessionId: string
}

const alice_keypair = Keypair.generate()

client.on('signTransactions', async (e) => {
  const tx = e.transactions[0]
  tx.sign([alice_keypair])
  // resolve
  await client.resolveSignTransaction({
    requestId: e.requestId,
    signedTransactions: [tx]
  })
})

```

</TabItem>

<TabItem value="SUI" label="SUI">

```js
import {
  Ed25519Keypair,
  messageWithIntent,
  toB64,
  toSerializedSignature,
  TransactionBlock,
} from '@mysten/sui.js'
import { blake2b } from '@noble/hashes/blake2b'

interface SignSuiTransactionEvent {
  sessionId: string
  requestId: string
  transactions: Array<TransactionToSign>
}

const alice_keypair: Ed25519Keypair  = Ed25519Keypair.fromSecretKey(hexToBytes(ALICE_PRIVE_KEY))

client.on('signTransactions', async (e) => {
  const tx = e.transactions[0].transaction
  const transactionBlockBytes = await TransactionBlock.from(tx).build({
    provider: suiConnection,
    onlyTransactionKind: true
  })

  const intentMessage = messageWithIntent(IntentScope.TransactionData, transactionBlockBytes)
  const digest = blake2b(intentMessage, { dkLen: 32 })
  const signatureArray = alice_keypair.signData(digest)
  const signature = toSerializedSignature({
    signature: signatureArray, // Uint8Array
    signatureScheme: 'ED25519', // SignatureScheme
    pubKey: alice_keypair.getPublicKey() // PublicKey
  })

  // resolve
  await client.resolveSignTransaction({
    responseId: e.requestId,
    signedTransactions: [
      {
        transactionBlockBytes: toB64(transactionBlockBytes),
        signature: signature
      }
    ]
  })
})
```

</TabItem>
<TabItem value="Substrate" label="Substrate">

```js

const alice_keypair = new Keyring()
alice_keypair.setSS58Format(42)
const aliceKeyringPair = alice_keypair.createFromUri('//Alice')

client.on('signTransactions', async (e) => {
  const payload = e.transactions[0] as SignerPayloadRaw
  const signature = aliceKeyringPair.sign(payload.data, { withType: true })
   await client.resolveSignTransaction({
      requestId: e.requestId,
      signedTransactions: [{ signature: u8aToHex(signature), id: new Date().getTime() }]
    })
})
```

</TabItem>
</Tabs>
