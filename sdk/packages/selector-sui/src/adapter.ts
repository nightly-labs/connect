/* eslint-disable @typescript-eslint/no-empty-function */
import { publicKeyFromRawBytes } from '@mysten/sui/verify'
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
  ConnectionOptions,
  ConnectionType,
  ISelectedWallet,
  IWalletListItem,
  NightlyConnectSelectorModal,
  WalletMetadata,
  XMLOptions,
  clearRecentWalletForNetwork,
  clearSessionIdForNetwork,
  defaultConnectionOptions,
  getRecentWalletForNetwork,
  isMobileBrowser,
  logoBase64,
  persistRecentWalletForNetwork,
  sleep,
  triggerConnect
} from '@nightlylabs/wallet-selector-base'
import type { StandardEventsChangeProperties, WalletAccount } from '@wallet-standard/core'
import bs58 from 'bs58'
import EventEmitter from 'eventemitter3'
import { getSuiWalletsList } from './detection'

export const convertBase58toBase64 = (base58: string) => {
  const buffer = bs58.decode(base58)
  return buffer.toString('base64')
}

export type SuiAdapterEvents = {
  connect(publicKey: WalletAccount[]): void
  disconnect(): void
  error(error: any): void
  change(properties: StandardEventsChangeProperties): void
}

export class NightlyConnectSuiAdapter extends EventEmitter<SuiAdapterEvents> {
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
  private _walletsList: IWalletListItem[] = []
  private _chosenMobileWalletName: string | undefined
  private _accounts: WalletAccount[] = []
  private _connectionType: ConnectionType | undefined
  private _metadataWallets: WalletMetadata[] = []
  private _selectedWallet: ISelectedWallet | undefined = undefined

  private _connectionOptions: ConnectionOptions = defaultConnectionOptions

  // interval used for checking for wallets with delayed detection
  private _detectionIntervalId: NodeJS.Timeout | undefined
  private _maxNumberOfChecks = 10

  get walletsList() {
    return this._walletsList
  }

  set walletsList(list: IWalletListItem[]) {
    this._walletsList = list
    if (this._modal) {
      this._modal.walletsList = list
    }
  }

  get sessionId() {
    return this._app?.sessionId
  }

  get qrCode() {
    return this._modal?.qrCode
  }

  get selectedWallet() {
    return this._selectedWallet
  }

  // We need internal _connecting since sui messes with connecting state
  private _connecting = false
  constructor(appInitData: AppInitData, connectionOptions?: ConnectionOptions) {
    super()
    this._connecting = false
    this.connecting = false
    this.connected = false
    this._appInitData = appInitData
    if (appInitData.persistent !== false) this._appInitData.persistent = true
    this._loading = false
    this._connectionOptions = { ...this._connectionOptions, ...connectionOptions }

    if (!this._appInitData.persistent) {
      clearSessionIdForNetwork(SUI_NETWORK)
    }
  }

  public static initApp = async (appInitData: AppInitData): Promise<[AppSui, WalletMetadata[]]> => {
    try {
      return await Promise.all([
        AppSui.build(appInitData),
        AppSui.getWalletsMetadata(
          `${appInitData.url ?? 'https://nc2.nightly.app'}/get_wallets_metadata`
        )
      ])
    } catch {
      clearSessionIdForNetwork(SUI_NETWORK)
      return await Promise.all([
        AppSui.build(appInitData),
        AppSui.getWalletsMetadata(
          `${appInitData.url ?? 'https://nc2.nightly.app'}/get_wallets_metadata`
        )
      ])
    }
  }

