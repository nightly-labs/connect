import { AppSui, SUI_NETWORK } from '@nightlylabs/nightly-connect-sui'
import { StandardWalletAdapter } from '@mysten/wallet-adapter-wallet-standard'
import { NightlyConnectSuiWallet, StandardAdapterWithDisconnectAction } from './wallet'
import { publicKeyFromSerialized } from '@mysten/sui.js'
import { suiWalletsFilter } from './detection'
import {
  AppInitData,
  MetadataWallet,
  NCBaseSelector,
  QueryNetwork,
  clearRecentStandardWalletForNetwork,
  clearSessionIdForNetwork,
  persistStandardDisconnectForNetwork,
  persistRecentStandardWalletForNetwork
} from '@nightlylabs/wallet-selector-base'
import { StandardWalletAdapterWallet } from '@mysten/wallet-standard'
import bs58 from 'bs58'

export const convertBase58toBase64 = (base58: string) => {
  const buffer = bs58.decode(base58)
  return buffer.toString('base64')
}

export class NCSuiSelector extends NCBaseSelector<StandardWalletAdapter> {
  private _app: AppSui

  constructor(
    appInitData: AppInitData,
    app: AppSui,
    metadataWallets: MetadataWallet[],
    onConnected: (adapter: StandardWalletAdapter) => void,
    eagerConnect?: boolean,
    anchorRef?: HTMLElement,
    onOpen?: () => void,
    onClose?: () => void
  ) {
    super(
      appInitData,
      metadataWallets,
      (wallet) => {
        const adapter = new StandardAdapterWithDisconnectAction(
          wallet as StandardWalletAdapterWallet,
          () => {
            persistStandardDisconnectForNetwork(SUI_NETWORK)
          }
        )
        return adapter
      },
      suiWalletsFilter,
      {
        network: QueryNetwork.SUI,
        name: 'Sui',
        icon: 'https://assets.coingecko.com/coins/images/26375/small/sui_asset.jpeg'
      },
      app.sessionId,
      (walletName, url) => {
        this._app.base.connectDeeplink({
          walletName,
          url
        })
      },
      onConnected,
      eagerConnect,
      anchorRef,
      onOpen,
      onClose
    )
    this._app = app
    this.setApp(app)
  }

  private setApp = (app: AppSui) => {
    this._app = app
    this.sessionId = app.sessionId
    if (this._app.base.hasBeenRestored && !!this._app.base.connectedPublicKeys.length) {
      this.eagerConnectDeeplink(SUI_NETWORK)
      this.initNCAdapter(this._app.base.connectedPublicKeys)
    }

    this.eagerConnectToRecent()

    this._app.on('userConnected', (e) => {
      if (this._chosenMobileWalletName) {
        persistRecentStandardWalletForNetwork(this._chosenMobileWalletName, SUI_NETWORK)
      } else {
        clearRecentStandardWalletForNetwork(SUI_NETWORK)
      }
      this.initNCAdapter(e.publicKeys)
    })
  }

  initNCAdapter = (publicKeys: string[]) => {
    const adapter = new StandardWalletAdapter({
      wallet: new NightlyConnectSuiWallet(
        this._app,
        publicKeys.map((pk) => publicKeyFromSerialized('ED25519', convertBase58toBase64(pk))),
        async () => {
          clearSessionIdForNetwork(SUI_NETWORK)
          const app = await AppSui.build(this._appInitData)
          this.setApp(app)
        }
      )
    })
    adapter.connect().then(() => {
      this._onConnected(adapter)
      this.closeModal()
    })
  }

  public static build = async (
    appInitData: AppInitData,
    onConnected: (adapter: StandardWalletAdapter) => void,
    eagerConnectForStandardWallets?: boolean,
    anchorRef?: HTMLElement,
    onOpen?: () => void,
    onClose?: () => void
  ) => {
    const [app, metadataWallets] = await Promise.all([
      AppSui.build(appInitData),
      AppSui.getWalletsMetadata('https://nc2.nightly.app/get_wallets_metadata')
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
    const selector = new NCSuiSelector(
      appInitData,
      app,
      metadataWallets,
      onConnected,
      eagerConnectForStandardWallets,
      anchorRef,
      onOpen,
      onClose
    )

    return selector
  }
}
