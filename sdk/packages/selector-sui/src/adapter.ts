/* eslint-disable @typescript-eslint/no-empty-function */
import { publicKeyFromRawBytes } from '@mysten/sui.js/verify'
import { StandardWalletAdapter } from '@mysten/wallet-adapter-wallet-standard'
import type {
  SuiSignAndExecuteTransactionBlockMethod,
  SuiSignPersonalMessageMethod,
  SuiSignTransactionBlockMethod
} from '@mysten/wallet-standard'

import { type StandardWalletAdapterConfig } from '@mysten/wallet-adapter-wallet-standard/dist/StandardWalletAdapter'
import { SUI_CHAINS } from '@mysten/wallet-standard'
import { AppSui, SUI_NETWORK } from '@nightlylabs/nightly-connect-sui'
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
  logoBase64,
  persistRecentStandardWalletForNetwork,
  persistStandardConnectForNetwork,
  persistStandardDisconnectForNetwork,
  sleep,
  triggerConnect
} from '@nightlylabs/wallet-selector-base'
import type { StandardEventsOnMethod, WalletAccount } from '@wallet-standard/core'
import bs58 from 'bs58'
import { suiWalletsFilter } from './detection'

export const convertBase58toBase64 = (base58: string) => {
  const buffer = bs58.decode(base58)
  return buffer.toString('base64')
}
export class NightlyConnectSuiAdapter {
  // TODO: add later "implements WalletAdapter"
  name = 'Nightly Connect' as const
  icon = logoBase64
  connected = false
  connecting = false
  // Nightly connect fields
  private _app: AppSui | undefined
  private _innerStandardAdapter: StandardWalletAdapter | undefined
  private _loading = false
  private _modal: NightlyConnectSelectorModal | undefined
  private _appInitData: AppInitData
  private _eagerConnectForStandardWallets = true
  private _walletsList: IWalletListItem[] = []
  private _chosenMobileWalletName: string | undefined
  private _accounts: WalletAccount[] = []
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
    this.connecting = false
    this.connected = false
    this._appInitData = appInitData
    this._eagerConnectForStandardWallets = eagerConnectForStandardWallets ?? true
    this._loading = false
    this._initOnConnect = initOnConnect
  }

  on: StandardEventsOnMethod = (event, listener) => {
    if (!this._innerStandardAdapter) {
      console.warn('Only supported on standard wallet')
      return () => {}
    }
    return this._innerStandardAdapter.on(event, listener)
  }

  public static initApp = async (appInitData: AppInitData): Promise<[AppSui, MetadataWallet[]]> => {
    try {
      return await Promise.all([
        AppSui.build(appInitData),
        AppSui.getWalletsMetadata(
          `${appInitData.url ?? 'https://nc2.nightly.app'}/get_wallets_metadata`
        )
          .then((list) =>
            list.map((wallet) => ({
              slug: wallet.slug,
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
      clearSessionIdForNetwork(SUI_NETWORK)
      return await Promise.all([
        AppSui.build(appInitData),
        AppSui.getWalletsMetadata(
          `${appInitData.url ?? 'https://nc2.nightly.app'}/get_wallets_metadata`
        )
          .then((list) =>
            list.map((wallet) => ({
              slug: wallet.slug,
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
    const adapter = new NightlyConnectSuiAdapter(appInitData, eagerConnectForStandardWallets)

    adapter.walletsList = getWalletsList(
      [],
      suiWalletsFilter,
      getRecentStandardWalletForNetwork(SUI_NETWORK) ?? undefined
    )
    adapter._modal = new NightlyConnectSelectorModal(
      adapter.walletsList,
      appInitData.url ?? 'https://nc2.nightly.app',
      {
        name: SUI_NETWORK,
        icon: 'https://registry.nightly.app/networks/sui.png'
      },
      anchorRef,
      uiOverrides?.variablesOverride,
      uiOverrides?.stylesOverride,
      uiOverrides?.qrConfigOverride
    )

    const [app, metadataWallets] = await NightlyConnectSuiAdapter.initApp(appInitData)

    adapter._app = app
    adapter._metadataWallets = metadataWallets
    adapter.walletsList = getWalletsList(
      metadataWallets,
      suiWalletsFilter,
      getRecentStandardWalletForNetwork(SUI_NETWORK) ?? undefined
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
    const adapter = new NightlyConnectSuiAdapter(appInitData, eagerConnectForStandardWallets)

    adapter.walletsList = getWalletsList(
      [],
      suiWalletsFilter,
      getRecentStandardWalletForNetwork(SUI_NETWORK) ?? undefined
    )
    adapter._modal = new NightlyConnectSelectorModal(
      adapter.walletsList,
      appInitData.url ?? 'https://nc2.nightly.app',
      {
        name: SUI_NETWORK,
        icon: 'https://registry.nightly.app/networks/sui.png'
      },
      anchorRef,
      uiOverrides?.variablesOverride,
      uiOverrides?.stylesOverride,
      uiOverrides?.qrConfigOverride
    )

    adapter._loading = true

    NightlyConnectSuiAdapter.initApp(appInitData).then(([app, metadataWallets]) => {
      adapter._app = app
      adapter._metadataWallets = metadataWallets
      adapter.walletsList = getWalletsList(
        metadataWallets,
        suiWalletsFilter,
        getRecentStandardWalletForNetwork(SUI_NETWORK) ?? undefined
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
    const adapter = new NightlyConnectSuiAdapter(appInitData, eagerConnectForStandardWallets, true)

    adapter.walletsList = getWalletsList(
      [],
      suiWalletsFilter,
      getRecentStandardWalletForNetwork(SUI_NETWORK) ?? undefined
    )

    adapter._modal = new NightlyConnectSelectorModal(
      adapter.walletsList,
      appInitData.url ?? 'https://nc2.nightly.app',
      {
        name: SUI_NETWORK,
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
    return new Promise<void>((resolve, reject) => {
      const innerConnect = async () => {
        try {
          if (this.connected || this._connecting) {
            resolve()
            return
          }

          if (this._initOnConnect) {
            this._connecting = true
            this.connecting = true

            if (!this._app) {
              try {
                const [app, metadataWallets] = await NightlyConnectSuiAdapter.initApp(
                  this._appInitData
                )

                this._app = app
                this._metadataWallets = metadataWallets
                this.walletsList = getWalletsList(
                  metadataWallets,
                  suiWalletsFilter,
                  getRecentStandardWalletForNetwork(SUI_NETWORK) ?? undefined
                )
              } catch {
                if (!this._app) {
                  this._connecting = false
                  this.connecting = false
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
            this.connecting = true
          }

          if (this._app.hasBeenRestored() && this._app.connectedPublicKeys.length > 0) {
            // Try to eager connect if session is restored
            try {
              this.eagerConnectDeeplink()
              // TODO add support for Secp256k1 key and features detection
              this._accounts = this._app.connectedPublicKeys.map((pk) =>
                createSuiWalletAccountFromString(pk)
              )
              this.connected = true
              this._connecting = false
              this.connecting = false
              this._connectionType = ConnectionType.Nightly
              resolve()
              return
            } catch (error) {
              // If we fail because of whatever reason
              // Reset session since it might be corrupted
              const [app] = await NightlyConnectSuiAdapter.initApp(this._appInitData)
              this._app = app
            }
          }
          const recentName = getRecentStandardWalletForNetwork(SUI_NETWORK)
          if (
            this._eagerConnectForStandardWallets &&
            recentName !== null &&
            isStandardConnectedForNetwork(SUI_NETWORK)
          ) {
            await this.connectToStandardWallet(recentName, resolve)
            return
          }

          this._app.on('userConnected', (e) => {
            try {
              if (this._chosenMobileWalletName) {
                persistRecentStandardWalletForNetwork(this._chosenMobileWalletName, SUI_NETWORK)
              } else {
                clearRecentStandardWalletForNetwork(SUI_NETWORK)
              }
              this._accounts = e.publicKeys.map((pk) => createSuiWalletAccountFromString(pk))
              this.connected = true
              this._connecting = false
              this.connecting = false
              this._connectionType = ConnectionType.Nightly
              this._modal?.closeModal()
              resolve()
            } catch (e) {
              this.disconnect()
              this._modal?.closeModal()
              reject(e)
            }
          })
          if (!this._modal) {
            this._connecting = false
            this.connecting = false
            reject(new Error('Wallet not ready'))
          }
          // _modal is defined here
          this._modal!.onClose = () => {
            if (this._connecting) {
              this._connecting = false
              this.connecting = false
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
            this.connecting = false
            const error = new Error('Failed to open modal')
            reject(error)
          }
        } catch (error: unknown) {
          this._connecting = false
          this.connecting = false

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
        clearSessionIdForNetwork(SUI_NETWORK)
        // Refresh app session
        this._app = await AppSui.build(this._appInitData)

        break
      }
      case ConnectionType.WalletStandard: {
        if (this._innerStandardAdapter) {
          await this._innerStandardAdapter.disconnect()
          persistStandardDisconnectForNetwork(SUI_NETWORK)
          this.walletsList = getWalletsList(
            this._metadataWallets,
            suiWalletsFilter,
            getRecentStandardWalletForNetwork(SUI_NETWORK) ?? undefined
          )
        }
        break
      }
    }
    this._innerStandardAdapter = undefined
    this._connectionType = undefined
    this.connected = false
    this._connecting = false
    this.connecting = false
  }

  signPersonalMessage: SuiSignPersonalMessageMethod = async (messageInput) => {
    if (!this._app || !this._connectionType) {
      throw new Error('Wallet not ready')
    }
    switch (this._connectionType) {
      case ConnectionType.Nightly: {
        const message = await this._app.signMessage(messageInput)
        return {
          bytes: message.messageBytes,
          signature: message.signature
        }
      }
      case ConnectionType.WalletStandard: {
        if (!this._innerStandardAdapter) {
          throw new Error('Wallet not ready')
        }
        return await this._innerStandardAdapter.signPersonalMessage(messageInput)
      }
    }
  }

  //
  signTransactionBlock: SuiSignTransactionBlockMethod = async (transactionInput) => {
    if (!this._app || !this._connectionType) {
      throw new Error('Wallet not ready')
    }
    switch (this._connectionType) {
      case ConnectionType.Nightly: {
        return await this._app.signTransactionBlock(transactionInput)
        // return { bytes: res.transactionBlockBytes, signature: res.signature }
      }
      case ConnectionType.WalletStandard: {
        if (!this._innerStandardAdapter) {
          throw new Error('Wallet not ready')
        }
        // @ts-expect-error(remove after standard will use 0.42)
        return await this._innerStandardAdapter.signTransactionBlock(transactionInput)
      }
    }
  }

  signAndExecuteTransactionBlock: SuiSignAndExecuteTransactionBlockMethod = async (
    transactionInput
  ) => {
    if (!this._app || !this._connectionType) {
      throw new Error('Wallet not ready')
    }
    switch (this._connectionType) {
      case ConnectionType.Nightly: {
        return await this._app.signAndExecuteTransactionBlock(transactionInput)
      }
      case ConnectionType.WalletStandard: {
        if (!this._innerStandardAdapter) {
          throw new Error('Wallet not ready')
        }
        // @ts-expect-error(remove after standard will use 0.42)
        return await this._innerStandardAdapter.signAndExecuteTransactionBlock(transactionInput)
      }
    }
  }

  getAccounts = async (): Promise<readonly WalletAccount[]> => {
    return this._accounts
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
      getRecentStandardWalletForNetwork(SUI_NETWORK) !== null &&
      isStandardConnectedForNetwork(SUI_NETWORK)
    ) {
      return true
    }

    return false
  }

  eagerConnectDeeplink = () => {
    if (isMobileBrowser() && this._app) {
      const mobileWalletName = getRecentStandardWalletForNetwork(SUI_NETWORK)
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
  connectToStandardWallet = async (walletName: string, onSuccess: () => void) => {
    try {
      if (this._modal) {
        this._modal.setStandardWalletConnectProgress(true)
      }
      const wallet = this.walletsList.find((w) => w.name === walletName)
      if (typeof wallet?.standardWallet === 'undefined') {
        throw new Error('Wallet not found')
      }

      const adapter = new StandardWalletAdapter({
        wallet: wallet.standardWallet
      } as StandardWalletAdapterConfig)

      await adapter.connect()

      persistRecentStandardWalletForNetwork(walletName, SUI_NETWORK)
      persistStandardConnectForNetwork(SUI_NETWORK)
      this._connectionType = ConnectionType.WalletStandard
      this._innerStandardAdapter = adapter
      this._accounts = (await adapter.getAccounts()).map((a) => a)
      this.connected = true
      this._connecting = false
      this.connecting = false

      this._modal?.closeModal()
      onSuccess()
    } catch (e) {
      if (this._modal) {
        this._modal.setStandardWalletConnectProgress(false)
      }
      clearRecentStandardWalletForNetwork(SUI_NETWORK)
      this._connecting = false
      this.connecting = false
      throw e
    }
  }
}
export const createSuiWalletAccountFromString = (publicKey: string): WalletAccount => {
  const suiPk = publicKeyFromRawBytes('ED25519', bs58.decode(publicKey))
  return {
    address: suiPk.toSuiAddress(),
    publicKey: suiPk.toBytes(),
    chains: SUI_CHAINS,
    features: [
      'standard:connect',
      'standard:events',
      'sui:signTransactionBlock',
      'sui:signAndExecuteTransactionBlock',
      'sui:signMessage'
    ]
  }
}
