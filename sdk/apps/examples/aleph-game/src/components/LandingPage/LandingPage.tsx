import { Component, Show } from 'solid-js'
import Logo from '../../static/svg/Logo.svg'
import artFennec from '../../static/svg/artFennec.svg'
import './Landingpage.css'

export interface ILandingPageProps {
  isConnected: boolean
  hasTicketsToClaim: boolean
  onConnectWallet?: () => void
  onAddTickets?: () => void
  onClaimTickets?: () => void
}

export const LandingPage: Component<ILandingPageProps> = (props) => {
  return (
    <div class="mainContainer">
      <img class="headerImg" src={Logo} alt="" />
      <div class="landingDescription">
        <span>Win up to $150 with Nightly</span>
        <span class="landingTextInfo">
          Connect your Nightly Wallet and claim your raffle ticket.
        </span>
        <Show when={props.isConnected && !props.hasTicketsToClaim}>
          <div style={{ display: 'flex', 'justify-content': 'center' }}>
            <button onClick={props.onClaimTickets} class="landingButton">
              Already Claimed! See your tickets
            </button>
          </div>
        </Show>
        <Show when={props.isConnected && props.hasTicketsToClaim}>
          <div style={{ display: 'flex', 'justify-content': 'center' }}>
            <button onClick={props.onAddTickets} class="landingClaimButton">
              Claim ticket!
            </button>
          </div>
        </Show>
        <Show when={!props.isConnected}>
          <div style={{ display: 'flex', 'justify-content': 'center' }}>
            <button class="landingButton" onClick={props.onConnectWallet}>
              Connect wallet
            </button>
          </div>
        </Show>
      </div>
      <img class="fennecImg" src={artFennec} alt="" />
    </div>
  )
}
