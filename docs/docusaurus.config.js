module.exports = {
  title: "Solarti Program Library Docs",
  tagline:
    "Miraland is an open source project implementing a new, high-performance, permissionless blockchain.",
  url: "https://spl.miraland.io",
  baseUrl: "/",
  favicon: "img/favicon.ico",
  organizationName: "miraland-labs", // Usually your GitHub org/user name.
  projectName: "miraland-program-library", // Usually your repo name.
  themeConfig: {
    navbar: {
      logo: {
        alt: "Miraland Logo",
        src: "img/logo-horizontal.svg",
        srcDark: "img/logo-horizontal-dark.svg",
      },
      items: [
        {
          href: "https://docs.miraland.io/",
          label: "Docs »",
          position: "left",
        },
        {
          href: "https://miraland.io/discord",
          label: "Chat",
          position: "right",
        },

        {
          href: "https://github.com/miraland-labs/miraland-program-library",
          label: "GitHub",
          position: "right",
        },
      ],
    },
    footer: {
      style: "dark",
      links: [
        {
          title: "Community",
          items: [
            {
              label: "Discord",
              href: "https://miraland.io/discord",
            },
            {
              label: "Twitter",
              href: "https://twitter.com/miracleland6",
            },
            {
              label: "Forums",
              href: "https://forums.miraland.io",
            },
          ],
        },
        {
          title: "More",
          items: [
            {
              label: "GitHub",
              href: "https://github.com/miraland-labs/miraland-program-library",
            },
          ],
        },
      ],
      copyright: `Copyright © ${new Date().getFullYear()} Miraland Foundation`,
    },
  },
  plugins: [require.resolve('docusaurus-lunr-search')],
  presets: [
    [
      "@docusaurus/preset-classic",
      {
        docs: {
          path: "src",
          routeBasePath: "/",
          sidebarPath: require.resolve("./sidebars.js"),
        },
        theme: {
          customCss: require.resolve("./src/css/custom.css"),
        },
      },
    ],
  ],
};
