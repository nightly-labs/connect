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
