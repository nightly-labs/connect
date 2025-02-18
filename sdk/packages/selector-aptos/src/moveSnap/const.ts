import { Ed25519PublicKey } from '@aptos-labs/ts-sdk'
import { AccountInfo, AptosWalletAccount } from '@aptos-labs/wallet-standard'

export const METAMASK_FLASK_ICON =
  'https://upload.wikimedia.org/wikipedia/commons/thumb/3/36/MetaMask_Fox.svg/2048px-MetaMask_Fox.svg.png'

export const defaultAccount = {
  address: '0x1',
  publicKey: new Uint8Array(0),
  chains: [],
  features: [],
  label: '',
  icon: METAMASK_FLASK_ICON as 'data:image/svg+xml;base64,'
}

export const defaultAccountAptos: AptosWalletAccount = {
  ...defaultAccount,
  signingScheme: 0
}

export const defaultAccountInfo: AccountInfo = new AccountInfo({
  address: '0x1',
  publicKey: new Ed25519PublicKey(
    '0x1111111111111111111111111111111111111111111111111111111111111111'
  )
})
