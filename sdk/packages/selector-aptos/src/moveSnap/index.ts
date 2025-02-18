import {
  AccountAuthenticator,
  Aptos,
  AptosConfig,
  Deserializer,
  Ed25519PublicKey,
  Hex,
  Network
} from '@aptos-labs/ts-sdk'
import {
  AccountInfo,
  APTOS_DEVNET_CHAIN,
  APTOS_MAINNET_CHAIN,
  APTOS_TESTNET_CHAIN,
  AptosChangeNetworkMethod,
  AptosChangeNetworkNamespace,
  AptosConnectMethod,
  AptosConnectNamespace,
  AptosDisconnectMethod,
  AptosDisconnectNamespace,
  AptosFeatures,
  AptosGetAccountMethod,
  AptosGetAccountNamespace,
  AptosGetNetworkMethod,
  AptosGetNetworkNamespace,
  AptosOnAccountChangeInput,
  AptosOnAccountChangeMethod,
  AptosOnAccountChangeNamespace,
  AptosOnNetworkChangeInput,
  AptosOnNetworkChangeMethod,
  AptosOnNetworkChangeNamespace,
  AptosSignAndSubmitTransactionMethod,
  AptosSignAndSubmitTransactionNamespace,
  AptosSignMessageInput,
  AptosSignMessageMethod,
  AptosSignMessageNamespace,
  AptosSignMessageOutput,
  AptosSignTransactionMethod,
  AptosSignTransactionNamespace,
  AptosWalletAccount,
  NetworkInfo,
  UserResponseStatus,
  Wallet
} from '@aptos-labs/wallet-standard'
import { MetaMaskInpageProvider } from '@metamask/providers'
import { getRandomId } from '@nightlylabs/nightly-connect-base'
import { defaultSnapOrigin } from './config'
import { defaultAccountAptos, defaultAccountInfo, METAMASK_FLASK_ICON } from './const'
import { getInvokeSnap, InvokeSnapParams } from './snap/getInvoke'
import { getReadyStatus } from './snap/getMetamask'
import { getSnapsProvider } from './snap/provider'
import { getRequestSnaps } from './snap/requestSnaps'
import { Snap } from './types'
import { encodeAptosTransaction, isLocalSnap } from './utils'

export class MetamaskFlask implements Wallet {
  #activeAccount: AptosWalletAccount = defaultAccountAptos

  private _activeAccountInfo: AccountInfo = defaultAccountInfo
  private _isFlaskInstalled = false
  private _installedSnap: Snap | null = null
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
    return [this.#activeAccount]
  }

  get url() {
    return 'https://docs.metamask.io/snaps/get-started/install-flask/'
  }

  get name() {
    return 'MetaMask'
  }

  get icon() {
    return METAMASK_FLASK_ICON as 'data:image/svg+xml;base64,'
  }

  get flaskStatus() {
    return this._isFlaskInstalled
  }

  get installedSnap() {
    return this._installedSnap
  }

