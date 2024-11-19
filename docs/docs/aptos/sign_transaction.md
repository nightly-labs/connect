---
title: Sign Transaction
slug: aptos/sign_transaction
---

Sending a signTransaction requires established connection with user wallet.

Transaction are sent to the client via wallet interface. Client can accept or reject the request. Once client signs transaction, `signAndSubmitTransaction()` method returns resolved promise with Signed Transaction.

```js
import { Aptos } from '@aptos-labs/ts-sdk'

const aptos = new Aptos()

const accountInfo = {
  address: '' // Generated inside the onAccountChange event listener
  ...
}

const transaction = await aptos.transaction.build.simple({
  sender: accountInfo.address.toString(),
  data: {
    function: '0x1::coin::transfer',
    typeArguments: ['0x1::aptos_coin::AptosCoin'],
    functionArguments: ['0x960dbc655b847cad38b6dd056913086e5e0475abc27152b81570fd302cb10c38', 100]
  }
})

const signedTx = await adapter.signAndSubmitTransaction({
  rawTransaction: transaction.rawTransaction
})
```
