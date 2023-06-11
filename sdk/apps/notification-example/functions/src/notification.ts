import { onRequest } from 'firebase-functions/v2/https'
import { getMessaging } from 'firebase-admin/messaging'
import { initializeApp } from 'firebase-admin'
import { NotificationPayload } from '../../../../bindings/NotificationPayload'
import { RequestContent } from '../../../base/src/content'
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