  public static build = async (
    appInitData: AppInitData,
    connectionOptions?: ConnectionOptions,
    anchorRef?: HTMLElement | null,
    uiOverrides?: {
      variablesOverride?: object
      stylesOverride?: string
      qrConfigOverride?: Partial<XMLOptions>
    }
  ) => {
    const adapter = new NightlyConnectSuiAdapter(appInitData, connectionOptions)

    try {
      adapter.walletsList = getSuiWalletsList(
        [],
        getRecentWalletForNetwork(SUI_NETWORK)?.walletName ?? undefined
      )

      if (!adapter._connectionOptions.disableModal)
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

      adapter.walletsList = getSuiWalletsList(
        metadataWallets,
        getRecentWalletForNetwork(SUI_NETWORK)?.walletName ?? undefined
      )

      adapter.checkForArrivingWallets(metadataWallets)

      // Add event listener for userConnected
      app.on('userConnected', async (e) => {
        try {
          persistRecentWalletForNetwork(SUI_NETWORK, {
            walletName: adapter._chosenMobileWalletName || '',
            walletType: ConnectionType.Nightly
          })

          if (!adapter._app || adapter._app.connectedPublicKeys.length <= 0) {
            adapter.connected = false
            // If user does not pass any accounts, we should disconnect
            adapter.disconnect()
            return
          }
          adapter.setSelectedWallet({ isRemote: true })
          adapter._accounts = e.publicKeys.map((pk) => createSuiWalletAccountFromString(pk))
          adapter.connected = true
          adapter.emit('connect', adapter._accounts)
        } catch {
          adapter.disconnect()
        }
      })
    } catch {
      console.log('Error building adapter')
    }

    return adapter
  }

  public static buildLazy = (
    appInitData: AppInitData,
    connectionOptions?: ConnectionOptions,
    anchorRef?: HTMLElement | null,
    uiOverrides?: {
      variablesOverride?: object
      stylesOverride?: string
      qrConfigOverride?: Partial<XMLOptions>
    }
  ) => {
    const adapter = new NightlyConnectSuiAdapter(appInitData, connectionOptions)

    adapter.walletsList = getSuiWalletsList(
      [],
      getRecentWalletForNetwork(SUI_NETWORK)?.walletName ?? undefined
    )

    // Fetch wallets from registry
    adapter.fetchWalletsFromRegistry().then((metadataWallets) => {
      adapter._metadataWallets = metadataWallets

      adapter.walletsList = getSuiWalletsList(
        metadataWallets,
        getRecentWalletForNetwork(SUI_NETWORK)?.walletName ?? undefined
      )
    })

    if (!adapter._connectionOptions.disableModal)
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

    if (!adapter._connectionOptions.initOnConnect) {
      adapter._loading = true

      NightlyConnectSuiAdapter.initApp(appInitData)
        .then(([app, metadataWallets]) => {
          adapter._app = app
          adapter._metadataWallets = metadataWallets

          adapter.walletsList = getSuiWalletsList(
            metadataWallets,
            getRecentWalletForNetwork(SUI_NETWORK)?.walletName ?? undefined
          )

          adapter.checkForArrivingWallets(metadataWallets)

          app.on('userConnected', async (e) => {
            try {
              persistRecentWalletForNetwork(SUI_NETWORK, {
                walletName: adapter._chosenMobileWalletName || '',
                walletType: ConnectionType.Nightly
              })

              if (!adapter._app || adapter._app.connectedPublicKeys.length <= 0) {
                adapter.connected = false
                // If user does not pass any accounts, we should disconnect
                adapter.disconnect()
                return
              }
              adapter.setSelectedWallet({ isRemote: true })
              adapter._accounts = e.publicKeys.map((pk) => createSuiWalletAccountFromString(pk))
              adapter.connected = true
              adapter.emit('connect', adapter._accounts)
            } catch {
              adapter.disconnect()
            }
          })

          adapter._loading = false
        })
        .catch(() => {
          adapter._loading = false
          throw new Error('Failed to initialize adapter')
        })
    }

    return adapter
  }

