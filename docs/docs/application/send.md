---
title: Send Transaction
slug: application/send
---
import Tabs from '@theme/Tabs';
import TabItem from '@theme/TabItem';

To transfer tokens, first application needs to create a transaction, which has to be signed by client.
Client on its side will fetch the transaction data and parse it to information (amount, tokens, value, icon, recipient address) to be displayed to the user.

Transaction are sent to the client as Promises. Client can accept or reject the request. Once client signs transaction, ```signTransaction()``` method returns resolved promise with Signed Transaction.

<Tabs>
<TabItem value="Solana" label="Solana">
  To transfer tokens, application needs to provide following information:
  recipient and sender public keys, the amount of lamports per transaction, feePayer and recentBlockhash.

```js
import { Keypair, LAMPORTS_PER_SOL, PublicKey, SystemProgram, Transaction } from '@solana/web3.js'

const alice_keypair: Keypair = Keypair.generate()
const alice_publicKey: PublicKey = alice_keypair.publicKey
const receiver_publicKey: PublicKey = Keypair.generate().publicKey

const ix = SystemProgram.transfer({
    fromPubkey: alice_publicKey, // PublicKey: string
    lamports: LAMPORTS_PER_SOL, // number
    toPubkey: receiver_publicKey // PublicKey: string
  })
  
const tx = new Transaction().add(ix)
tx.feePayer = alice_publicKey
tx.recentBlockhash = someRecentBlockhash

await application.signTransaction(tx)
```
</TabItem>

<TabItem value="Near" label="Near">
  To transfer tokens, application needs to provide following information:
  recipient and sender account ids, sender public key, the action (type of transaction, e.g. transfer) and recentBlockhash.

```js
import { KeyPairEd25519, PublicKey } from 'near-api-js/lib/utils'
import { transactions } from 'near-api-js'

const alice_keypair: KeyPairEd25519 = KeyPairEd25519.fromRandom()
const alice_publicKey: PublicKey = alice_keypair.publicKey
const action = transactions.transfer(new BN(amount))

const tx = transactions.createTransaction(
  FROM_ACCOUNT_ID, // signerId: string
  alice_publicKey, // PublicKey: string
  TARGET_ACCOUNT_ID, // receiverId: string
  1, // nonce: number
  [action], // actions: transactions.Action[]
  recentBlockhash // blockHash: Uint8Array
)

await application.signTransaction(tx)
```

</TabItem>
</Tabs>