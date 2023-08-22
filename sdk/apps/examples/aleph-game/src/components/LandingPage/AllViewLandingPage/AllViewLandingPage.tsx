import { LandingPage } from '../LandingPage'

// THIS COMPONENT IS TO BE REMOVED AFTER REVIEW

export const AllViewLandingPage = () => {
  return (
    <div style={{ display: 'flex' }}>
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
      <div style={{ margin: '0 10px' }}>
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
    </div>
  )
}
