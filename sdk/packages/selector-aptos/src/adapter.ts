/* eslint-disable @typescript-eslint/no-empty-function */
import {
  AppInitData,
  ConnectionType,
  IWalletListItem,
  MetadataWallet,
  NightlyConnectSelectorModal,
  XMLOptions,
  clearRecentStandardWalletForNetwork,
  clearSessionIdForNetwork,
  getRecentStandardWalletForNetwork,
  getWalletsList,
  isMobileBrowser,
  isStandardConnectedForNetwork,
  persistRecentStandardWalletForNetwork,
  persistStandardConnectForNetwork,
  persistStandardDisconnectForNetwork,
  sleep,
  triggerConnect
} from '@nightlylabs/wallet-selector-base'
import {
  AccountInfo,
  AdapterPlugin,
  NetworkInfo,
  OnAccountChange,
  OnNetworkChange,
  SignMessagePayload,
  SignMessageResponse,
  Types,
  WalletName
} from '@aptos-labs/wallet-adapter-core'
import { AppAptos } from '@nightlylabs/nightly-connect-aptos'
import { logoBase64 } from './icon'
import { aptosWalletsFilter } from './detection'

export const AptosWalletName = 'Nightly Connect' as WalletName<'Nightly Connect'>
export const APTOS_NETWORK = 'aptos'

export class NightlyConnectAptosAdapter implements AdapterPlugin {
  // TODO: add later "implements WalletAdapter"
  name = AptosWalletName
  icon: `data:image/${'svg+xml' | 'webp' | 'png' | 'gif'};base64,${string}` = logoBase64
  readonly url =
    'https://chromewebstore.google.com/detail/nightly/fiikommddbeccaoicoejoniammnalkfa?hl=en'
  connected = false
  // Nightly connect fields
  private _app: AppAptos | undefined
  private _innerStandardAdapter: AdapterPlugin | undefined
  private _loading = false
  private _modal: NightlyConnectSelectorModal | undefined
  private _appInitData: AppInitData
  private _eagerConnectForStandardWallets = true
  private _walletsList: IWalletListItem[] = []
  private _chosenMobileWalletName: string | undefined
  private _account: AccountInfo | undefined = undefined
  private _connectionType: ConnectionType | undefined
  private _metadataWallets: MetadataWallet[] = []
  private _initOnConnect: boolean

  get walletsList() {
    return this._walletsList
  }

  set walletsList(list: IWalletListItem[]) {
    this._walletsList = list
    if (this._modal) {
      this._modal.walletsList = list
    }
  }

  // We need internal _connecting since sui messes with connecting state
  private _connecting = false
  constructor(
    appInitData: AppInitData,
    eagerConnectForStandardWallets?: boolean,
    initOnConnect = false
  ) {
    this._connecting = false
    this.connected = false
    this._appInitData = appInitData
    this._eagerConnectForStandardWallets = eagerConnectForStandardWallets ?? true
    this._loading = false
    this._initOnConnect = initOnConnect
  }

  get provider(): any {
    if (!this.connected) {
      throw new Error('Wallet not connected')
    }
    switch (this._connectionType) {
      case ConnectionType.Nightly: {
        throw new Error('Not supported')
      }
      case ConnectionType.WalletStandard: {
        return this._innerStandardAdapter?.provider
      }
    }
    return
  }

