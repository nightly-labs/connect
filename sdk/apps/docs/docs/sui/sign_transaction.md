---
title: Sign Transaction
slug: sui/sign_transaction
---

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
