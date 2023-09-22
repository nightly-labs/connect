import { Title } from 'solid-start'
import fennecNearGame from '../static/svg/fennecNearGame.png'
export default function NotFound() {
  return (
    <main>
      <Title>Chilling room</Title>
      <span style={{ color: 'white', 'font-size': '44px' }}>The event has not yet started</span>
      <span style={{ color: 'white', 'font-size': '44px', 'padding-bottom': '20px' }}>
        It will start :
      </span>
      <strong>
        <span style={{ color: 'white', 'font-size': '50px' }}>02.10.2023 14:00 CET</span>
      </strong>
      <div>
        <img
          src={fennecNearGame}
          style={{ width: '500px', 'border-radius': '32px', 'margin-top': '20px' }}
        />{' '}
      </div>
    </main>
  )
}