  get isMetamaskReady() {
    return isLocalSnap(defaultSnapOrigin) ? this._isFlaskInstalled : this._provider !== null
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

  constructor() {
    this.setUpProvider()
  }

  private async setUpProvider() {
    try {
      const provider = await getSnapsProvider()
      this._provider = provider
      if (provider) {
        this.checkReady()
        const invokeSnap = getInvokeSnap(provider)
        this._invokeSnap = invokeSnap
      }
    } catch (error) {
      this._provider = null
      this._invokeSnap = null
    }
  }

  private async checkReady() {
    const { isFlask, installedSnap } = await getReadyStatus(this._provider!)
    this._isFlaskInstalled = isFlask
    this._installedSnap = installedSnap
  }

  public network = () => {
    return { network: this.networkInfo.name }
  }

  public connect: AptosConnectMethod = async (silent, networkInfo) => {
    try {
      // on eager connect set up provider
      if (silent) {
        await this.setUpProvider()
      }
      if (!this._invokeSnap || !this._provider) {
        throw new Error("Can't invoke snap")
      }
      await this.checkReady()
      if (!this._installedSnap) {
        const { requestSnap, installedSnap } = getRequestSnaps(this._provider)
        this._installedSnap = installedSnap
        await requestSnap()
      }
      const response = (await this._invokeSnap({ method: 'connect' })) as {
        publicKey: string
        address: string
      }
      this._activeAccountInfo = new AccountInfo({
        address: response.address,
        publicKey: new Ed25519PublicKey(response.publicKey)
      })
      if (networkInfo && networkInfo.chainId !== this.networkInfo.chainId) {
        try {
          await this.changeNetwork(networkInfo)
        } catch {
          // silent error
        }
      }
      return {
        status: UserResponseStatus.APPROVED,
        args: this._activeAccountInfo
      }
    } catch (error) {
      console.log(error, 'Error connecting')
      return new Promise((resolve) =>
        resolve({
          status: UserResponseStatus.REJECTED
        })
      )
    }
  }

  public getAccount: AptosGetAccountMethod = async () => {
    return Promise.resolve(this._activeAccountInfo ?? defaultAccountInfo)
  }

  public getNetwork: AptosGetNetworkMethod = async () => {
    try {
      if (!this._invokeSnap || !this._provider) {
        throw new Error("Can't invoke snap")
      }

      const response = (await this._invokeSnap({
        method: 'getNetwork'
      })) as NetworkInfo

      if (!response) {
        throw new Error("Couldn't get network info")
      }

      this.networkInfo = response

      return response
    } catch (error) {
      console.log('Error getting network info', error)
      return this.networkInfo
    }
  }
  public signTransaction: AptosSignTransactionMethod = async (transaction) => {
    try {
      if (!this._invokeSnap || !this._provider) {
        throw new Error("Can't invoke snap")
      }

      const response = await this._invokeSnap({
        method: 'signAndSubmitTransaction',
        params: {
          payload: encodeAptosTransaction(transaction)
        }
      })

      const hexAuthericator = Hex.fromHexString(response as string)
      const signature = AccountAuthenticator.deserialize(
        new Deserializer(hexAuthericator.toUint8Array())
      )

      return {
        status: UserResponseStatus.APPROVED,
        args: signature
      }
    } catch (error) {
      console.log(error, 'Error signing tx')
      return new Promise((resolve) =>
        resolve({
          status: UserResponseStatus.REJECTED
        })
      )
    }
  }

  public signAndSubmitTransaction: AptosSignAndSubmitTransactionMethod = async (input) => {
    try {
      if (!this._invokeSnap || !this._provider) {
        throw new Error("Can't invoke snap")
      }
      const aptosConfig = new AptosConfig({
        fullnode: this.networkInfo.url ?? 'https://fullnode.mainnet.aptoslabs.com/v1',
        network: this.networkInfo.name
      })
      const aptos = new Aptos(aptosConfig)
      const transaction = await aptos.transaction.build.simple({
        sender: this._activeAccountInfo.address.toUint8Array(),
        data: input.payload
      })

      const response = await this._invokeSnap({
        method: 'signAndSubmitTransaction',
        params: {
          payload: encodeAptosTransaction(transaction)
        }
      })

      const hexAuthericator = Hex.fromHexString(response as string)
      const signature = AccountAuthenticator.deserialize(
        new Deserializer(hexAuthericator.toUint8Array())
      )
      const tx = await aptos.transaction.submit.simple({
        transaction: transaction,
        senderAuthenticator: signature
      })
      return {
        status: UserResponseStatus.APPROVED,
        args: tx
      }
    } catch (error) {
      console.log(error, 'Error signing and submitting tx')
      return new Promise((resolve) =>
        resolve({
          status: UserResponseStatus.REJECTED
        })
      )
    }
  }

  public signMessage: AptosSignMessageMethod = async (msg) => {
    try {
      if (!this._invokeSnap || !this._provider) {
        throw new Error("Can't invoke snap")
      }
      let toSign: AptosSignMessageInput
      if (typeof msg === 'string') {
        toSign = {
          message: msg,
          nonce: getRandomId()
        }
      } else {
        toSign = msg
      }

      const signature = await this._invokeSnap({
        method: 'signMessage',
        params: {
          ...toSign
        }
      })
      if (!signature) {
        throw new Error('Invalid sidnature')
      }
      const response = {
        fullMessage: toSign.message,
        message: toSign.message,
        nonce: toSign.nonce,
        prefix: 'APTOS' as const,
        signature
      } as AptosSignMessageOutput

      return {
        status: UserResponseStatus.APPROVED,
        args: response
      }
    } catch (error) {
      console.log(error, 'Error signing message')
      return new Promise((resolve) =>
        resolve({
          status: UserResponseStatus.REJECTED
        })
      )
    }
  }
  public disconnect: AptosDisconnectMethod = () => {
    return new Promise((resolve) => {
      resolve()
    })
  }

  public onAccountChange: AptosOnAccountChangeMethod = (input) => {
    this.onAccountChangeInput = input

    return new Promise((resolve) => {
      input(
        new AccountInfo({
          address: this._activeAccountInfo.address,
          publicKey: this._activeAccountInfo.publicKey
        })
      )
      resolve()
    })
  }

  public onNetworkChange: AptosOnNetworkChangeMethod = (input) => {
    this.onNetworkChangeInput = input
    return new Promise((resolve) => {
      input(this.networkInfo)
      resolve()
    })
  }

  changeNetwork: AptosChangeNetworkMethod = async (input) => {
    console.log(input, 'input')
    try {
      if (!this._invokeSnap || !this._provider) {
        throw new Error("Can't invoke snap")
      }

      if (this.networkInfo.name === input.name && this.networkInfo.chainId === input.chainId) {
        return {
          status: UserResponseStatus.APPROVED,
          args: {
            success: true
          }
        }
      }
      await this._invokeSnap({
        method: 'changeNetwork',
        // @ts-expect-error - generic params type - Record<string, unknown>
        params: input
      })

      this.onNetworkChangeInput?.(input)
      return {
        status: UserResponseStatus.APPROVED,
        args: {
          success: true
        }
      }
    } catch (error) {
      console.log('Error changing network', error)
      return {
        status: UserResponseStatus.REJECTED
      }
    }
  }
}

let _adapter: MetamaskFlask | null = null
export const getMetamaskFlaskAdapter = () => {
  if (_adapter) {
    return _adapter
  }
  _adapter = new MetamaskFlask()
  return _adapter
}