  public static initApp = async (
    appInitData: AppInitData
  ): Promise<[AppAptos, MetadataWallet[]]> => {
    try {
      return await Promise.all([
        AppAptos.build({ ...appInitData }),
        AppAptos.getWalletsMetadata(
          `${appInitData.url ?? 'https://nc2.nightly.app'}/get_wallets_metadata`
        )
          .then((list) =>
            list.map((wallet) => ({
              name: wallet.name,
              icon: wallet.image.default,
              deeplink: wallet.mobile,
              link: wallet.homepage,
              walletType: wallet.walletType
            }))
          )
          .catch(() => [] as MetadataWallet[])
      ])
    } catch {
      clearSessionIdForNetwork(APTOS_NETWORK)
      return await Promise.all([
        AppAptos.build({ ...appInitData }),
        AppAptos.getWalletsMetadata(
          `${appInitData.url ?? 'https://nc2.nightly.app'}/get_wallets_metadata`
        )
          .then((list) =>
            list.map((wallet) => ({
              name: wallet.name,
              icon: wallet.image.default,
              deeplink: wallet.mobile,
              link: wallet.homepage,
              walletType: wallet.walletType
            }))
          )
          .catch(() => [] as MetadataWallet[])
      ])
    }
  }

  public static build = async (
    appInitData: AppInitData,
    eagerConnectForStandardWallets?: boolean,
    anchorRef?: HTMLElement | null,
    uiOverrides?: {
      variablesOverride?: object
      stylesOverride?: string
      qrConfigOverride?: Partial<XMLOptions>
    }
  ) => {
    const adapter = new NightlyConnectAptosAdapter(appInitData, eagerConnectForStandardWallets)

    adapter._modal = new NightlyConnectSelectorModal(
      adapter.walletsList,
      appInitData.url ?? 'https://nc2.nightly.app',
      {
        name: APTOS_NETWORK,
        icon: 'https://registry.nightly.app/networks/aptos.png'
      },
      anchorRef,
      uiOverrides?.variablesOverride,
      uiOverrides?.stylesOverride,
      uiOverrides?.qrConfigOverride
    )

    const [app, metadataWallets] = await NightlyConnectAptosAdapter.initApp(appInitData)
    console.log({ metadataWallets })
    adapter._app = app
    adapter._metadataWallets = metadataWallets
    adapter.walletsList = getWalletsList(
      metadataWallets,
      aptosWalletsFilter,
      getRecentStandardWalletForNetwork(APTOS_NETWORK) ?? undefined
    )

    return adapter
  }

