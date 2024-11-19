---
title: Sign Transaction
slug: substrate/sign_transaction
---

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