  connect = async () => {
    return new Promise<void>((resolve, reject) => {
      const innerConnect = async () => {
        try {
          if (this._connecting) {
            reject('Cannot connect while connecting')
            return
          }

          if (this.connected) {
            resolve()
            return
          }

          const recentWallet = getRecentWalletForNetwork(SUI_NETWORK)
          if (!this._connectionOptions.disableEagerConnect && recentWallet !== null) {
            // Eager connect standard if possible
            if (recentWallet.walletType === ConnectionType.WalletStandard) {
              await this.connectToStandardWallet(recentWallet.walletName)
              resolve()
              return
            }

            // Eager connect remote if possible
            if (recentWallet.walletType === ConnectionType.Nightly) {
              if (this._app?.hasBeenRestored() && this._app.connectedPublicKeys.length > 0) {
                // Try to eager connect if session is restored
                try {
                  // TODO add support for Secp256k1 key and features detection
                  this._accounts = this._app.connectedPublicKeys.map((pk) =>
                    createSuiWalletAccountFromString(pk)
                  )
                  this.connected = true
                  this._connecting = false
                  this.connecting = false
                  this._connectionType = ConnectionType.Nightly
                  this.setSelectedWallet({ isRemote: true })
                  // we only run it to reassign deeplinks on eager connect (not on qr connection)
                  if (isMobileBrowser()) {
                    const wallet = this.walletsList.find((w) => w.name === 'Nightly')
                    if (wallet?.mobile) {
                      // If we have a native deeplink, we should use it
                      if (wallet.mobile?.native !== null) {
                        this._app.connectDeeplink({
                          walletName: wallet.name,
                          url: wallet.mobile.native
                        })
                      }
                      // If we have a universal deeplink, we should use it
                      else if (wallet.mobile?.universal !== null) {
                        this._app.connectDeeplink({
                          walletName: wallet.name,
                          url: wallet.mobile.universal
                        })
                      }
                    }
                  }
                  this.emit('connect', this._accounts)
                  resolve()
                  return
                } catch (error) {
                  // If we fail because of whatever reason
                  // Reset session since it might be corrupted
                  const [app] = await NightlyConnectSuiAdapter.initApp(this._appInitData)
                  this._app = app
                }
              }
            }
          }

          if (this._connectionOptions.disableModal) {
            reject('Modal is disabled')
            return
          }

          if (this._connectionOptions.initOnConnect) {
            this._loading = true
            NightlyConnectSuiAdapter.initApp(this._appInitData)
              .then(([app, metadataWallets]) => {
                this._app = app
                this._metadataWallets = metadataWallets
                this.walletsList = getSuiWalletsList(
                  metadataWallets,
                  getRecentWalletForNetwork(SUI_NETWORK)?.walletName ?? undefined
                )

                this.checkForArrivingWallets(metadataWallets)

                // Add event listener for userConnected
                app.on('userConnected', async (e) => {
                  try {
                    persistRecentWalletForNetwork(SUI_NETWORK, {
                      walletName: this._chosenMobileWalletName || '',
                      walletType: ConnectionType.Nightly
                    })

                    if (!this._app || this._app.connectedPublicKeys.length <= 0) {
                      this.connected = false
                      // If user does not pass any accounts, we should disconnect
                      this.disconnect()
                      return
                    }
                    this.setSelectedWallet({ isRemote: true })
                    this._accounts = e.publicKeys.map((pk) => createSuiWalletAccountFromString(pk))
                    this.connected = true
                    this.emit('connect', this._accounts)
                  } catch {
                    this.disconnect()
                  }
                })
                this._loading = false
              })
              .catch(() => {
                this._loading = false
                throw new Error('Failed to initialize adapter')
              })
          }

          // Interval that checks if app has connected

          if (!this._modal) {
            this._connecting = false
            this.connecting = false
            const error = new Error('Wallet not ready')
            this.emit('error', error)
            reject(error)
            return
          }

          // modal is defined here
          this._connecting = true
          this.connecting = true
          this._modal.onClose = () => {
            if (this._connecting) {
              this._connecting = false
              this.connecting = false
              const error = new Error('Connection cancelled')
              this.emit('error', error)
              reject(error)
              return
            }
          }

          // Try open
          const opened = this._modal!.openModal(
            this._app?.sessionId ?? undefined,
            async (walletName) => {
              if (
                isMobileBrowser() &&
                !this.walletsList.find((w) => w.name === walletName)?.standardWallet
              ) {
                this.connectToMobileWallet(walletName)
              } else {
                try {
                  await this.connectToStandardWallet(walletName)
                  resolve()
                } catch (error) {
                  reject(error)
                  return
                }
              }
            }
          )

          // If modal is not opened, reject
          // This might be caused by SSR
          if (!opened) {
            this._connecting = false
            this.connecting = false
            const error = new Error('Failed to open modal')
            this.emit('error', error)
            reject(error)
          }

          // loop until app is connected or we timeout
          let checks = 0
          const loadingInterval: NodeJS.Timeout = setInterval(async (): Promise<void> => {
            checks++
            if (this._app) {
              // Clear interval if app is connected
              clearInterval(loadingInterval)
              if (this._modal) this._modal.sessionId = this._app.sessionId

              this._app.on('userConnected', async () => {
                try {
                  if (!this._app || this._app.connectedPublicKeys.length <= 0) {
                    reject(new Error('No accounts found'))
                  }
                  this.connected = true
                  this._connecting = false
                  this.connecting = false
                  this._connectionType = ConnectionType.Nightly
                  this._modal?.closeModal()
                  resolve()
                } catch (error) {
                  reject(error)
                } finally {
                  this._connecting = false
                }
              })
              return
            }
            // timeout after 5 seconds
            if (checks > 500) {
              clearInterval(loadingInterval)
              // reject(new Error('Connecting takes too long'))
              if (this._modal) this._modal.timeoutError = 'Connecting is taking too long'
            }
          }, 10)
        } catch (error: unknown) {
          this._connecting = false
          this.connecting = false
          this.emit('error', error)
          reject(error)
        }
      }

      innerConnect()
    })
  }
  disconnect = async () => {
    if (this.connected) {
      switch (this._connectionType) {
        case ConnectionType.Nightly: {
          clearSessionIdForNetwork(SUI_NETWORK)
          // Refresh app session
          this._app = await AppSui.build(this._appInitData)
          this._selectedWallet = undefined

          // Add event listener for userConnected
          this._app.on('userConnected', async (e) => {
            try {
              persistRecentWalletForNetwork(SUI_NETWORK, {
                walletName: this._chosenMobileWalletName || '',
                walletType: ConnectionType.Nightly
              })

              if (!this._app || this._app.connectedPublicKeys.length <= 0) {
                this.connected = false

                // If user does not pass any accounts, we should disconnect
                this.disconnect()
                return
              }
              this.setSelectedWallet({ isRemote: true })
              this._accounts = e.publicKeys.map((pk) => createSuiWalletAccountFromString(pk))
              this.connected = true
              this.emit('connect', this._accounts)
            } catch {
              this.disconnect()
            }
          })

          break
        }
        case ConnectionType.WalletStandard: {
          if (this._innerStandardAdapter) {
            await this._innerStandardAdapter.disconnect()
            clearRecentWalletForNetwork(SUI_NETWORK)
            this.walletsList = getSuiWalletsList(
              this._metadataWallets,
              getRecentWalletForNetwork(SUI_NETWORK)?.walletName ?? undefined
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
      this.emit('disconnect')
    }
  }

  signPersonalMessage: SuiSignPersonalMessageMethod = async (messageInput) => {
    if (!this._app || !this._connectionType) {
      const error = new Error('Wallet not ready')
      this.emit('error', error)
      throw error
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

  signTransactionBlock: SuiSignTransactionBlockMethod = async (transactionInput) => {
    if (!this._app || !this._connectionType) {
      const error = new Error('Wallet not ready')
      this.emit('error', error)
      throw error
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
      const error = new Error('Wallet not ready')
      this.emit('error', error)
      throw error
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
    // If eager connect is disabled, we can't eager connect
    if (this._connectionOptions.disableEagerConnect) return false

    // Get recent wallet for network
    const recentWallet = getRecentWalletForNetwork(SUI_NETWORK)

    // If there is no recent wallet, we can't eager connect
    if (recentWallet === null) return false

    // If we use wallet standard, we can eager connect
    if (
      recentWallet.walletName !== null &&
      recentWallet.walletType === ConnectionType.WalletStandard
    ) {
      return true
    }

    // If we use nightly connect we need to make sure app is restored
    if (recentWallet.walletType === ConnectionType.Nightly) {
      if (this._connectionOptions.initOnConnect) {
        return false
      }

      // Wait for app to be restored
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

      // If app is restored and has connected public keys, we can eager connect

      if (this._app && this._app.hasBeenRestored() && this._app.connectedPublicKeys.length > 0) {
        return true
      }
    }

    return false
  }

  connectToMobileWallet = (walletName: string) => {
    try {
      if (this._modal) {
        this._modal.setStandardWalletConnectProgress(true)
      }

      const wallet = this.walletsList.find((w) => w.name === walletName)
      this.setSelectedWallet({ wallet })

      if (!this._app) {
        throw new Error('Wallet not ready')
      }

      if (typeof wallet === 'undefined') {
        throw new Error('Wallet not found')
      }

      if (wallet.mobile === null) {
        throw new Error('Deeplink not found')
      }

      // If we have a native deeplink, we should use it
      if (wallet.mobile.native !== null) {
        this._app.connectDeeplink({
          walletName: wallet.name,
          url: wallet.mobile.native
        })

        this._chosenMobileWalletName = walletName

        triggerConnect(
          wallet.mobile.native,
          this._app.sessionId,
          this._appInitData.url ?? 'https://nc2.nightly.app'
        )
        return
      }

      // If we have a universal deeplink, we should use it
      if (wallet.mobile.universal !== null) {
        this._app.connectDeeplink({
          walletName: wallet.name,
          url: wallet.mobile.universal
        })

        this._chosenMobileWalletName = walletName

        triggerConnect(
          wallet.mobile.universal,
          this._app.sessionId,
          this._appInitData.url ?? 'https://nc2.nightly.app'
        )
        return
      }
    } catch (err) {
      clearRecentWalletForNetwork(SUI_NETWORK)
      if (this._modal) {
        this._modal.setStandardWalletConnectProgress(false)
      }
      throw err
    }
  }

  connectToStandardWallet = async (walletName: string) => {
    try {
      if (this._modal) {
        this._modal.setStandardWalletConnectProgress(true)
      }

      const wallet = this.walletsList.find((w) => w.name === walletName)
      this.setSelectedWallet({ wallet })

      if (typeof wallet?.standardWallet === 'undefined') {
        if (this._modal) {
          this._modal.setStandardWalletConnectProgress(false)
        }
        throw new Error('Wallet not found')
      }

      const adapter = new StandardWalletAdapter({
        wallet: wallet.standardWallet
      } as StandardWalletAdapterConfig)

      await adapter.connect()

      this._connectionType = ConnectionType.WalletStandard
      this._innerStandardAdapter = adapter
      this._accounts = (await adapter.getAccounts()).map((a) => a)
      this.connected = true
      this._connecting = false
      this.connecting = false
      this.emit('connect', this._accounts)

      // Subscribe to change event
      adapter.wallet.features['standard:events'].on('change', (a) => {
        this.emit('change', a)
      })

      persistRecentWalletForNetwork(SUI_NETWORK, {
        walletName,
        walletType: ConnectionType.WalletStandard
      })

      this._modal?.closeModal()
    } catch (err) {
      if (this._modal) {
        this._modal.setStandardWalletConnectProgress(false)
      }
      clearRecentWalletForNetwork(SUI_NETWORK)
      this._connecting = false
      this.connecting = false
      throw err
    }
  }

  connectToWallet = async (walletName: string) => {
    if (isMobileBrowser() && !this.walletsList.find((w) => w.name === walletName)?.standardWallet) {
      this.connectToMobileWallet(walletName)
    } else {
      await this.connectToStandardWallet(walletName)
    }
  }

  fetchWalletsFromRegistry: () => Promise<WalletMetadata[]> = async () => {
    this._metadataWallets = await AppSui.getWalletsMetadata(
      `${this._appInitData.url ?? 'https://nc2.nightly.app'}/get_wallets_metadata`
    )
    return this._metadataWallets
  }

  fetchAllWallets = async () => {
    const metadataWallets = await this.fetchWalletsFromRegistry()
    this.walletsList = getSuiWalletsList(
      metadataWallets,
      getRecentWalletForNetwork(SUI_NETWORK)?.walletName ?? undefined
    )
    return this.walletsList
  }

  checkForArrivingWallets = (metadataWallets: WalletMetadata[]) => {
    clearInterval(this._detectionIntervalId)
    let checks = 0

    this._detectionIntervalId = setInterval(() => {
      if (checks >= this._maxNumberOfChecks || this.connected) {
        clearInterval(this._detectionIntervalId)
      }
      checks++
      this.walletsList = getSuiWalletsList(
        metadataWallets,
        getRecentWalletForNetwork(SUI_NETWORK)?.walletName ?? undefined
      )
    }, 500)
  }

  setSelectedWallet = ({
    wallet,
    isRemote = false
  }: {
    wallet?: IWalletListItem
    isRemote?: boolean
  }) => {
    if (!wallet) {
      // Connecting to the nightly mobile app
      wallet = this.walletsList.find((wallet) => wallet.name === 'Nightly')
    }

    if (wallet) {
      this._selectedWallet = {
        name: wallet.name,
        image: wallet.image,
        homepage: wallet.homepage,
        walletType: isRemote ? 'mobile' : wallet.walletType
      }
    }
  }
}

export const createSuiWalletAccountFromString = (publicKey: string): WalletAccount => {
  const suiPk = publicKeyFromRawBytes('ED25519', bs58.decode(publicKey))
  return {
    address: suiPk.toSuiAddress(),
    publicKey: suiPk.toRawBytes(),
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
