import { AppSolana, SOLANA_NETWORK } from '@nightlylabs/nightly-connect-solana'
import { StandardWalletAdapter } from '@solana/wallet-standard'
import { NightlyConnectSolanaWallet } from './wallet'
import { PublicKey } from '@solana/web3.js'
import {
  AppInitData,
  MetadataWallet,
  NCBaseSelector,
  QueryNetwork,
  clearSessionIdForNetwork,
  clearUseStandardEagerForNetwork,
  persistRecentStandardWalletForNetwork
} from '@nightlylabs/wallet-selector-base'
import { solanaWalletsFilter } from './detection'
import { WalletAdapterCompatibleStandardWallet } from '@solana/wallet-adapter-base'

export class NCSolanaSelector extends NCBaseSelector<StandardWalletAdapter> {
  private _app: AppSolana

  constructor(
    appInitData: AppInitData,
    app: AppSolana,
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
        const adapter = new StandardWalletAdapter({
          wallet: wallet as WalletAdapterCompatibleStandardWallet
        })
        adapter.on('disconnect', () => {
          clearUseStandardEagerForNetwork(SOLANA_NETWORK)
        })
        return adapter
      },
      solanaWalletsFilter,
      {
        network: QueryNetwork.SOLANA,
        name: 'Solana',
        icon: 'https://assets.coingecko.com/coins/images/4128/small/solana.png'
      },
      app.sessionId,
      (walletName, url) => {
        this._app.connectDeeplink({
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

  private setApp = (app: AppSolana) => {
    this._app = app
    this.sessionId = app.sessionId

    if (this._app.base.hasBeenRestored && !!this._app.base.connectedPublicKeys.length) {
      this.initNCAdapter(this._app.base.connectedPublicKeys)
    }

    this.eagerConnectToRecent()

    this._app.on('userConnected', (e) => {
      if (this._chosenMobileWalletName) {
        persistRecentStandardWalletForNetwork(this._chosenMobileWalletName, SOLANA_NETWORK)
      }
      this.initNCAdapter(e.publicKeys)
    })
  }

  initNCAdapter = (publicKeys: string[]) => {
    const adapter = new StandardWalletAdapter({
      wallet: new NightlyConnectSolanaWallet(this._app, new PublicKey(publicKeys[0]), async () => {
        clearSessionIdForNetwork(SOLANA_NETWORK)
        const app = await AppSolana.build(this._appInitData)
        this.setApp(app)
      })
    })
    adapter.connect().then(() => {
      this._onConnected(adapter)
      this._modal?.handleClose()
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
      AppSolana.build(appInitData),
      AppSolana.getWalletsMetadata('https://nc2.nightly.app/get_wallets_metadata')
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
    const selector = new NCSolanaSelector(
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