  public static buildLazy = (
    appInitData: AppInitData,
    eagerConnectForStandardWallets?: boolean,
    anchorRef?: HTMLElement | null,
    uiOverrides?: {
      variablesOverride?: object
      stylesOverride?: string
      qrConfigOverride?: Partial<XMLOptions>
    }
  ) => {
    const adapter = new NightlyConnectAptosAdapter(appInitData, eagerConnectForStandardWallets)

    adapter.walletsList = getWalletsList(
      [],
      aptosWalletsFilter,
      getRecentStandardWalletForNetwork(APTOS_NETWORK) ?? undefined
    )
    adapter._modal = new NightlyConnectSelectorModal(
      adapter.walletsList,
      appInitData.url ?? 'https://nc2.nightly.app',
      {
        name: APTOS_NETWORK,
        icon: 'https://registry.nightly.app/networks/sui.png'
      },
      anchorRef,
      uiOverrides?.variablesOverride,
      uiOverrides?.stylesOverride,
      uiOverrides?.qrConfigOverride
    )

    adapter._loading = true

    NightlyConnectAptosAdapter.initApp(appInitData).then(([app, metadataWallets]) => {
      adapter._app = app
      adapter._metadataWallets = metadataWallets
      adapter.walletsList = getWalletsList(
        metadataWallets,
        aptosWalletsFilter,
        getRecentStandardWalletForNetwork(APTOS_NETWORK) ?? undefined
      )
      adapter._loading = false
    })

    return adapter
  }
  public static buildWithInitOnConnect = (
    appInitData: AppInitData,
    eagerConnectForStandardWallets?: boolean,
    anchorRef?: HTMLElement | null,
    uiOverrides?: {
      variablesOverride?: object
      stylesOverride?: string
      qrConfigOverride?: Partial<XMLOptions>
    }
  ) => {
    const adapter = new NightlyConnectAptosAdapter(
      appInitData,
      eagerConnectForStandardWallets,
      true
    )

    adapter.walletsList = getWalletsList(
      [],
      aptosWalletsFilter,
      getRecentStandardWalletForNetwork(APTOS_NETWORK) ?? undefined
    )

    adapter._modal = new NightlyConnectSelectorModal(
      adapter.walletsList,
      appInitData.url ?? 'https://nc2.nightly.app',
      {
        name: APTOS_NETWORK,
        icon: 'https://registry.nightly.app/networks/sui.png'
      },
      anchorRef,
      uiOverrides?.variablesOverride,
      uiOverrides?.stylesOverride,
      uiOverrides?.qrConfigOverride
    )

    return adapter
  }
  connect = async () => {
    return new Promise<AccountInfo>((resolve, reject) => {
      const innerConnect = async () => {
        try {
          if (this._connecting) {
            reject()
            return
          }
          if (this._account) {
            resolve(this._account)
            return
          }

          if (this._initOnConnect) {
            this._connecting = true

            if (!this._app) {
              try {
                const [app, metadataWallets] = await NightlyConnectAptosAdapter.initApp(
                  this._appInitData
                )

                this._app = app
                this._metadataWallets = metadataWallets
                this.walletsList = getWalletsList(
                  metadataWallets,
                  aptosWalletsFilter,
                  getRecentStandardWalletForNetwork(APTOS_NETWORK) ?? undefined
                )
              } catch {
                if (!this._app) {
                  this._connecting = false
                  throw new Error('Wallet not ready')
                }
              }
            }
          } else {
            if (this._loading) {
              // we do it to ensure proper connect flow in case if adapter is lazily built, but e. g. sui wallets selector uses its own eager connect
              for (let i = 0; i < 200; i++) {
                await sleep(10)

                if (!this._loading) {
                  break
                }
              }

              if (this._loading) {
                throw new Error('Wallet not ready')
              }
            }

            if (!this._app) {
              throw new Error('Wallet not ready')
            }

            this._connecting = true
          }

          if (this._app.hasBeenRestored() && this._app.connectedPublicKeys.length > 0) {
            this.eagerConnectDeeplink()
            const keys = this._app.connectedPublicKeys.map((pk) => {
              const accountInfo: AccountInfo = JSON.parse(pk)
              return accountInfo
            })
            this._account = keys[0]
            this.connected = true
            this._connecting = false
            this._connectionType = ConnectionType.Nightly
            resolve(this._account)
            return
          }
          const recentName = getRecentStandardWalletForNetwork(APTOS_NETWORK)
          if (
            this._eagerConnectForStandardWallets &&
            recentName !== null &&
            isStandardConnectedForNetwork(APTOS_NETWORK)
          ) {
            await this.connectToStandardWallet(recentName, resolve)
            return
          }

          this._app.on('userConnected', (e) => {
            try {
              if (this._chosenMobileWalletName) {
                persistRecentStandardWalletForNetwork(this._chosenMobileWalletName, APTOS_NETWORK)
              } else {
                clearRecentStandardWalletForNetwork(APTOS_NETWORK)
              }
              this._account = e.accounts[0]
              this.connected = true
              this._connecting = false
              this._connectionType = ConnectionType.Nightly
              this._modal?.closeModal()
              resolve(this._account)
            } catch (e) {
              this.disconnect()
              this._modal?.closeModal()
              reject(e)
            }
          })
          if (!this._modal) {
            this._connecting = false
            reject(new Error('Wallet not ready'))
          }
          // _modal is defined here
          this._modal!.onClose = () => {
            if (this._connecting) {
              this._connecting = false
              const error = new Error('Connection cancelled')
              reject(error)
            }
          }
          // Try open
          const opened = this._modal!.openModal(this._app!.sessionId, (walletName) => {
            if (
              isMobileBrowser() &&
              !this.walletsList.find((w) => w.name === walletName)?.standardWallet
            ) {
              this.connectToMobileWallet(walletName)
            } else {
              this.connectToStandardWallet(walletName, resolve)
            }
          })
          // If modal is not opened, reject
          // This might be caused by SSR
          if (!opened) {
            this._connecting = false
            const error = new Error('Failed to open modal')
            reject(error)
          }
        } catch (error: unknown) {
          this._connecting = false
          reject(error)
        }
      }

      innerConnect()
    })
  }

