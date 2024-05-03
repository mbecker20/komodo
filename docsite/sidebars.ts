import type {SidebarsConfig} from '@docusaurus/plugin-content-docs';

/**
 * Creating a sidebar enables you to:
 - create an ordered group of docs
 - render a sidebar for each doc of that group
 - provide next/previous navigation

 The sidebars can be generated from the filesystem, or explicitly defined here.

 Create as many sidebars as you want.
 */
const sidebars: SidebarsConfig = {
  docs: [
    "intro",
    "core-setup",
    {
      type: "category",
      label: "connecting servers",
      link: {
        type: "doc",
        id: "connecting-servers/index",
      },
      items: [
        "connecting-servers/setup-periphery",
        "connecting-servers/add-server",
      ],
    },
    {
      type: "category",
      label: "build images",
      link: {
        type: "doc",
        id: "build-images/index",
      },
      items: [
        "build-images/configuration",
        "build-images/pre-build",
        "build-images/choosing-builder",
        "build-images/versioning",
      ],
    },
    {
      type: "category",
      label: "deploy containers",
      link: {
        type: "doc",
        id: "deploy-containers/index",
      },
      items: [
        "deploy-containers/configuration",
        "deploy-containers/lifetime-management",
        // "deploy-containers/choosing-builder",
        // "deploy-containers/versioning",
      ],
    },
    "permissioning",
    "file-paths",
  ],
};

export default sidebars;
