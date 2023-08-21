import { NightlyConnectAdapter } from '@nightlylabs/wallet-selector-polkadot'

// Create a single supabase client for interacting with your database
let _adapter: NightlyConnectAdapter | undefined
export const getAdapter = async () => {
  if (_adapter) return _adapter
  _adapter = await NightlyConnectAdapter.build(
    {
      appMetadata: {
        name: 'NC TEST AlephZero',
        description: 'Nightly Connect Test',
        icon: 'https://docs.nightly.app/img/logo.png',
        additionalInfo: 'Courtesy of Nightly Connect team'
      },
      network: 'AlephZero'
    },
    true, // change this to false to test disabling eager connect
    document.getElementById('modalAnchor')
  )
  return _adapter
}
