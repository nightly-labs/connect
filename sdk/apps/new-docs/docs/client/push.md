---
title: Push notifications
slug: client/push
---

Application sends POST request to user Endpoint, which contains data on request.
In order to display the connect push notification on user device `connect()` function required notificationEndpoint and token.

```js
type Connect = {
  publicKeys: string[],
  sessionId: string,
  notification?: Notification | undefined, // required for notification purposes
  device?: Device | undefined,
  metadata?: string | undefined
}

interface Notification {
  token: string;
  notificationEndpoint: string;
}
```

POST call example:

```js
const app = initializeApp(undefined, 'trigger-notification')
export const triggerNotification = onRequest(async (request, response) => {
  try {
    if (request.method !== 'POST') {
      response.status(400).send('Invalid request method')
      return
    }
    const payload = request.body as NotificationPayload
    const messaging = getMessaging(app)
    const requestContent = JSON.parse(payload.request) as RequestContent
    await messaging.send({
      token: payload.token,
      android: payload.device === 'Android' ? {} : undefined,
      notification: {
        title: requestContent.type,
        body: 'You have a new request' + payload.appMetadata.name,
        imageUrl: payload.appMetadata.icon
      },
      data: { payload: JSON.stringify(request.body) }
    })
    response.status(200).send('OK')
    return
  } catch (error: any) {
    console.log(error)
    response.status(400).send(error.toString())
    return
  }
})
```
