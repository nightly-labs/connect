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
        <span>
          Connect a wallet and <span class="landingMainText">start your adventure</span> with
          Nightly!
        </span>
        <span class="landingTextInfo">
          Scan QR codes and claim tickets! There’s 9 tickets to get. Good luck!
        </span>
        <Show when={props.isConnected && !props.hasTicketsToClaim}>
          <button onClick={props.onClaimTickets} class="landingButton">
            Connected! See your tickets
          </button>
        </Show>
        <Show when={props.isConnected && props.hasTicketsToClaim}>
          <button onClick={props.onAddTickets} class="landingClaimButton">
            Claim ticket!
          </button>
        </Show>
        <Show when={!props.isConnected}>
          <button class="landingButton" onClick={props.onConnectWallet}>
            Connect wallet
          </button>
        </Show>
      </div>
      <img class="fennecImg" src={artFennec} alt="" />
    </div>
  )
}
