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
      label: "Connecting Servers",
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
      label: "Build Images",
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
      label: "Deploy Containers",
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
    "sync-resources",
    "permissioning",
    "file-paths",
  ],
};

export default sidebars;
