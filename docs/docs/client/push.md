---
title: Push notifications
slug: client/push
---

Push server will allow to applications notify the user on incoming requests for previously established connections.  
Push notifications are being implemented for the client, when the device is locked. Though when the user application is open and session is active, all incoming requests are added to the List of PendingRequests. Once a client opens an application on its devices, he will have the option to decide on the transaction (approve or reject it).

### Push server

In order to display the connect push notification on user device, pass to `connect()` notification argument.

```js
type Connect = {
  publicKeys: string[],
  sessionId: string,
  notification?: Notification | undefined, // for notification purposes
  device?: Device | undefined,
  metadata?: string | undefined
}

interface Notification {
  token: string;
  notificationEndpoint: string;
}
```
