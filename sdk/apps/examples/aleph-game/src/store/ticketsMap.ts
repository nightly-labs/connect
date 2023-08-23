export const TICKETS_MAP = {
  // 9 random tickets to index
  uh3Rccil: 1,
  a8UISBYU: 2,
  NFKZcvMB: 3,
  '5ryVaSo0': 4,
  z8aiJmga: 5,
  '8wGsgPHa': 6,
  rK0IQ0LJ: 7,
  H6kwvuqg: 8,
  eibElg0N: 9
}
export type TicketId = keyof typeof TICKETS_MAP
export type TicketValue = typeof TICKETS_MAP[TicketId]
export type TicketsMapType = {
  TicketId?: number
}

// https://aleph.game.nightly.app/ticket/uh3Rccil
// https://aleph.game.nightly.app/ticket/a8UISBYU
// https://aleph.game.nightly.app/ticket/NFKZcvMB
// https://aleph.game.nightly.app/ticket/5ryVaSo0
// https://aleph.game.nightly.app/ticket/z8aiJmga
// https://aleph.game.nightly.app/ticket/8wGsgPHa
// https://aleph.game.nightly.app/ticket/rK0IQ0LJ
// https://aleph.game.nightly.app/ticket/H6kwvuqg
// https://aleph.game.nightly.app/ticket/eibElg0N
