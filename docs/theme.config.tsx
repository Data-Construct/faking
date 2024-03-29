import React from 'react'
import { DocsThemeConfig } from 'nextra-theme-docs'
import Logo from "./pages/assets/LogoBackground.png";

const config: DocsThemeConfig = {
  logo: (
    <span
      style={{
        display: "flex",
        gap: "0.5rem"
      }}
    >
      <img
        src={Logo.src}
        width="24px"
        alt=""
        style={{
          borderRadius: "4px",
        }}
      /> DataConstruct Docs
    </span>
  ),
  head: (<>
    <link rel="icon" href="/assets/Logo.ico" sizes="any" />
    <link rel="icon" href="/assets/Logo.svg" type="image/svg+xml" />
  </>),
  project: {
    link: 'https://github.com/Data-Construct/faking',
  },
  editLink: {
    text: "Edit this page on GitHub",
  },
  docsRepositoryBase: 'https://github.com/Data-Construct/faking/tree/main/docs',
  primaryHue: 219,
  // primarySaturation: "",
  chat: {
    link: 'https://discord.gg/zxcpKF4db7',
  },
  footer: {
    text: (
      <>
        <a href="https://www.dataconstruct.io">DataConstruct.io</a>
      </>
    ),
  },
}

export default config
