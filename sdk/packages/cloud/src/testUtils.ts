import { HttpLoginRequest } from '../../../bindings/HttpLoginRequest'
import { HttpRegisterWithPasswordRequest } from '../../../bindings/HttpRegisterWithPasswordRequest'
import { NightlyCloud } from './app'

export async function createUser(cloudClient: NightlyCloud): Promise<string> {
  const email = randomEmail() + '@gmail.com'
  const password = 'Password123'

  let registerPayload = {
    email,
    password
  } as HttpRegisterWithPasswordRequest

  await await cloudClient.registerWithPassword(registerPayload)

  let loginPayload = {
    email,
    password,
    enforceIp: false
  } as HttpLoginRequest

  return (await cloudClient.loginWithPassword(loginPayload)).userId.toString()
}

export function randomEmail(): string {
  return Math.random().toString(36).substring(7)
}
