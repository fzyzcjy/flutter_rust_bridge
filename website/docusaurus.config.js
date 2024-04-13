// @ts-check
// Note: type annotations allow type checking and IDEs autocompletion

const lightCodeTheme = require('prism-react-renderer/themes/github');
const darkCodeTheme = require('prism-react-renderer/themes/dracula');

/** @type {import('@docusaurus/types').Config} */
const config = {
    title: 'flutter_rust_bridge',
    tagline: '', // TODO
    url: 'https://cjycode.com',
    baseUrl: '/flutter_rust_bridge/',
    onBrokenLinks: 'throw',
    onBrokenMarkdownLinks: 'warn',
    staticDirectories: ['static'],

    // GitHub pages deployment config.
    // If you aren't using GitHub pages, you don't need these.
    organizationName: 'fzyzcjy', // Usually your GitHub org/user name.
    projectName: 'flutter_rust_bridge', // Usually your repo name.

    // https://docusaurus.io/docs/api/docusaurus-config#trailingSlash
    // https://github.com/facebook/docusaurus/issues/3372
    trailingSlash: false,

    // Even if you don't use internalization, you can use this field to set useful
    // metadata like html lang. For example, if your site is Chinese, you may want
    // to replace "en" with "zh-Hans".
    i18n: {
        defaultLocale: 'en',
        locales: ['en'],
    },

    markdown: {
        mermaid: true,
    },
    themes: ['@docusaurus/theme-mermaid'],

    presets: [
        [
            'classic',
            /** @type {import('@docusaurus/preset-classic').Options} */
            ({
                docs: {
                    sidebarPath: require.resolve('./sidebars.js'),
                    // Please change this to your repo.
                    // Remove this to remove the "edit this page" links.
                    editUrl: 'https://github.com/fzyzcjy/flutter_rust_bridge/tree/master/website/',
                    // https://docusaurus.io/docs/docs-introduction#docs-only-mode
                    routeBasePath: '/',
                },
                // https://docusaurus.io/docs/docs-introduction#docs-only-mode
                blog: false,
                // blog: {
                //     showReadingTime: true,
                //     // Please change this to your repo.
                //     // Remove this to remove the "edit this page" links.
                //     editUrl:
                //         'https://github.com/fzyzcjy/flutter_smooth/tree/main/packages/create-docusaurus/templates/shared/',
                // },
                theme: {
                    customCss: require.resolve('./src/css/custom.css'),
                },
            }),
        ],
    ],

    themeConfig:
    /** @type {import('@docusaurus/preset-classic').ThemeConfig} */
        ({
            docs: {
                sidebar: {
                    autoCollapseCategories: true,
                },
            },
            navbar: {
                title: 'Flutter Rust Bridge',
                logo: {
                    alt: 'Logo',
                    src: 'logo.png',
                    srcDark: 'logo_dark.png',
                },
                items: [
                    // TODO
                    // {
                    //     type: 'doc',
                    //     docId: 'intro',
                    //     position: 'left',
                    //     label: 'Doc',
                    // },
                    // {to: '/blog', label: 'Blog', position: 'left'},
                    {
                        type: 'dropdown',
                        label: 'v2',
                        position: 'right',
                        items: [
                            {
                                label: 'v2 (current)',
                                href: '/flutter_rust_bridge',
                            },
                            {
                                label: 'v1',
                                href: 'https://fzyzcjy.github.io/flutter_rust_bridge/v1',
                            },
                            // ... more items
                        ],
                    },
                    {
                        href: 'https://github.com/fzyzcjy/flutter_rust_bridge',
                        label: 'GitHub',
                        position: 'right',
                    },
                ],
            },
            // footer: {
            //     style: 'dark',
            //     links: [
            //         {
            //             title: 'Docs',
            //             items: [
            //                 {
            //                     label: 'Tutorial',
            //                     to: '/docs/intro',
            //                 },
            //             ],
            //         },
            //         {
            //             title: 'Community',
            //             items: [
            //                 {
            //                     label: 'Stack Overflow',
            //                     href: 'https://stackoverflow.com/questions/tagged/docusaurus',
            //                 },
            //                 {
            //                     label: 'Discord',
            //                     href: 'https://discordapp.com/invite/docusaurus',
            //                 },
            //                 {
            //                     label: 'Twitter',
            //                     href: 'https://twitter.com/docusaurus',
            //                 },
            //             ],
            //         },
            //         {
            //             title: 'More',
            //             items: [
            //                 {
            //                     label: 'Blog',
            //                     to: '/blog',
            //                 },
            //                 {
            //                     label: 'GitHub',
            //                     href: 'https://github.com/fzyzcjy/flutter_smooth',
            //                 },
            //             ],
            //         },
            //     ],
            //     copyright: `Copyright © ${new Date().getFullYear()} My Project, Inc. Built with Docusaurus.`,
            // },
            prism: {
                theme: lightCodeTheme,
                darkTheme: darkCodeTheme,
                defaultLanguage: "dart",
                additionalLanguages: ["dart", "cpp", "yaml", "toml", "rust", "kotlin"],
            },
            algolia: {
                // The application ID provided by Algolia
                appId: 'LN8F0L9BJQ',
                // Public API key: it is safe to commit it
                apiKey: '05baf74fb51d368bbef454edaea08ab1',
                indexName: 'cjycodecom',
                // Optional: see doc section below
                contextualSearch: true,
                // Optional: Specify domains where the navigation should occur through window.location instead on history.push. Useful when our Algolia config crawls multiple documentation sites and we want to navigate with window.location.href to them.
                // externalUrlRegex: 'external\\.com|domain\\.com',
                // Optional: Algolia search parameters
                // searchParameters: {},
                // Optional: path for search page that enabled by default (`false` to disable it)
                searchPagePath: 'search',
                //... other Algolia params
            },
        }),

    plugins: [
        // https://github.com/facebook/docusaurus/issues/3272
        function (context, options) {
            return {
                name: 'webpack-configuration-plugin',
                configureWebpack(config, isServer, utils) {
                    return {
                        resolve: {
                            symlinks: false,
                        }
                    };
                }
            };
        },
        // https://dev.to/sajclarke_62/using-tailwindcss-v3-in-docusaurus-in-5-steps-5c26
        async function myPlugin(context, options) {
            return {
                name: "docusaurus-tailwindcss",
                configurePostCss(postcssOptions) {
                    // Appends TailwindCSS and AutoPrefixer.
                    postcssOptions.plugins.push(require("tailwindcss"));
                    postcssOptions.plugins.push(require("autoprefixer"));
                    return postcssOptions;
                },
            };
        },
    ],
};

module.exports = config;
