/* eslint-disable @typescript-eslint/no-empty-function */
import { AppPolkadot, POLKADOT_NETWORK } from '@nightlylabs/nightly-connect-polkadot'
import {
  AppInitData,
  IWalletListItem,
  MetadataWallet,
  NightlyConnectSelectorModal,
  QueryNetwork,
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
import { type Injected } from '@polkadot/extension-inject/types'
import { WalletNotReadyError, WalletWindowClosedError } from '@solana/wallet-adapter-base'
import { PolkadotWalletInjected, getPolkadotWallets } from './detection'

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

  private _appInitData: AppInitData
  private _eagerConnectForStandardWallets: boolean

  private _metadataWallets: MetadataWallet[] = []
  private _injectedWallets: PolkadotWalletInjected[] = []
  private _walletsList: IWalletListItem[] = []

  private _chosenMobileWalletName: string | undefined

  private _loading: boolean

  constructor(appInitData: AppInitData, eagerConnectForStandardWallets?: boolean) {
    this._connecting = false
    this._connected = false
    this._appInitData = appInitData
    this._eagerConnectForStandardWallets = !!eagerConnectForStandardWallets
    this._appSessionActive = false
    this._loading = false
  }

  get accounts() {
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

  public static build = async (
    appInitData: AppInitData,
    eagerConnectForStandardWallets?: boolean,
    anchorRef?: HTMLElement | null
  ) => {
    const adapter = new NightlyConnectAdapter(appInitData, eagerConnectForStandardWallets)

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
    const recentWalletName = getRecentStandardWalletForNetwork(POLKADOT_NETWORK) ?? undefined

    adapter._walletsList = metadataWallets.map((wallet) => ({
      name: wallet.name,
      icon: wallet.icon,
      link: wallet.link,
      deeplink: wallet.deeplink,
      recent: recentWalletName === wallet.name
    }))
    const detectedWallets = getPolkadotWallets()
    console.log({ detectedWallets })
    adapter._injectedWallets = detectedWallets
    // TODO do the deduplication
    adapter._walletsList = adapter._walletsList.concat(
      ...detectedWallets.map((wallet) => ({
        name: wallet.name ?? 'Unknown',
        icon: wallet.icon ?? '',
        deeplink: null,
        link: '',
        recent: recentWalletName === wallet.name,
        detected: true
      }))
    )

    adapter._modal = new NightlyConnectSelectorModal(
      adapter._walletsList,
      appInitData.url ?? 'https://nc2.nightly.app',
      {
        network: QueryNetwork.POLKADOT,
        name: POLKADOT_NETWORK,
        icon: 'https://assets.coingecko.com/coins/images/4128/small/solana.png' // TODO add polka icon
      },
      anchorRef
    )

    return adapter
  }

  public static buildLazy = (
    appInitData: AppInitData,
    eagerConnectForStandardWallets?: boolean,
    anchorRef?: HTMLElement | null
  ) => {
    const adapter = new NightlyConnectAdapter(appInitData, eagerConnectForStandardWallets)

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
      const recentWalletName = getRecentStandardWalletForNetwork(POLKADOT_NETWORK) ?? undefined

      adapter._walletsList = metadataWallets.map((wallet) => ({
        name: wallet.name,
        icon: wallet.icon,
        link: wallet.link,
        deeplink: wallet.deeplink,
        recent: recentWalletName === wallet.name
      }))
      console.log(adapter._walletsList)
      const detectedWallets = getPolkadotWallets()
      console.log({ detectedWallets })
      adapter._injectedWallets = detectedWallets
      // TODO do the deduplication

      adapter._walletsList = adapter._walletsList.concat(
        ...detectedWallets.map((wallet) => ({
          name: wallet.name ?? 'Unknown',
          icon: wallet.icon ?? '',
          deeplink: null,
          link: '',
          recent: recentWalletName === wallet.name,
          detected: true
        }))
      )

      adapter._modal = new NightlyConnectSelectorModal(
        adapter._walletsList,
        appInitData.url ?? 'https://nc2.nightly.app',
        {
          network: QueryNetwork.POLKADOT,
          name: POLKADOT_NETWORK,
          icon: 'https://assets.coingecko.com/coins/images/4128/small/solana.png' // TODO change icon
        },
        anchorRef
      )

      adapter._loading = false
    })

    return adapter
  }
  // ensureLoaded = async () => {}
  canEagerConnect = async () => {
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
      this._eagerConnectForStandardWallets &&
      getRecentStandardWalletForNetwork(POLKADOT_NETWORK) !== null &&
      isStandardConnectedForNetwork(POLKADOT_NETWORK)
    ) {
      return true
    }

    return false
  }

  eagerConnectDeeplink = () => {
    if (isMobileBrowser() && this._app) {
      const mobileWalletName = getRecentStandardWalletForNetwork(POLKADOT_NETWORK)
      const wallet = this._walletsList.find((w) => w.name === mobileWalletName)

      if (typeof wallet === 'undefined') {
        return
      }

      if (wallet.deeplink === null) {
        return
      }

      if (wallet.deeplink.universal !== null) {
        this._app.connectDeeplink({
          walletName: wallet.name,
          url: wallet.deeplink.universal
        })
        return
      }

      if (wallet.deeplink.native !== null) {
        this._app.connectDeeplink({
          walletName: wallet.name,
          url: wallet.deeplink.native
        })
      }
    }
  }

  connectToMobileWallet = (walletName: string) => {
    if (this._modal) {
      this._modal.setStandardWalletConnectProgress(true)
    }

    const wallet = this._walletsList.find((w) => w.name === walletName)

    if (!this._app || typeof wallet === 'undefined') {
      return
    }

    if (wallet.deeplink === null) {
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
    }
  }

  connectToStandardWallet = async (walletName: string, onSuccess: () => void) => {
    console.log('connect to standard ')
    if (this._modal) {
      this._modal.setStandardWalletConnectProgress(true)
    }
    console.log({ walletName })
    const adapter = this._injectedWallets.find((w) => w.name === walletName)
    if (typeof adapter === 'undefined') {
      if (this._modal) {
        this._modal.setStandardWalletConnectProgress(false)
      }
      throw new Error('Wallet not found')
    }
    console.log({ adapter })

    try {
      const inject = await adapter!.enable!('Nightly Connect') // TODO should we also use connect?
      console.log({ inject })

      persistRecentStandardWalletForNetwork(walletName, POLKADOT_NETWORK)
      persistStandardConnectForNetwork(POLKADOT_NETWORK)
      this._innerStandardAdapter = inject
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
              throw new WalletNotReadyError()
            }
          }

          if (!this._app) {
            throw new WalletNotReadyError()
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

          const recentName = getRecentStandardWalletForNetwork(POLKADOT_NETWORK)
          if (
            this._eagerConnectForStandardWallets &&
            recentName !== null &&
            isStandardConnectedForNetwork(POLKADOT_NETWORK)
          ) {
            await this.connectToStandardWallet(recentName, resolve)

            if (this._connected) {
              return
            }
          }

          this._app.on('userConnected', (e) => {
            if (this._chosenMobileWalletName) {
              persistRecentStandardWalletForNetwork(this._chosenMobileWalletName, POLKADOT_NETWORK)
            } else {
              clearRecentStandardWalletForNetwork(POLKADOT_NETWORK)
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
                const error = new WalletWindowClosedError()
                reject(error)
              }
            }
            this._modal.openModal(this._app.sessionId, (walletName) => {
              if (
                isMobileBrowser() &&
                !this._walletsList.find((w) => w.name === walletName)?.standardWallet
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
        clearSessionIdForNetwork(POLKADOT_NETWORK)
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
        persistStandardDisconnectForNetwork(POLKADOT_NETWORK)
      }
      if (this._modal) {
        this._modal.walletsList = this._walletsList
      }
      this._connected = false
    }
  }
}