  disconnect = async () => {
    if (!this.connected) {
      throw new Error('Wallet not connected')
    }
    switch (this._connectionType) {
      case ConnectionType.Nightly: {
        clearSessionIdForNetwork(APTOS_NETWORK)
        // Refresh app session
        this._app = await AppAptos.build(this._appInitData)
        break
      }
      case ConnectionType.WalletStandard: {
        if (this._innerStandardAdapter) {
          await this._innerStandardAdapter.disconnect()
          persistStandardDisconnectForNetwork(APTOS_NETWORK)
          this.walletsList = getWalletsList(
            this._metadataWallets,
            aptosWalletsFilter,
            getRecentStandardWalletForNetwork(APTOS_NETWORK) ?? undefined
          )
        }
        break
      }
    }
    this._innerStandardAdapter = undefined
    this._connectionType = undefined
    this.connected = false
    this._connecting = false
    this._account = undefined
  }
  async account(): Promise<AccountInfo> {
    if (!this.connected) {
      throw new Error('Wallet not connected')
    }
    switch (this._connectionType) {
      case ConnectionType.Nightly: {
        return this._app!.account
      }
      case ConnectionType.WalletStandard: {
        // @ts-expect-error Aptos types suck
        return this._innerStandardAdapter!.account
      }
    }
    throw 'Should not happen'
  }
  async network(): Promise<NetworkInfo> {
    if (!this.connected) {
      throw new Error('Wallet not connected')
    }
    switch (this._connectionType) {
      case ConnectionType.Nightly: {
        return this._app!.network
      }
      case ConnectionType.WalletStandard: {
        return this._innerStandardAdapter!.network as NetworkInfo
      }
    }
    throw 'Should not happen'
  }
  async onNetworkChange(callback: Parameters<OnNetworkChange>[0]): Promise<void> {
    if (!this.connected) {
      throw new Error('Wallet not connected')
    }
    switch (this._connectionType) {
      case ConnectionType.Nightly: {
        throw new Error('Not supported')
      }
      case ConnectionType.WalletStandard: {
        return this._innerStandardAdapter!.onNetworkChange(callback)
      }
    }
    throw 'Should not happen'
  }
  async onAccountChange(callback: Parameters<OnAccountChange>[0]): Promise<void> {
    if (!this.connected) {
      throw new Error('Wallet not connected')
    }
    switch (this._connectionType) {
      case ConnectionType.Nightly: {
        throw new Error('Not supported')
      }
      case ConnectionType.WalletStandard: {
        return this._innerStandardAdapter!.onAccountChange(callback)
      }
    }
    throw 'Should not happen'
  }
  async signAndSubmitTransaction(
    transaction: Types.TransactionPayload,
    options?: any
  ): Promise<{ hash: Types.HexEncodedBytes }> {
    if (!this._app || !this._connectionType) {
      throw new Error('Wallet not ready')
    }
    switch (this._connectionType) {
      case ConnectionType.Nightly: {
        return await this._app.signAndSubmitTransaction(transaction, options)
      }
      case ConnectionType.WalletStandard: {
        return await this._innerStandardAdapter!.signAndSubmitTransaction(transaction, options)
      }
    }
  }
  async signMessage(message: SignMessagePayload): Promise<SignMessageResponse> {
    if (!this._app || !this._connectionType) {
      throw new Error('Wallet not ready')
    }
    switch (this._connectionType) {
      case ConnectionType.Nightly: {
        return await this._app.signMessage(message)
      }
      case ConnectionType.WalletStandard: {
        return await this._innerStandardAdapter!.signMessage(message)
      }
    }
  }

