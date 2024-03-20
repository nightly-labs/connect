---
title: Listening for events
slug: aptos/events
---

An app can listen to events, subscribe to them and run some code whenever the particular event occurs.

If you have created a connection you can listen for events using the `on()` function.

```js
const adapter = NightlyConnectAptosAdapter.buildLazy(
  {
    appMetadata: {
      name: 'NCTestAptos',
      description: 'Nightly Connect Test',
      icon: 'https://docs.nightly.app/img/logo.png',
      additionalInfo: 'Courtesy of Nightly Connect team'
    },
    url: 'https://nc2.nightly.app'
  }
)

adapter.on('connect', (publicKey) => {
    ...
})

adapter.on('accountChange', (accInfo) => {
    ...
})

adapter.on('networkChange', (networkInfo) => {
    ...
})

adapter.on('disconnect', () => {
    ...
})

adapter.on('error', (error) => {
    ...
})
```
