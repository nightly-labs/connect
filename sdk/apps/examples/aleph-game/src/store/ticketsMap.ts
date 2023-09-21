export const TICKETS_MAP = {
  // 9 random tickets to index
  Oq6PjIHod9: 1,
  '3l9VgG9uhu': 2,
  Wa8tOKb9FL: 3,
  PU6vP65fu2: 4,
  abvYiMHOSa: 5,
  FKEzmQy0Pk: 6,
  UACtREt7st: 7,
  Kox6D1FAj1: 8,
  a8vSAdyEmE: 9
}
export type TicketId = keyof typeof TICKETS_MAP
export type TicketValue = typeof TICKETS_MAP[TicketId]
export type TicketsMapType = {
  TicketId?: number
}

// http://near.game.nightly.app/ticket/Oq6PjIHod9
// http://near.game.nightly.app/ticket/3l9VgG9uhu
// http://near.game.nightly.app/ticket/Wa8tOKb9FL
// http://near.game.nightly.app/ticket/PU6vP65fu2
// http://near.game.nightly.app/ticket/abvYiMHOSa
// http://near.game.nightly.app/ticket/FKEzmQy0Pk
// http://near.game.nightly.app/ticket/UACtREt7st
// http://near.game.nightly.app/ticket/Kox6D1FAj1
// http://near.game.nightly.app/ticket/a8vSAdyEmE
