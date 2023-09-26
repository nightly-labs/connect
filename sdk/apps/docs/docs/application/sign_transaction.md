---
title: Sign Transaction
slug: application/send
---

import Tabs from '@theme/Tabs';
import TabItem from '@theme/TabItem';

<Tabs>
<TabItem value="Solana" label="Solana">
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

<TabItem value="SUI" label="SUI">
Sending a signTransaction requires established connection with user wallet.

Transaction are sent to the client via wallet interface. Client can accept or reject the request. Once client signs transaction, `signTransactionBlock()` method returns resolved promise with Signed Transaction.

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
<TabItem value="Substrate" label="Substrate">

Nightly connect automatically turns into remote signer after connection from client (Mobile/Extension).
Signing transaction is as simple as passing `signer` object

```js
// Create transaction
const payload = api.tx.balances.transfer(RECEIVER, 50000)
// Sign transaction using adapter
const signed = await payload.signAsync(SENDER, { signer: adapter.signer })
// Send transaction
await signed.send()
```

</TabItem>
</Tabs>
