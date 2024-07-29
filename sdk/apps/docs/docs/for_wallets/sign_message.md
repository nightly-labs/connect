---
title: Sign Messages
slug: for_wallets/sign_message
---

import Tabs from '@theme/Tabs';
import TabItem from '@theme/TabItem';

Client can listen to the event `client.on('signMessages')`, which will returns user requests to sign messages. To resolve the transaction client needs to pass in requestId and signed message.

<Tabs>

<TabItem value="Solana" label="Solana">

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

</TabItem>

<TabItem value="SUI" label="SUI">

```js
import { fromB64, messageWithIntent, toB64, toSerializedSignature } from '@mysten/sui'
import { blake2b } from '@noble/hashes/blake2b'

client.on('signMessages', async (e) => {
  const msg = e.messages[0].message
  const msgTo64 = toB64(new TextEncoder().encode(msg))
  const intentMessage = messageWithIntent('PersonalMessage', fromB64(msgTo64))
  const digest = blake2b(intentMessage, { dkLen: 32 })
  const signature = alice_keypair.signData(digest)
  const signedMessage = {
    messageBytes: msg,
    signature: toSerializedSignature({
      signature,
      signatureScheme: 'ED25519',
      publicKey: alice_keypair.getPublicKey()
    })
  }
  // resolve
  await client.resolveSignMessage({
    responseId: e.responseId,
    signature: signedMessage
  })
})
```

</TabItem>
<TabItem value="Substrate" label="Substrate">
Signing messages on Substrate works the same way as signing transactions
</TabItem>

<TabItem value="Aptos" label="Aptos">

```js
export interface ResolveSignAptosMessage {
  requestId: string
  signedMessages: Array<AptosSignMessageOutput>
  sessionId: string
}

client.on('signMessage', async (e) => {
  const payload = e.messages[0]
  const signature = alice.sign(new Buffer(payload.message).toString('hex'))

  await client.resolveSignMessage({
    requestId: e.requestId,
    signedMessages: [
      {
        message: payload.message,
        signature: signature,
        fullMessage: payload.message,
        nonce: payload.nonce,
        prefix: 'APTOS'
      }
    ],
  })
})
```

</TabItem>

<TabItem value="Movement" label="Movement">
The process mirrors that of Aptos.
</TabItem>
</Tabs>
