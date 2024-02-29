---
title: Sign Transaction
slug: sui/sign_transaction
---

import Tabs from '@theme/Tabs';
import TabItem from '@theme/TabItem';

<Tabs>
<TabItem value="Client" label="Client">

Client is listening to the event `client.on('signTransactions')`, which returns data about transactions that are being requested.

When user accepts and signs a transaction with keyPair, the transaction is approved and sent to the blockchain.
To resolve the transaction client needs to pass in requestId and signed transaction.

```js
import {
  Ed25519Keypair,
  messageWithIntent,
  toB64,
  toSerializedSignature,
  TransactionBlock,
} from '@mysten/sui.js'
import { blake2b } from '@noble/hashes/blake2b'

interface SignSuiTransactionEvent {
  sessionId: string
  requestId: string
  transactions: Array<TransactionToSign>
}

const alice_keypair: Ed25519Keypair  = Ed25519Keypair.fromSecretKey(hexToBytes(ALICE_PRIVE_KEY))

client.on('signTransactions', async (e) => {
  const tx = e.transactions[0].transaction
  const transactionBlockBytes = await TransactionBlock.from(tx).build({
    provider: suiConnection,
    onlyTransactionKind: true
  })

  const intentMessage = messageWithIntent(IntentScope.TransactionData, transactionBlockBytes)
  const digest = blake2b(intentMessage, { dkLen: 32 })
  const signatureArray = alice_keypair.signData(digest)
  const signature = toSerializedSignature({
    signature: signatureArray, // Uint8Array
    signatureScheme: 'ED25519', // SignatureScheme
    pubKey: alice_keypair.getPublicKey() // PublicKey
  })

  // resolve
  await client.resolveSignTransaction({
    responseId: e.requestId,
    signedTransactions: [
      {
        transactionBlockBytes: toB64(transactionBlockBytes),
        signature: signature
      }
    ]
  })
})
```

</TabItem>

<TabItem value="Application" label="Application">

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

</TabItem>
</Tabs>
