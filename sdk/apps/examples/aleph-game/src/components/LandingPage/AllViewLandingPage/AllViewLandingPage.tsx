import { MainPage } from '~/components/MainPage/MainPage'
import { LandingPage } from '../LandingPage'

// THIS COMPONENT IS TO BE REMOVED AFTER REVIEW

export const AllViewLandingPage = () => {
  return (
    <div style={{ display: 'grid', 'grid-template-columns': '1fr 1fr 1fr' }}>
      <div>
        <LandingPage
          hasTicketsToClaim={false}
          isConnected={false}
          onAddTickets={() => {
            console.log('add ticket')
          }}
          onClaimTickets={() => {
            console.log('Claim Ticket')
          }}
          onConnectWallet={() => {
            console.log('Connect Wallet')
          }}
        />
      </div>
      <div style={{ margin: '10px 10px' }}>
        <LandingPage
          hasTicketsToClaim={false}
          isConnected={true}
          onAddTickets={() => {
            console.log('add ticket')
          }}
          onClaimTickets={() => {
            console.log('Claim Ticket')
          }}
          onConnectWallet={() => {
            console.log('Connect Wallet')
          }}
        />
      </div>
      <div>
        <LandingPage
          hasTicketsToClaim={true}
          isConnected={true}
          onAddTickets={() => {
            console.log('add ticket')
          }}
          onClaimTickets={() => {
            console.log('Claim Ticket')
          }}
          onConnectWallet={() => {
            console.log('Connect Wallet')
          }}
        />
      </div>
      <div>
        <MainPage collectedTicket={true} counter="0" id={[]} time={9238974312734} />
      </div>
      <div>
        <MainPage collectedTicket={true} counter="6" id={[1, 2, 8, 5, 7, 6]} time={9238974312734} />
      </div>
      <div>
        <MainPage
          collectedTicket={true}
          counter="9"
          id={[1, 2, 3, 4, 5, 6, 7, 8, 9]}
          time={9238974312734}
        />
      </div>
    </div>
  )
}
