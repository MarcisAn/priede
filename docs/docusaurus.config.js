// @ts-check
// Note: type annotations allow type checking and IDEs autocompletion

const lightCodeTheme = require("prism-react-renderer/themes/github");
const darkCodeTheme = require("prism-react-renderer/themes/dracula");

/** @type {import('@docusaurus/types').Config} */
const config = {
  title: "PRIEDE",
  tagline: "Programmēšanas valoda ar sintaksi latviski",
  favicon: "img/favicon.ico",
  //TODO: Set the production url of your site here
  url: "https://your-docusaurus-test-site.com",
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
            docId: "/category/īsumā-par-priedi",
            position: "left",
            label: "Ātrā pamācība",
          },
          {
            type: "doc",
            docId: "/category/dokumentacija",
            position: "left",
            label: "Dokumentācija",
          },
          { to: "/blog", label: "Blogs", position: "left" },
          {
            href: "https://github.com/MarcisAn/priede",
            label: "GitHub",
            position: "right",
          },
        ],
      },
      footer: {
        style: "dark",
        links: [
          {
            title: "Dokumentācija",
            items: [
              {
                label: "Īsumā par priedi",
                to: "/docs/category/īsumā-par-priedi",
              },
              {
                label: "Pilnā dokumentācija",
                to: "/docs/category/īsumā-par-priedi",
              },
            ],
          },

          {
            title: "Saites",
            items: [
              {
                label: "Blogs",
                to: "/blog",
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
