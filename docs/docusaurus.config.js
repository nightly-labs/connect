/** @type {import('@docusaurus/types').DocusaurusConfig} */
module.exports = {
  title: 'Nightly Connect',
  tagline: 'Nightly Connect',
  url: 'https://nightly.app',
  baseUrl: '/',
  onBrokenLinks: 'ignore',
  onBrokenMarkdownLinks: 'warn',
  favicon: 'img/logo.png',
  organizationName: 'Nightly Connect',
  projectName: 'Nightly Connect',
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
  themeConfig: {
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
          href: 'https://twitter.com/Nightly_app',
          label: 'Twitter',
          position: 'right'
        },
        {
          href: 'https://discord.gg/3b6GecKPBH',
          label: 'Discord',
          position: 'right'
        },
        {
          href: 'https://github.com/nightly-labs',
          label: 'GitHub',
          position: 'right'
        }
      ]
    },
    algolia: {
      apiKey: '57f60c8df985aec6f8f44287f6b98c16',
      indexName: 'NightlyConnect',
      appId: 'L5GQNBSXIK',
      rateLimit: 5
    },
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
              href: 'https://twitter.com/Nightly_app'
            }
          ]
        },
        {
          title: 'More',
          items: [
            {
              label: 'GitHub',
              href: 'https://github.com/nightly-labs'
            }
          ]
        }
      ],
      copyright: `Copyright © ${new Date().getFullYear()} Nightly | Built with Docusaurus.`
    }
  }
}
