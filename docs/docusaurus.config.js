// @ts-check
// Note: type annotations allow type checking and IDEs autocompletion

const lightCodeTheme = require("prism-react-renderer/themes/github");
const darkCodeTheme = require("prism-react-renderer/themes/dracula");

/** @type {import('@docusaurus/types').Config} */
const config = {
  title: "PRIEDE",
  tagline: "Latviski rakstāma programmēšanas valoda",
  favicon: "img/favicon.ico",
  //TODO: Set the production url of your site here
  url: "https://priede.vercel.app",
  baseUrl: "/",
  i18n: {
    defaultLocale: "lv",
    locales: ["lv"],
  },

  presets: [
    [
      "classic",
      /** @type {import('@docusaurus/preset-classic').Options} */
      ({
        docs: {
          //TODO: rediģēšanas links
          sidebarPath: require.resolve("./sidebars.js"),
          // Please change this to your repo.
          // Remove this to remove the "edit this page" links.
        },
        blog: {
          showReadingTime: false,
        },
        theme: {
          customCss: require.resolve("./src/css/custom.css"),
        },
      }),
    ],
  ],

  themeConfig:
    /** @type {import('@docusaurus/preset-classic').ThemeConfig} */
    ({
      colorMode: {
        defaultMode: "dark",
        disableSwitch: false,
        respectPrefersColorScheme: false,
      },
      // TODO: Replace with your project's social card
      image: "img/docusaurus-social-card.jpg",
      navbar: {
        title: "PRIEDE",
        logo: {
          alt: "Priedes logo",
          src: "img/priede.png",
        },
        items: [
          {
            type: "doc",
            docId: "kas-ir-priede",
            position: "left",
            label: "Dokumentācija",
          },
          {
            href: "https://github.com/MarcisAn/priede",
            label: "GitHub",
            position: "right",
          },
          {
            href: "https://priede-editor.vercel.app",
            label: "Tiešsaistes redaktors",
            position: "right",
          },
        ],
      },
      footer: {
        style: "dark",
        links: [
          {
            items: [
              {
                label: "Dokumentācija",
                to: "/docs/kas-ir-priede",
              },
            ],
          },

          {
            title: "Saites",
            items: [
              {
                label: "WEB redaktors",
                to: "https://priede-editor.vercel.app",
              },
              {
                label: "GitHub",
                href: "https://github.com/MarcisAn/priede",
              },
            ],
          },
        ],
        copyright: `PRIEDE`,
      },
      prism: {
        theme: lightCodeTheme,
        darkTheme: darkCodeTheme,
      },
    }),
  onBrokenLinks: "ignore",
};

module.exports = config;
