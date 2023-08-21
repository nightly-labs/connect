/* eslint-disable @typescript-eslint/no-empty-function */
import { AppPolkadot, AppPolkadotInitialize } from '@nightlylabs/nightly-connect-polkadot'
import {
  MetadataWallet,
  NightlyConnectSelectorModal,
  clearRecentStandardWalletForNetwork,
  clearSessionIdForNetwork,
  getRecentStandardWalletForNetwork,
  isMobileBrowser,
  isStandardConnectedForNetwork,
  logoBase64,
  persistRecentStandardWalletForNetwork,
  persistStandardConnectForNetwork,
  persistStandardDisconnectForNetwork,
  sleep,
  triggerConnect
} from '@nightlylabs/wallet-selector-base'

import { type Signer as InjectedSigner } from '@polkadot/api/types'
import { InjectedAccounts, type Injected } from '@polkadot/extension-inject/types'
import { IPolkadotWalletListItem, getPolkadotWalletsList } from './detection'
import { networkToData, SupportedNetworks } from './utils'
export type AppSelectorInitialize = Omit<AppPolkadotInitialize, 'network'> & {
  network: SupportedNetworks
}
export class NightlyConnectAdapter implements Injected {
  name = 'Nightly Connect'
  url = 'https://nightly.app'
  icon = logoBase64

  private _connecting: boolean
  private _connected: boolean

  private _app: AppPolkadot | undefined
  private _appSessionActive: boolean
  private _innerStandardAdapter: Injected | undefined
  private _modal: NightlyConnectSelectorModal | undefined

  private _appInitData: AppSelectorInitialize
  private _useEagerConnect: boolean

  private _metadataWallets: MetadataWallet[] = []
  private _walletsList: IPolkadotWalletListItem[] = []

  private _chosenMobileWalletName: string | undefined

  private _loading: boolean

  constructor(appInitData: AppSelectorInitialize, useEagerConnect?: boolean) {
    this._connecting = false
    this._connected = false
    this._appInitData = appInitData
    this._useEagerConnect = !!useEagerConnect
    this._appSessionActive = false
    this._loading = false
  }

  get accounts(): InjectedAccounts {
    // Means that we are connected via standard wallet
    if (this._innerStandardAdapter) {
      return this._innerStandardAdapter.accounts
    }
    // Fall back to Nightly Connect
    if (this._app) {
      return this._app.accounts
    }

    return {
      get: async () => [],
      subscribe: () => {
        return () => {}
      }
    }
  }
  get signer(): InjectedSigner {
    if (!this.connected) {
      throw new Error('Not connected')
    }
    // Means that we are connected via standard wallet
    if (this._innerStandardAdapter) {
      return this._innerStandardAdapter.signer
    }
    // Fall back to Nightly Connect
    if (this._app) {
      return this._app.signer
    }
    throw new Error('App not initialized')
  }
  get connecting() {
    return this._connecting
  }

  get connected() {
    return this._connected
  }
  get network() {
    return this._appInitData.network
  }
  get walletsList() {
    return this._walletsList
  }

  set walletsList(list: IPolkadotWalletListItem[]) {
    this._walletsList = list
    if (this._modal) {
      this._modal.walletsList = list
    }
  }
  public static build = async (
    appInitData: AppSelectorInitialize,
    useEagerConnect?: boolean,
    anchorRef?: HTMLElement | null
  ) => {
    if (!useEagerConnect) {
      clearSessionIdForNetwork(appInitData.network)
    }

    const adapter = new NightlyConnectAdapter(appInitData, useEagerConnect)

    adapter.walletsList = getPolkadotWalletsList(
      [],
      getRecentStandardWalletForNetwork(adapter.network) ?? undefined
    )
    adapter._modal = new NightlyConnectSelectorModal(
      adapter.walletsList,
      appInitData.url ?? 'https://nc2.nightly.app',
      networkToData(adapter.network),
      anchorRef
    )

    const [app, metadataWallets] = await Promise.all([
      AppPolkadot.build(appInitData),
      AppPolkadot.getWalletsMetadata('https://nc2.nightly.app/get_wallets_metadata')
        .then((list) =>
          list.map((wallet) => ({
            name: wallet.name,
            icon: wallet.image.default,
            deeplink: wallet.mobile,
            link: wallet.homepage
          }))
        )
        .catch(() => [] as MetadataWallet[])
    ])

    adapter._app = app
    adapter._metadataWallets = metadataWallets

    adapter.walletsList = getPolkadotWalletsList(
      metadataWallets,
      getRecentStandardWalletForNetwork(adapter.network) ?? undefined
    )

    return adapter
  }

