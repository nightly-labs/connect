---
title: Sign Message
slug: substrate/sign_message
---

Client can listen to the event `client.on('signMessages')`, which will returns user requests to sign messages. To resolve the transaction client needs to pass in requestId and signed message.

Signing messages on Substrate works the same way as signing transactions
