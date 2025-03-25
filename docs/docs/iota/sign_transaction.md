---
title: Sign Transaction
slug: iota/sign_transaction
---

Sending a signTransaction requires established connection with user wallet.

Transaction are sent to the client via wallet interface. Client can accept or reject the request. Once client signs transaction, `signTransaction()` method returns resolved promise with Signed Transaction.

```js
import { Transaction } from '@iota/iota-sdk'

const transaction = new Transaction()
const coin = transaction.splitCoins(transaction.gas, [transaction.pure.u64(100)])
transaction.transferObjects([coin], transaction.pure.address(RECEIVER_IOTA_ADDRESS))
transaction.setSenderIfNotSet(RECEIVER_IOTA_ADDRESS)

await adapter.signTransaction({
  transaction: transaction, // Transaction
  chain: 'iota:testnet', // IdentifierString
  account: aliceWalletAccount // WalletAccount
})
```