  public static buildLazy = (
    appInitData: AppSelectorInitialize,
    useEagerConnect?: boolean,
    anchorRef?: HTMLElement | null
  ) => {
    if (!useEagerConnect) {
      clearSessionIdForNetwork(appInitData.network)
    }

    const adapter = new NightlyConnectAdapter(appInitData, useEagerConnect)

    adapter.walletsList = getPolkadotWalletsList(
      [],
      getRecentStandardWalletForNetwork(adapter.network) ?? undefined
    )
    adapter._modal = new NightlyConnectSelectorModal(
      adapter.walletsList,
      appInitData.url ?? 'https://nc2.nightly.app',
      networkToData(adapter.network),
      anchorRef
    )

    adapter._loading = true

    Promise.all([
      AppPolkadot.build(appInitData),
      AppPolkadot.getWalletsMetadata('https://nc2.nightly.app/get_wallets_metadata')
        .then((list) =>
          list.map((wallet) => ({
            name: wallet.name,
            icon: wallet.image.default,
            deeplink: wallet.mobile,
            link: wallet.homepage
          }))
        )
        .catch(() => [] as MetadataWallet[])
    ]).then(([app, metadataWallets]) => {
      adapter._app = app
      adapter._metadataWallets = metadataWallets
      adapter.walletsList = getPolkadotWalletsList(
        metadataWallets,
        getRecentStandardWalletForNetwork(adapter.network) ?? undefined
      )

      adapter._loading = false
    })

    return adapter
  }
  // ensureLoaded = async () => {}
  canEagerConnect = async () => {
    if (!this._useEagerConnect) {
      return false
    }

    // utility for case if somebody wants to fire connect asap, but doesn't want to show modal if e. g. there was no user connected on the device yet
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

    if (this._app && this._app.hasBeenRestored() && this._app.accounts.activeAccounts.length > 0) {
      return true
    }

    if (
      getRecentStandardWalletForNetwork(this.network) !== null &&
      isStandardConnectedForNetwork(this.network)
    ) {
      return true
    }

    return false
  }

  eagerConnectDeeplink = () => {
    if (isMobileBrowser() && this._app) {
      const mobileWalletName = getRecentStandardWalletForNetwork(this.network)
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
    if (this._modal) {
      this._modal.setStandardWalletConnectProgress(true)
    }
    const adapter = this.walletsList.find((w) => w.name === walletName)?.injectedWallet
    if (typeof adapter === 'undefined') {
      if (this._modal) {
        this._modal.setStandardWalletConnectProgress(false)
      }
      throw new Error('Wallet not found')
    }
    try {
      // @ts-expect-error we want to pass network to enable
      const inject = await adapter!.enable!('Nightly Connect', this.network) // TODO should we also use connect?

      persistRecentStandardWalletForNetwork(walletName, this.network)
      persistStandardConnectForNetwork(this.network)
      this._innerStandardAdapter = {
        ...inject,
        signer: {
          ...inject.signer,
          signPayload: inject.signer.signPayload
            ? // @ts-expect-error We want to also pass network to signPayload
              (payload) => inject.signer.signPayload!(payload, this.network)
            : undefined,
          signRaw: inject.signer.signRaw
            ? // @ts-expect-error We want to also pass network to signPayload
              (payload) => inject.signer.signRaw!(payload, this.network)
            : undefined
        }
      }
      this._connected = true
      this._connecting = false
      this._modal?.closeModal()
      onSuccess()
    } catch {
      if (this._modal) {
        this._modal.setStandardWalletConnectProgress(false)
      }
    }
  }

  connect = async () =>
    new Promise<void>((resolve, reject) => {
      const innerConnect = async () => {
        try {
          if (this._loading) {
            // we do it to ensure proper connect flow in case if adapter is lazily built, but e. g. solana wallets selector uses its own eager connect
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

          if (this.connected || this.connecting) {
            resolve()
            return
          }

          this._connecting = true
          if (this._app.hasBeenRestored() && this._app.accounts.activeAccounts.length > 0) {
            this.eagerConnectDeeplink()
            this._connected = true
            this._connecting = false
            this._appSessionActive = true
            resolve()
            return
          }

          const recentName = getRecentStandardWalletForNetwork(this.network)
          if (
            this._useEagerConnect &&
            recentName !== null &&
            isStandardConnectedForNetwork(this.network)
          ) {
            await this.connectToStandardWallet(recentName, resolve)

            if (this._connected) {
              return
            }
          }

          this._app.on('userConnected', () => {
            if (this._chosenMobileWalletName) {
              persistRecentStandardWalletForNetwork(this._chosenMobileWalletName, this.network)
            } else {
              clearRecentStandardWalletForNetwork(this.network)
            }
            this._connected = true
            this._connecting = false
            this._appSessionActive = true
            this._modal?.closeModal()
            resolve()
          })

          if (this._modal) {
            this._modal._onClose = () => {
              if (this._connecting) {
                this._connecting = false
                const error = new Error('Connection cancelled')
                reject(error)
              }
            }
            this._modal.openModal(this._app.sessionId, (walletName) => {
              if (
                isMobileBrowser() &&
                !this.walletsList.find((w) => w.name === walletName)?.injectedWallet
              ) {
                this.connectToMobileWallet(walletName)
              } else {
                this.connectToStandardWallet(walletName, resolve)
              }
            })
          }
          // eslint-disable-next-line @typescript-eslint/no-explicit-any
        } catch (error: any) {
          this._connecting = false
          reject(error)
        }
      }

      innerConnect()
    })

  disconnect = async () => {
    if (this.connected) {
      if (this._appSessionActive) {
        clearSessionIdForNetwork(this.network)
        this._appSessionActive = false
        this._loading = true
        AppPolkadot.build(this._appInitData)
          .then(
            (app) => {
              this._app = app
            },
            (err) => {
              console.log(err)
            }
          )
          .finally(() => {
            this._loading = false
          })
      }
      if (this._innerStandardAdapter) {
        this._innerStandardAdapter = undefined
        persistStandardDisconnectForNetwork(this.network)
      }
      // Update recent wallet
      this.walletsList = getPolkadotWalletsList(
        this._metadataWallets,
        getRecentStandardWalletForNetwork(this.network) ?? undefined
      )
      if (this._modal) {
        this._modal.walletsList = this.walletsList
      }
      this._connected = false
    }
  }
}
