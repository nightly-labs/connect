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
            to: 'docs/application/application/connect',
            activeBasePath: 'connect',
            label: 'Application',
            position: 'left'
          },
          {
            to: 'docs/client/client/connect',
            activeBasePath: 'connect',
            label: 'Client',
            position: 'left'
          },
          {
            href: 'https://example.connect.nightly.app/',
            label: 'Demo',
            position: 'right'
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
                label: 'home',
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
