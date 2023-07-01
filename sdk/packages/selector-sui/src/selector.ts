import { AppSui } from '@nightlylabs/nightly-connect-sui'
import { StandardWalletAdapter } from '@mysten/wallet-adapter-wallet-standard'
import { NightlyConnectSuiWallet } from './wallet'
import { publicKeyFromSerialized } from '@mysten/sui.js'
import { suiWalletsFilter } from './detection'
import {
  AppInitData,
  MetadataWallet,
  NCBaseSelector,
  NETWORK
} from '@nightlylabs/wallet-selector-base'
import { StandardWalletAdapterWallet } from '@mysten/wallet-standard'
import bs58 from 'bs58'

export const convertBase58toBase64 = (base58: string) => {
  const buffer = bs58.decode(base58)
  return buffer.toString('base64')
}

export class NCSuiSelector extends NCBaseSelector<StandardWalletAdapter> {
  private _app: AppSui

  constructor(appInitData: AppInitData, app: AppSui, metadataWallets: MetadataWallet[]) {
    super(
      appInitData,
      metadataWallets,
      (wallet) =>
      new StandardWalletAdapter({
        wallet: wallet as StandardWalletAdapterWallet
      }),
      suiWalletsFilter,
      {
        network: NETWORK.SUI,
        name: 'Sui',
        icon: 'https://assets.coingecko.com/coins/images/26375/small/sui_asset.jpeg'
      },
      app.sessionId,
      (walletName, url) => {
        this._app.base.connectDeeplink({
          walletName,
          url
        })
      }
    )
    this._app = app
    this.setApp(app)
  }

  private setApp = (app: AppSui) => {
    this._app = app
    this._app.on('userConnected', (e) => {
      const adapter = new StandardWalletAdapter({
        wallet: new NightlyConnectSuiWallet(
          app,
          e.publicKeys.map((pk) => publicKeyFromSerialized('ED25519', convertBase58toBase64(pk))),
          async () => {
            const app = await AppSui.build(this._appInitData)
            this.setApp(app)
          }
        )
      })
      adapter.connect().then(() => {
        this.onConnected?.(adapter)
        this.closeModal()
      })
    })
  }

  public static build = async (appInitData: AppInitData) => {
    const app = await AppSui.build(appInitData)
    const metadataWallets = await AppSui.getWalletsMetadata(
      'https://nc2.nightly.app/get_wallets_metadata'
    )
      .then((list) =>
        list.map((wallet) => ({
          name: wallet.name,
          icon: wallet.image.default,
          deeplink: wallet.mobile,
          link: wallet.homepage
        }))
      )
      .catch(() => [] as any)
    const selector = new NCSuiSelector(appInitData, app, metadataWallets)

    return selector
  }
}