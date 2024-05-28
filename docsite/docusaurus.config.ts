import {themes as prismThemes} from 'prism-react-renderer';
import type {Config} from '@docusaurus/types';
import type * as Preset from '@docusaurus/preset-classic';

import dotenv from "dotenv"
dotenv.config();

const config: Config = {
  title: "monitor",
  tagline: "distributed build and deployment system",
  favicon: "img/favicon.ico",

  // Set the production url of your site here
  url: "https://docs.monitor.mogh.tech",
  // Set the /<baseUrl>/ pathname under which your site is served
  // For GitHub pages deployment, it is often '/<projectName>/'
  // baseUrl: "/monitor/",
  baseUrl: "/",

  // GitHub pages deployment config.
  // If you aren't using GitHub pages, you don't need these.
  organizationName: "mbecker20", // Usually your GitHub org/user name.
  projectName: "monitor", // Usually your repo name.
  trailingSlash: false,
  deploymentBranch: "gh-pages-docs",

  onBrokenLinks: "throw",
  onBrokenMarkdownLinks: "warn",

  // Even if you don't use internationalization, you can use this field to set
  // useful metadata like html lang. For example, if your site is Chinese, you
  // may want to replace "en" with "zh-Hans".
  i18n: {
    defaultLocale: "en",
    locales: ["en"],
  },

  presets: [
    [
      "classic",
      {
        docs: {
          sidebarPath: "./sidebars.ts",
          // Please change this to your repo.
          // Remove this to remove the "edit this page" links.
          editUrl:
            "https://github.com/facebook/docusaurus/tree/main/packages/create-docusaurus/templates/shared/",
        },
        blog: {
          showReadingTime: true,
          // Please change this to your repo.
          // Remove this to remove the "edit this page" links.
          editUrl:
            "https://github.com/facebook/docusaurus/tree/main/packages/create-docusaurus/templates/shared/",
        },
        theme: {
          customCss: "./src/css/custom.css",
        },
      } satisfies Preset.Options,
    ],
  ],

  themeConfig: {
    // Replace with your project's social card
    image: "img/monitor-lizard.png",
    docs: {
      sidebar: {
        autoCollapseCategories: true,
      },
    },
    navbar: {
      title: "monitor",
      logo: {
        alt: "monitor lizard",
        src: "img/monitor-lizard.png",
        width: "46px"
      },
      items: [
        {
          type: "docSidebar",
          sidebarId: "docs",
          position: "left",
          label: "docs",
        },
        {
          href: "https://docs.rs/monitor_client/latest/monitor_client/",
          label: "docs.rs",
          position: "right",
        },
        {
          href: "https://github.com/mbecker20/monitor",
          label: "github",
          position: "right",
        },
      ],
    },
    footer: {
      style: "dark",
      copyright: `Built with Docusaurus`,
    },
    prism: {
      theme: prismThemes.github,
      darkTheme: prismThemes.dracula,
    },
  } satisfies Preset.ThemeConfig,
};

export default config;
