// @ts-check
// Note: type annotations allow type checking and IDEs autocompletion

const lightCodeTheme = require('prism-react-renderer/themes/github')
const darkCodeTheme = require('prism-react-renderer/themes/dracula')

/** @type {import('@docusaurus/types').Config} */
const config = {
  title: 'Nightly Connect',
  tagline: 'Nightly Connect',
  favicon: 'img/logo.png',
  url: 'https://nightly.app',
  baseUrl: '/',
  organizationName: 'Nightly Connect',
  projectName: 'Nightly Connect',
  onBrokenLinks: 'ignore',
  onBrokenMarkdownLinks: 'warn',
  i18n: {
    defaultLocale: 'en',
    locales: ['en']
  },

  presets: [
    [
      '@docusaurus/preset-classic',
      {
        docs: {
          sidebarPath: require.resolve('./sidebars.js'),
          editUrl: 'https://nightly.app/'
        },
        theme: {
          customCss: require.resolve('./src/css/custom.css')
        }
      }
    ]
  ],

  themeConfig:
    /** @type {import('@docusaurus/preset-classic').ThemeConfig} */
    ({
      colorMode: {
        defaultMode: 'dark',
        disableSwitch: false,
        respectPrefersColorScheme: false
      },
      image: 'img/docusaurus-social-card.jpg',
      navbar: {
        title: 'Nightly Connect',
        logo: {
          alt: 'Nightly Logo',
          src: 'img/logo.png'
        },
        items: [
          {
            to: 'docs/',
            activeBasePath: 'docs',
            label: 'Docs',
            position: 'left'
          },
          {
            to: 'docs/start/',
            activeBasePath: 'docs/start',
            label: 'Getting started',
            position: 'left'
          },
          {
            type: 'dropdown',
            label: 'Networks',
            position: 'left',
            items: [
              {
                to: 'docs/solana/solana/start',
                label: 'Solana',
                className: 'network_navlink solana_navlink'
              },
              { to: 'docs/sui/sui/start', label: 'Sui', className: 'network_navlink sui_navlink' },
              {
                to: 'docs/substrate/substrate/start',
                label: 'Substrate',
                className: 'network_navlink substrate_navlink'
              },
              {
                to: 'docs/aptos/aptos/start',
                label: 'Aptos',
                className: 'network_navlink aptos_navlink'
              },
              {
                to: 'docs/movement/movement/start',
                label: 'Movement',
                className: 'network_navlink movement_navlink'
              }
            ]
          },
          {
            to: 'docs/customization/customization/ui_overrides',
            activeBasePath: 'docs/customization/customization/ui_overrides',
            label: 'Customization',
            position: 'left'
          },
          {
            to: 'docs/for_wallets/for_wallets/connect',
            activeBasePath: 'docs/for_wallets/for_wallets/connect',
            label: 'For wallets',
            position: 'left'
          },
          {
            type: 'dropdown',
            label: 'Demo',
            position: 'right',
            items: [
              { href: 'https://solana-web3-template.nightly.app', label: 'Solana template' },
              { href: 'https://sui-web3-template.nightly.app', label: 'Sui template' },
              { href: 'https://aleph-zero-web3-template.nightly.app', label: 'Substrate template' },
              { href: 'https://aptos-web3-template.vercel.app', label: 'Aptos template' },
              { href: 'https://movement-web3-template.vercel.app', label: 'Movement template' }
            ]
          },
          {
            href: 'https://twitter.com/NightlyConnect',
            label: 'Twitter',
            position: 'right'
          },
          {
            href: 'https://discord.gg/3b6GecKPBH',
            label: 'Discord',
            position: 'right'
          },
          {
            href: 'https://github.com/nightly-labs/connect',
            label: 'GitHub',
            position: 'right'
          }
        ]
      },
      // algolia: {
      //   apiKey: '57f60c8df985aec6f8f44287f6b98c16',
      //   indexName: 'NightlyConnect',
      //   appId: 'L5GQNBSXIK',
      //   rateLimit: 5
      // },
      footer: {
        style: 'dark',
        links: [
          {
            title: 'Docs',
            items: [
              {
                label: 'Home',
                to: 'docs/'
              }
            ]
          },
          {
            title: 'Community',
            items: [
              {
                label: 'Discord',
                href: 'https://discord.gg/3b6GecKPBH'
              },
              {
                label: 'Twitter',
                href: 'https://twitter.com/NightlyConnect'
              }
            ]
          },
          {
            title: 'More',
            items: [
              {
                label: 'GitHub',
                href: 'https://github.com/nightly-labs/connect'
              }
            ]
          }
        ],

        copyright: `Copyright © ${new Date().getFullYear()} Nightly | Built with Docusaurus.`
      },
      prism: {
        theme: lightCodeTheme,
        darkTheme: darkCodeTheme
      }
    })
}

module.exports = config
