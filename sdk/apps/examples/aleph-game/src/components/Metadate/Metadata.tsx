import { Meta, Title } from 'solid-start'
export const Metadata = () => {
  return (
    <div>
      <Title>Near event</Title>
      <Meta charset="utf-8" />
      <Meta name="viewport" content="width=device-width, initial-scale=1" />
      <Meta name="title" content="Nightly NEAR Game - are you NEAR winning?" />

      {/* <!-- Twitter --> */}
      <Meta property="twitter:card" content="summary_large_image" />
      <Meta property="twitter:title" content="Nightly NEAR Game - are you NEAR winning?" />
      <Meta
        property="twitter:description"
        content="Enter Nightly's raffle on NEAR. Each ticket gives you a chance to win. Collect all nine tickets within Nightly Wallet and win token prize on NEAR blockchain."
      />
      <Meta
        property="twitter:image"
        content="https://near.game.nightly.app/graphicRaffleNear.png"
      />
      {/* DublinCore */}
      <Meta property="DC.title" content="Nightly NEAR Game - are you NEAR winning?" />
      <Meta
        property="DC.description"
        content="Enter Nightly's raffle on NEAR. Each ticket gives you a chance to win. Collect all nine tickets within Nightly Wallet and win token prize on NEAR blockchain."
      />
      <Meta property="DC:image" content="https://near.game.nightly.app/graphicRaffleNear.png" />

      {/* <!-- Open Graph / Facebook --> */}
      <Meta property="og:type" content="website" />
      <Meta property="og:title" content="Nightly NEAR Game - are you NEAR winning?" />
      <Meta
        property="og:description"
        content="Enter Nightly's raffle on NEAR. Each ticket gives you a chance to win. Collect all nine tickets within Nightly Wallet and win token prize on NEAR blockchain."
      />
      <Meta property="og:image" content="https://near.game.nightly.app/graphicRaffleNear.png" />
    </div>
  )
}
