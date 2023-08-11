import { AppMetadata } from '../../../bindings/AppMetadata'
import { Network } from '../../../bindings/Network'

export interface AppBaseInitialize {
  appMetadata: AppMetadata
  network: Network
  url?: string
  timeout?: number
  persistentSessionId?: string
  persistent?: boolean
}

export interface ClientBaseInitialize {
  clientId?: string
  url?: string
  timeout?: number
}
