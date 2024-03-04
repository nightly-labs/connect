---
title: Sign Transaction
slug: solana/sign_transaction
---

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
