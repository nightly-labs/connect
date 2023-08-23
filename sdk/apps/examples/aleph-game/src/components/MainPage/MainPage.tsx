import { Component } from 'solid-js'
import Logo from '../../static/svg/Logo.svg'
import { timeFormatter } from '../utils/formatting'
import firstPicture from '../../static/svg/firstPicture.svg'
import secondPicture from '../../static/svg/secondPicture.svg'
import thirdPicture from '../../static/svg/thirdPicture.svg'
import fourthPicture from '../../static/svg/fourthPicture.svg'
import fifthPicture from '../../static/svg/fifthPicture.svg'
import sixthPicture from '../../static/svg/sixthPicture.svg'
import seventhPicture from '../../static/svg/seventhPicture.svg'
import eighthPicture from '../../static/svg/eighthPicture.svg'
import ninthPicture from '../../static/svg/ninthPicture.svg'
import './MainPage.css'

export interface IMainPage {
  counter: string
  time: number
  collectedTicket: boolean
  id: number[]
}

export const MainPage: Component<IMainPage> = (props) => {
  const pictures = [
    { id: 1, src: firstPicture },
    { id: 2, src: secondPicture },
    { id: 3, src: thirdPicture },
    { id: 4, src: fourthPicture },
    { id: 5, src: fifthPicture },
    { id: 6, src: sixthPicture },
    { id: 7, src: seventhPicture },
    { id: 8, src: eighthPicture },
    { id: 9, src: ninthPicture }
  ]

  return (
    <div class="mainGameContainer">
      <img class="logo" src={Logo} alt="" />
      <div class="ticket-status-container">
        <span>
          Collected tickets: <span class="ticketCounterText">{props.counter}/9</span>
        </span>
        <span class="textInfo">
          {+props.counter < 4 && 'Don’t give up! You can do it!'}
          {+props.counter >= 4 && +props.counter < 9 && 'Keep looking... You’re almost there!'}
          {+props.counter === 9 &&
            'Congratulations! Know Nightly better by downloading Nightly Wallet app for Android or iOS.'}
        </span>
        <span class="timeText">
          Time left: <span class="timeCounterText">{timeFormatter(props.time)}</span>
        </span>
      </div>
      <div class="gameContainer">
        {pictures.map((picture, index) => (
          <img
            class="title"
            style={
              props.collectedTicket && props.id.includes(picture.id)
                ? { visibility: 'hidden' }
                : { display: 'grid' }
            }
            src={picture.src}
            alt={`Picture ${index + 1}`}
          />
        ))}
      </div>
    </div>
  )
}
