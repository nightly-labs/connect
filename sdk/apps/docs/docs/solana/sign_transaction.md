---
title: Sign Transaction
slug: solana/sign_transaction
---

import Tabs from '@theme/Tabs';
import TabItem from '@theme/TabItem';

<Tabs>
<TabItem value="Client" label="Client">

Client is listening to the event `client.on('signTransactions')`, which returns data about transactions that are being requested.

When user accepts and signs a transaction with keyPair, the transaction is approved and sent to the blockchain.
To resolve the transaction client needs to pass in requestId and signed transaction.

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

<TabItem value="Application" label="Application">

Sending a signTransaction requires established connection with user wallet.

Transaction are sent to the client via wallet interface. Client can accept or reject the request. Once client signs transaction, `signTransaction()` method returns resolved promise with Signed Transaction.

```js
import {
  Keypair,
  LAMPORTS_PER_SOL,
  SystemProgram,
  Transaction,
  VersionedTransaction
} from '@solana/web3.js'

const RECEIVER = Keypair.generate()
const ix = SystemProgram.transfer({
  fromPubkey: alice_keypair.publicKey,
  lamports: LAMPORTS_PER_SOL,
  toPubkey: RECEIVER.publicKey
})
const tx = new Transaction().add(ix)
tx.feePayer = alice_keypair.publicKey
tx.recentBlockhash = recentBlockhash

const signed: VersionedTransaction = await app.signTransaction(tx)
```

</TabItem>
</Tabs>
