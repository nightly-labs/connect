---
title: Send Transaction
slug: application/send
---

import Tabs from '@theme/Tabs';
import TabItem from '@theme/TabItem';

To transfer tokens, first application needs to create a transaction, which has to be signed by client.
Client on its side will fetch the transaction data and parse it to information (amount, tokens, value, icon, recipient address) to be displayed to the user.

Transaction are sent to the client as Promises. Client can accept or reject the request. Once client signs transaction, `signTransaction()` method returns resolved promise with Signed Transaction.

<Tabs>
<TabItem value="Solana" label="Solana">
  To transfer tokens, application needs to provide following information:
  recipient and sender public keys, the amount of lamports per transaction, feePayer and recentBlockhash.

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

<TabItem value="SUI" label="SUI">
  To transfer tokens, application needs to provide following information: transactionBlock, wallet account and chain.

```js
import { TransactionBlock } from '@mysten/sui.js'

const tx = new TransactionBlock()
const coin = tx.splitCoins(tx.gas, [tx.pure(100)])
tx.transferObjects([coin], tx.pure(RECEIVER_SUI_ADDRESS))
tx.setSenderIfNotSet(RECEIVER_SUI_ADDRESS)

const signedTx: SignedTransaction = await app.signTransactionBlock({
  transactionBlock: tx, // TransactionBlock
  account: aliceWalletAccount, // WalletAccount
  chain: 'sui:testnet' // IdentifierString
})
```

</TabItem>
</Tabs>
