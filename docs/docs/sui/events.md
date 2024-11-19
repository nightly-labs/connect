---
title: Listening for events
slug: sui/events
---

An app can listen to events, subscribe to them and run some code whenever the particular event occurs.

If you have created a connection you can listen for events using the `on()` function.

```js
const adapter = NightlyConnectAdapter.buildLazy({
  appMetadata: {
    name: 'NCTestSui',
    description: 'Nightly Connect Test',
    icon: 'https://docs.nightly.app/img/logo.png',
    additionalInfo: 'Courtesy of Nightly Connect team'
  }
})

adapter.on('connect', (accounts) => {
    ...
})

adapter.on('change', (adapter) => {
    ...
})

adapter.on('disconnect', () => {
    ...
})

adapter.on('error', (error) => {
    ...
})
```
