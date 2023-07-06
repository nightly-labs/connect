import { AppSui, SUI_NETWORK } from '@nightlylabs/nightly-connect-sui'
import { StandardWalletAdapter } from '@mysten/wallet-adapter-wallet-standard'
import { NightlyConnectSuiWallet } from './wallet'
import { publicKeyFromSerialized } from '@mysten/sui.js'
import { suiWalletsFilter } from './detection'
import {
  AppInitData,
  MetadataWallet,
  NCBaseSelector,
  QueryNetwork,
  clearSessionIdForNetwork
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
    anchorRef?: HTMLElement,
    onOpen?: () => void,
    onClose?: () => void
  ) {
    super(
      appInitData,
      metadataWallets,
      (wallet) =>
        new StandardWalletAdapter({
          wallet: wallet as StandardWalletAdapterWallet
        }),
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
      this.initNCAdapter(this._app.base.connectedPublicKeys)
    }

    this._app.on('userConnected', (e) => {
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
      anchorRef,
      onOpen,
      onClose
    )

    return selector
  }
}
