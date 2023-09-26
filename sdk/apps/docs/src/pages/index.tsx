import React from 'react'
import { Redirect } from '@docusaurus/router'

export default function Home(): JSX.Element {
  return (
    <>
      <meta charSet="utf-8" />
      <meta name="viewport" content="width=device-width, initial-scale=1" />
      <meta name="title" content="Nightly Connect - entire Web3 in one place" />

      {/* <!-- Twitter --> */}
      <meta property="twitter:card" content="summary_large_image" />
      <meta property="twitter:title" content="Nightly Connect - entire Web3 in one place" />
      <meta
        property="twitter:description"
        content="Find Nightly Connect documentation in one place. Try Nightly Connect's open-source code and push your projects on Solana, Aleph Zero, or Sui to the next level."
      />
      {/* <meta
        property="twitter:image"
        content="https://near.game.nightly.app/graphicRaffleNear.png"
      /> */}
      {/* DublinCore */}
      <meta property="DC.title" content="Nightly Connect - entire Web3 in one place" />
      <meta
        property="DC.description"
        content="Find Nightly Connect documentation in one place. Try Nightly Connect's open-source code and push your projects on Solana, Aleph Zero, or Sui to the next level."
      />
      {/* <meta property="DC:image" content="https://near.game.nightly.app/graphicRaffleNear.png" /> */}

      {/* <!-- Open Graph / Facebook --> */}
      <meta property="og:type" content="website" />
      <meta property="og:title" content="Nightly Connect - entire Web3 in one place" />
      <meta
        property="og:description"
        content="Find Nightly Connect documentation in one place. Try Nightly Connect's open-source code and push your projects on Solana, Aleph Zero, or Sui to the next level."
      />
      {/* <meta property="og:image" content="https://near.game.nightly.app/graphicRaffleNear.png" /> */}

      <Redirect to="/docs/" />
    </>
  )
}
