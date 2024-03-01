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
      label: 'Solana',
      collapsed: false,
      items: ['solana/connect', 'solana/events', 'solana/sign_transaction', 'solana/sign_message']
    },
    {
      type: 'category',
      label: 'SUI',
      collapsed: false,
      items: ['sui/connect', 'sui/events', 'sui/sign_transaction', 'sui/sign_message']
    },
    {
      type: 'category',
      label: 'Substrate',
      collapsed: false,
      items: ['substrate/connect', 'substrate/sign_transaction', 'substrate/sign_message']
    },
    {
      type: 'category',
      label: 'Customization',
      collapsed: false,
      items: ['customization/ui_overrides', 'customization/external_modal']
    },
    'push'
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
