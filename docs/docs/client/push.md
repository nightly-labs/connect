---
title: Push notifications
slug: client/push
---

Push server will allow to applications notify the user on incoming requests for previously established connections.  
Push notifications are being implemented for the client, when the device is locked. Though when the user application is open and session is active, all incoming requests are added to the List of PendingRequests. Once a client opens an application on its devices, he will have the option to decide on the transaction (approve or reject it).

### Push server ### 
In order to display the connect push notification on user device, pass to ```connect()``` method argument token: string.

```js
await client.connect({
  publicKey: alice_publicKey, // PublicKey: required
  sessionId: application.sessionId, // string: required
  accountId, // string: required
  token: token // string: optional for push notification purposes only
})
```
Depending on AppRequest, application will trigger push notification. Structure for notifications is following: 

```js
enum AppRequest {SignTransactions = 'SignTransactions', SignMessage = 'SignMessage' }

interface IBody {
  token: string
  notification: { 
      title: string; 
      body: string; // AppRequest
      icon: string 
    }
  data: {
    publicKey: string
    request: any // AppRequest 
    body: string // AppRequest
    title: string
    applicationName: string
    icon: string
    network: Network
    device: Device
    sessionId: string
  }
}
```