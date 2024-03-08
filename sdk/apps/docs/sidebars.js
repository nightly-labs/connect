/**
 * Creating a sidebar enables you to:
 - create an ordered group of docs
 - render a sidebar for each doc of that group
 - provide next/previous navigation

 The sidebars can be generated from the filesystem, or explicitly defined here.

 Create as many sidebars as you want.
 */

// @ts-check

/** @type {import('@docusaurus/plugin-content-docs').SidebarsConfig} */
const sidebars = {
  // By default, Docusaurus generates a sidebar from the docs folder structure
  docs: [
    'home',
    'start',
    {
      type: 'category',
      className: 'drop solana-dropdown',
      label: ' Solana',
      collapsed: false,
      items: ['solana/start', 'solana/connect', 'solana/events', 'solana/sign_transaction']
    },
    {
      type: 'category',
      className: 'drop sui-dropdown',
      label: 'Sui',
      collapsed: false,
      items: ['sui/start', 'sui/connect', 'sui/events', 'sui/sign_transaction']
    },
    {
      type: 'category',
      className: 'drop substrate-dropdown',
      label: 'Substrate',
      collapsed: false,
      items: ['substrate/start', 'substrate/connect', 'substrate/sign_transaction']
    },
    {
      type: 'category',
      label: 'Customization',
      collapsed: true,
      items: ['customization/ui_overrides', 'customization/external_modal']
    },
    {
      type: 'category',
      label: 'For wallets',
      collapsed: true,
      items: [
        'for_wallets/connect',
        'for_wallets/sign_transaction',
        'for_wallets/sign_message',
        'for_wallets/push'
      ]
    }
  ]

  // But you can create a sidebar manually
  /*
  tutorialSidebar: [
    'intro',
    'hello',
    {
      type: 'category',
      label: 'Tutorial',
      items: ['tutorial-basics/create-a-document'],
    },
  ],
   */
}

module.exports = sidebars
