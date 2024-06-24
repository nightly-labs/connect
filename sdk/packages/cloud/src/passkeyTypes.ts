/// REFERENCE https://github.com/kanidm/webauthn-rs/blob/master/tutorial/server/axum/assets/auth.js

// Register
export type HttpRegisterWithPasskeyStartResponse = { publicKey: PublicKeyCredentialCreationOptions }
export type HttpRegisterWithPasskeyFinishRequest = {
  email: string
  credential: {
    id: string
    rawId: string
    type: string
    response: { clientDataJSON: string; attestationObject: string }
  }
}

// Login
export type HttpLoginWithPasskeyStartResponse = { publicKey: PublicKeyCredentialRequestOptions }
export type HttpLoginWithPasskeyFinishRequest = {
  email: string
  credential: {
    id: string
    rawId: string
    type: string
    response: {
      clientDataJSON: string
      authenticatorData: string
      signature: string
      userHandle: string
    }
  }
  enforceIp: boolean
}

// Add passkey
export type HttpAddNewPasskeyStartResponse = { publicKey: PublicKeyCredentialCreationOptions }
export type HttpAddNewPasskeyFinishRequest = {
  credential: {
    id: string
    rawId: string
    type: string
    response: { clientDataJSON: string; attestationObject: string }
  }
}

// Reset passkey
export type HttpResetPasskeyStartResponse = { publicKey: PublicKeyCredentialCreationOptions }
export type HttpResetPasskeyFinishRequest = {
  email: string
  credential: {
    id: string
    rawId: string
    type: string
    response: { clientDataJSON: string; attestationObject: string }
  }
  code: string
}

// Passkey 2FA
export type HttpGetPasskeyChallengeResponse = { publicKey: PublicKeyCredentialRequestOptions }

// 2FA actions
export type HttpDeletePasskeyRequest = {
  passkeyId: string
  credential: PublicKeyCredential
}
