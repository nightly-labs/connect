import { Ed25519PublicKey, Network, PublicKey, type AccountAddressInput } from '@aptos-labs/ts-sdk'
import {
  AccountInfo,
  APTOS_DEVNET_CHAIN,
  APTOS_MAINNET_CHAIN,
  APTOS_TESTNET_CHAIN,
  AptosChangeNetworkMethod,
  AptosChangeNetworkNamespace,
  AptosConnectNamespace,
  AptosDisconnectMethod,
  AptosDisconnectNamespace,
  AptosGetAccountNamespace,
  AptosGetNetworkMethod,
  AptosGetNetworkNamespace,
  AptosOnAccountChangeMethod,
  AptosOnAccountChangeNamespace,
  AptosOnNetworkChangeMethod,
  AptosOnNetworkChangeNamespace,
  AptosSignAndSubmitTransactionMethod,
  AptosSignAndSubmitTransactionNamespace,
  AptosSignMessageMethod,
  AptosSignMessageNamespace,
  AptosSignTransactionMethod,
  AptosSignTransactionNamespace,
  AptosWalletAccount,
  UserResponseStatus,
  Wallet,
  type AptosConnectMethod,
  type AptosFeatures,
  type AptosGetAccountMethod,
  type AptosOnAccountChangeInput,
  type AptosOnNetworkChangeInput,
  type NetworkInfo
} from '@aptos-labs/wallet-standard'
import { MetaMaskInpageProvider } from '@metamask/providers'
import { defaultAccountAptos, defaultAccountInfo, METAMASK_FLASK_ICON } from './const'
import { getSnapsProvider } from './provider'
import { getFlaskStatus, getInvokeSnap, getRequestSnaps, InvokeSnapParams } from './utils'

export interface IWebsiteMetadata {
  title: string | null
  url: string | null
  image: string | null
}

// TODO remove if types are to be exported
export type AccountInfoInput = {
  address: AccountAddressInput
  publicKey: PublicKey
  ansName?: string
}

export class MetamaskFlask implements Wallet {
  private _activeAccountInfo: AccountInfo | null = defaultAccountInfo
  private _activeAccount: AptosWalletAccount = defaultAccountAptos
  private _isFlaskInstalled = false
  _metadata: IWebsiteMetadata | null = null
  onNetworkChangeInput: AptosOnNetworkChangeInput | undefined
  onAccountChangeInput: AptosOnAccountChangeInput | undefined

  private networkInfo: NetworkInfo = {
    url: 'https://fullnode.mainnet.aptoslabs.com/v1',
    chainId: 1,
    name: Network.MAINNET
  }

  private _provider: MetaMaskInpageProvider | null = null
  private _invokeSnap: (({ method, params }: InvokeSnapParams) => Promise<unknown>) | null = null

  get version() {
    return '1.0.0' as const
  }

  get chains() {
    return [APTOS_DEVNET_CHAIN, APTOS_TESTNET_CHAIN, APTOS_MAINNET_CHAIN] as const
  }

  get accounts() {
    return [this._activeAccount]
  }

  get url() {
    return ''
  }

  get name() {
    return 'Flask'
  }

  get icon() {
    return METAMASK_FLASK_ICON as 'data:image/svg+xml;base64,'
  }

  get flaskStatus() {
    return this._isFlaskInstalled
  }

  get features(): AptosFeatures {
    return {
      [AptosConnectNamespace]: {
        version: '1.0.0',
        connect: this.connect
      },
      [AptosSignTransactionNamespace]: {
        version: '1.0.0',
        signTransaction: this.signTransaction
      },
      [AptosSignAndSubmitTransactionNamespace]: {
        version: '1.1.0',
        signAndSubmitTransaction: this.signAndSubmitTransaction
      },
      [AptosSignMessageNamespace]: {
        version: '1.0.0',
        signMessage: this.signMessage
      },
      [AptosDisconnectNamespace]: {
        version: '1.0.0',
        disconnect: this.disconnect
      },
      [AptosGetAccountNamespace]: {
        version: '1.0.0',
        account: this.getAccount
      },
      [AptosGetNetworkNamespace]: {
        version: '1.0.0',
        network: this.getNetwork
      },
      [AptosOnAccountChangeNamespace]: {
        version: '1.0.0',
        onAccountChange: this.onAccountChange
      },
      [AptosOnNetworkChangeNamespace]: {
        version: '1.0.0',
        onNetworkChange: this.onNetworkChange
      },
      [AptosChangeNetworkNamespace]: {
        version: '1.0.0',
        changeNetwork: this.changeNetwork
      }
    }
  }

  constructor(metadata: IWebsiteMetadata | null = null) {
    this._metadata = metadata
    this.setUpProvider()
  }

  private async setUpProvider() {
    try {
      const provider = await getSnapsProvider()
      this._provider = provider

      if (provider) {
        this._isFlaskInstalled = await getFlaskStatus(provider)
        const invokeSnap = getInvokeSnap(provider)
        this._invokeSnap = invokeSnap
      }
    } catch (error) {
      this._provider = null
      this._invokeSnap = null
    }
  }

  public network = () => {
    return { network: this.networkInfo.name }
  }

  public connect: AptosConnectMethod = async (silent, networkInfo) => {
    try {
      if (!this._invokeSnap || !this._provider) {
        throw new Error("Can't invoke snap")
      }
      const { requestSnap } = getRequestSnaps(this._provider)
      await requestSnap()
      const response = (await this._invokeSnap({ method: 'connect' })) as {
        publicKey: string
        address: string
      }

      this._activeAccountInfo = new AccountInfo({
        address: response.address,
        publicKey: new Ed25519PublicKey(response.publicKey)
      })

      return {
        status: UserResponseStatus.APPROVED,
        args: this._activeAccountInfo
      }
    } catch (error) {
      console.log(error, 'Error connecting')
      return {
        status: UserResponseStatus.REJECTED,
        args: null
      }
    }
  }

  public getAccount: AptosGetAccountMethod = async () => {
    return Promise.resolve(this._activeAccountInfo ?? defaultAccountInfo)
  }

  public getNetwork: AptosGetNetworkMethod = async () => {
    return await new Promise((resolve) => {
      resolve(this.networkInfo)
    })
  }
  // @ts-expect-error - uedninde
  public signTransaction: AptosSignTransactionMethod = async (input, asFeePayer) => {}
  // @ts-expect-error - uedninde
  public signAndSubmitTransaction: AptosSignAndSubmitTransactionMethod = async (input) => {}
  // @ts-expect-error - uedninde
  public signMessage: AptosSignMessageMethod = async (msg) => {}
  // @ts-expect-error - uedninde
  public disconnect: AptosDisconnectMethod = () => {}
  // @ts-expect-error - uedninde
  public onAccountChange: AptosOnAccountChangeMethod = (input) => {}

  public onNetworkChange: AptosOnNetworkChangeMethod = (input) => {
    this.onNetworkChangeInput = input
    return new Promise((resolve) => {
      input(this.networkInfo)
      resolve()
    })
  }
  onInjectNetworkChange = (input: NetworkInfo) => {}

  onInjectAccountChange = (input: string) => {}
  // @ts-expect-error - uedninde
  changeNetwork: AptosChangeNetworkMethod = async (input) => {}
}

let _adapter: MetamaskFlask | null = null
export const getMetamaskFlaskAdapter = () => {
  if (_adapter) {
    return _adapter
  }
  _adapter = new MetamaskFlask()
  return _adapter
}
