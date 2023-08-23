import { Component, Show } from 'solid-js'
import Logo from '../../static/svg/Logo.svg'
import richFennec from '../../static/svg/richFennec.svg'
import poorFennec from '../../static/svg/poorFennec.svg'
import './ResolvePage.css'

export interface IResolvePage {
  resolve: boolean
}

export const ResolvePage: Component<IResolvePage> = (props) => {
  return (
    <Show
      when={props.resolve}
      fallback={
        <div class="mainResolveContainer">
          <img class="resolveLogo" src={Logo} alt="" />
          <div class="textResolveContainer">
            <span class="loseHeader">Awww... :(!</span>
            <span class="loseTextInfo">
              The luck wasn't on your side today... But don’t worry, we’ll meet again with new
              challenges in the future!{' '}
            </span>
          </div>
          <img class="poorfennec" src={poorFennec} alt="" />
        </div>
      }>
      <div class="mainResolveContainer">
        <img class="resolveLogo" src={Logo} alt="" />
        <div class="textResolveContainer">
          <span class="winHeader">Congratulations!</span>
          <span class="loseTextInfo">
            You’ve won lorem ipsum! Your reward will be automatically send on your wallet.
          </span>
        </div>
        <img class="richFennec" src={richFennec} alt="" />
      </div>
    </Show>
  )
}
