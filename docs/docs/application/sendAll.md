---
title: Send All Transactions
slug: application/sendAll
---
import Tabs from '@theme/Tabs';
import TabItem from '@theme/TabItem';

To send all transactions at once, use ```signAllTransactions()``` method, which requires array of transactions to be send, as well as public key. Function will return resolved or rejected promise, depending on client's action. Once client signs transaction, ```signAllTransactions()``` method returns resolved promise with Signed Transaction.

<Tabs>
<TabItem value="Solana" label="Solana">

```js
import { LAMPORTS_PER_SOL, SystemProgram, Transaction } from '@solana/web3.js'

const alice_keypair: Keypair = Keypair.generate()
const alice_publicKey: PublicKey = alice_keypair.publicKey
const receiver_publicKey: PublicKey = Keypair.generate().publicKey

const ix = SystemProgram.transfer({
  fromPubkey: alice_publicKey,
  lamports: LAMPORTS_PER_SOL,
  toPubkey: receiver_publicKey
})

const txs = [new Transaction().add(ix), new Transaction().add(ix)]
txs.forEach((tx) => {
  tx.feePayer = alice_publicKey
  tx.recentBlockhash = someRecentBlockhash
})

await application.signAllTransactions(txs)
```
</TabItem>

<TabItem value="Near" label="Near">

```js 
import { KeyPairEd25519, PublicKey } from 'near-api-js/lib/utils'
import { transactions } from 'near-api-js'

const alice_keypair: KeyPairEd25519 = KeyPairEd25519.fromRandom()
const alice_publicKey: PublicKey = alice_keypair.publicKey
const action = transactions.transfer(new BN(amount))


const tx = transactions.createTransaction(
  FROM_ACCOUNT_ID, // signerId: string
  alice_publicKey, // publicKey: NearPublicKey
  TARGET_ACCOUNT_ID, // receiverId: string
  1, // nonce: number
  [action], // actions: transactions.Action[]
  recentBlockhash // blockHash: Uint8Array
)

const tx2 = transactions.createTransaction(
  FROM_ACCOUNT_ID, // signerId: string
  alice_publicKey, // publicKey: NearPublicKey
  TARGET_ACCOUNT_ID, // receiverId: string
  1, // nonce: number
  [action], // actions: transactions.Action[]
  recentBlockhash // blockHash: Uint8Array
)

// Send and wait for processing
await application.signAllTransactions([tx, tx2])
```

</TabItem>
</Tabs>