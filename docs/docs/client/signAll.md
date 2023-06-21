---
title: Sign All Transactions
slug: client/signAll
---
import Tabs from '@theme/Tabs';
import TabItem from '@theme/TabItem';

Another option would be to sign and send multiple transactions, all at once.
The process is the same as a single transaction, but allows to pass in array of signed transactions.

<Tabs>

<TabItem value="Solana" label="Solana">

```js
import { Transaction } from '@solana/web3.js'

const signRequest = requests.requests[0] as SignTransactionsRequest

// Sign request
const signed = signRequest.transactions.map((tx) => {
  const txToSign = Transaction.from(Buffer.from(tx, 'hex'))
  txToSign.sign(alice_keypair)
  return txToSign
})

// Send signed transaction
await client.resolveSignTransaction({
  requestId: signRequest.id,
  signedTransactions: signed
})
```
</TabItem>

<TabItem value="Near" label="Near">

```js
import { transactions } from 'near-api-js'

const signRequest = requests.requests[0] as SignTransactionsRequest

// Sign request
const signed = signRequest.transactions.map((tx) => {
  const txToSign = transactions.Transaction.decode(
    Buffer.from(signRequest.transactions[0], 'hex')
  )
  const signature = alice_keypair.sign(Uint8Array.from(sha256.array(txToSign.encode())))
  return new transactions.SignedTransaction({
    transaction: txToSign,
    signature: new transactions.Signature({
      keyType: txToSign.publicKey.keyType,
      data: signature.signature
    })
  })
})

// Send signed transaction
await client.resolveSignTransaction({
  requestId: signRequest.id,
  signedTransactions: signed
})
```
</TabItem>
</Tabs>