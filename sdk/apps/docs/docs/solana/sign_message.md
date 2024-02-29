---
title: Sign Message
slug: solana/sign_message
---

Client can listen to the event `client.on('signMessages')`, which will returns user requests to sign messages. To resolve the transaction client needs to pass in requestId and signed message.

```js
import nacl from 'tweetnacl'

client.on('signMessages', async (e) => {
  const msg = e.messages[0].message
  const encoded = Uint8Array.from(sha256.array(msg))
  const signature = nacl.sign.detached(encoded, alice_keypair.secretKey)
  // resolve
  await client.resolveSignMessage({
    requestId: e.responseId,
    signature: signature
  })
})
```