  canEagerConnect = async () => {
    // utility for case if somebody wants to fire connect asap, but doesn't want to show modal
    // if e. g. there was no user connected on the device yet
    if (this._loading) {
      for (let i = 0; i < 200; i++) {
        await sleep(10)

        if (!this._loading) {
          break
        }
      }
    }

    if (this._loading) {
      false
    }

    if (this._app && this._app.hasBeenRestored() && this._app.connectedPublicKeys.length > 0) {
      return true
    }

    if (
      this._eagerConnectForStandardWallets &&
      getRecentStandardWalletForNetwork(APTOS_NETWORK) !== null &&
      isStandardConnectedForNetwork(APTOS_NETWORK)
    ) {
      return true
    }

    return false
  }

  eagerConnectDeeplink = () => {
    if (isMobileBrowser() && this._app) {
      const mobileWalletName = getRecentStandardWalletForNetwork(APTOS_NETWORK)
      const wallet = this.walletsList.find((w) => w.name === mobileWalletName)

      if (typeof wallet === 'undefined') {
        return
      }

      if (wallet.deeplink === null) {
        return
      }

      if (wallet.deeplink.native !== null) {
        this._app.connectDeeplink({
          walletName: wallet.name,
          url: wallet.deeplink.native
        })
        return
      }
      if (wallet.deeplink.universal !== null) {
        this._app.connectDeeplink({
          walletName: wallet.name,
          url: wallet.deeplink.universal
        })
        return
      }
    }
  }

  connectToMobileWallet = (walletName: string) => {
    if (this._modal) {
      this._modal.setStandardWalletConnectProgress(true)
    }

    const wallet = this.walletsList.find((w) => w.name === walletName)

    if (!this._app || typeof wallet === 'undefined') {
      return
    }

    if (wallet.deeplink === null) {
      return
    }

    if (wallet.deeplink.native !== null) {
      this._app.connectDeeplink({
        walletName: wallet.name,
        url: wallet.deeplink.native
      })

      this._chosenMobileWalletName = walletName

      triggerConnect(
        wallet.deeplink.native,
        this._app.sessionId,
        this._appInitData.url ?? 'https://nc2.nightly.app'
      )
      return
    }

    if (wallet.deeplink.universal !== null) {
      this._app.connectDeeplink({
        walletName: wallet.name,
        url: wallet.deeplink.universal
      })

      this._chosenMobileWalletName = walletName

      triggerConnect(
        wallet.deeplink.universal,
        this._app.sessionId,
        this._appInitData.url ?? 'https://nc2.nightly.app'
      )
      return
    }
  }
  connectToStandardWallet = async (walletName: string, onSuccess: (acc: AccountInfo) => void) => {
    try {
      if (this._modal) {
        this._modal.setStandardWalletConnectProgress(true)
      }
      const wallet = this.walletsList.find((w) => w.name === walletName)
      if (typeof wallet?.standardWallet === 'undefined') {
        throw new Error('Wallet not found')
      }

      const adapter = wallet.standardWallet as unknown as AdapterPlugin

      const account: AccountInfo = await adapter.connect()

      persistRecentStandardWalletForNetwork(walletName, APTOS_NETWORK)
      persistStandardConnectForNetwork(APTOS_NETWORK)
      this._connectionType = ConnectionType.WalletStandard
      this._innerStandardAdapter = adapter
      this.connected = true
      this._connecting = false

      this._modal?.closeModal()
      onSuccess(account)
    } catch (e) {
      if (this._modal) {
        this._modal.setStandardWalletConnectProgress(false)
      }
      clearRecentStandardWalletForNetwork(APTOS_NETWORK)
      this._connecting = false
      throw e
    }
  }
}
