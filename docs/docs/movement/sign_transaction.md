---
title: Sign Transaction
slug: movement/sign_transaction
---

Sending a signTransaction requires established connection with user wallet.

Transaction are sent to the client via wallet interface. Client can accept or reject the request.

To create a transaction, first we have to establish a connection with an `Aptos` provider.

```js
import { Aptos, AptosConfig, AptosSettings } from '@aptos-labs/ts-sdk'

let _provider: Aptos | undefined
const endpoint = 'https://aptos.devnet.m1.movementlabs.xyz'

export const getAptos = () => {
  if (_provider) return _provider
  const conf = new AptosConfig({
    fullnode: endpoint,
    faucet: 'https://faucet.movementlabs.xyz'
  })
  _provider = new Aptos(conf) // DEVNET

  const a = async () => {
    const acc = await getAptos().account.getAccountResources({
      accountAddress: '0x975c0bad4ee36fcb48fe447647834b9c09ef44349ff593e90dd816dc5a3eccdc'
    })
    console.log(acc)
    const resp = await getAptos().faucet.fundAccount({
      accountAddress: '0x975c0bad4ee36fcb48fe447647834b9c09ef44349ff593e90dd816dc5a3eccdc',
      amount: 10000
    })
    console.log(resp)
  }

  a()

  return _provider
}
```

Then we can use the above function, to get Aptos provider instance for later use.

```js
const aptos = getAptos()
```

And create the transaction as such.

```js

const userAccount = {
  address: '' // Generated inside the onAccountChange event listener
  ...
}

const transaction = await aptos.transaction.build.simple({
  sender: userAccount.address.toString(),
  data: {
    function: '0x1::coin::transfer',
    typeArguments: ['0x1::aptos_coin::AptosCoin'],
    functionArguments: [
      '0x99881b6cdf90c9edb04e6b5912c236630b55587161dedc1fc05d53f72eec07e8',
      1_000_000,
    ],
  },
})

```

Finally we can sign the created transaction using the `signAndSubmitTransaction()` method, which returns resolved promise with Signed Transaction.

```js
const signedTx = await adapter.signAndSubmitTransaction(transaction)
```
