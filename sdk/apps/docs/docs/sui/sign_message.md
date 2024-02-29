---
title: Sign Message
slug: sui/sign_message
---

Client can listen to the event `client.on('signMessages')`, which will returns user requests to sign messages. To resolve the transaction client needs to pass in requestId and signed message.

```js
import {
  fromB64,
  IntentScope,
  messageWithIntent,
  toB64,
  toSerializedSignature
} from '@mysten/sui.js'
import { blake2b } from '@noble/hashes/blake2b'

client.on('signMessages', async (e) => {
  const msg = e.messages[0].message
  const msgTo64 = toB64(new TextEncoder().encode(msg))
  const intentMessage = messageWithIntent(IntentScope.PersonalMessage, fromB64(msgTo64))
  const digest = blake2b(intentMessage, { dkLen: 32 })
  const signature = alice_keypair.signData(digest)
  const signedMessage = {
    messageBytes: msg,
    signature: toSerializedSignature({
      signature,
      signatureScheme: 'ED25519',
      pubKey: alice_keypair.getPublicKey()
    })
  }
  // resolve
  await client.resolveSignMessage({
    responseId: e.responseId,
    signature: signedMessage
  })
})
```
