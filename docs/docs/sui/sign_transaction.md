---
title: Sign Transaction
slug: sui/sign_transaction
---

Sending a signTransaction requires established connection with user wallet.

Transaction are sent to the client via wallet interface. Client can accept or reject the request. Once client signs transaction, `signTransactionBlock()` method returns resolved promise with Signed Transaction.

```js
import { Transaction } from '@mysten/sui'

const tx = new Transaction()
const coin = tx.splitCoins(tx.gas, [tx.pure.u64(100)])
tx.transferObjects([coin], tx.pure.address(RECEIVER_SUI_ADDRESS))
tx.setSenderIfNotSet(RECEIVER_SUI_ADDRESS)

const signedTx: SignedTransaction = await app.signTransactionBlock({
  transactionBlock: tx, // Transaction
  account: aliceWalletAccount, // WalletAccount
  chain: 'sui:testnet' // IdentifierString
})
```
