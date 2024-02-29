---
title: Sign Transaction
slug: substrate/sign_transaction
---

import Tabs from '@theme/Tabs';
import TabItem from '@theme/TabItem';

<Tabs>
<TabItem value="Client" label="Client">

Client is listening to the event `client.on('signTransactions')`, which returns data about transactions that are being requested.

When user accepts and signs a transaction with keyPair, the transaction is approved and sent to the blockchain.
To resolve the transaction client needs to pass in requestId and signed transaction.

```js
const alice_keypair = new Keyring()
alice_keypair.setSS58Format(42)
const aliceKeyringPair = alice_keypair.createFromUri('//Alice')

client.on('signTransactions', async (e) => {
  const payload = e.transactions[0] as SignerPayloadRaw
  const signature = aliceKeyringPair.sign(payload.data, { withType: true })
   await client.resolveSignTransaction({
      requestId: e.requestId,
      signedTransactions: [{ signature: u8aToHex(signature), id: new Date().getTime() }]
    })
})
```

</TabItem>

<TabItem value="Application" label="Application">

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
