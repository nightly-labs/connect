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

// https://fountain-zimbabwe-kay-training.trycloudflare.com/ticket/uh3Rccil
// https://fountain-zimbabwe-kay-training.trycloudflare.com/ticket/a8UISBYU
// https://fountain-zimbabwe-kay-training.trycloudflare.com/ticket/NFKZcvMB
// https://fountain-zimbabwe-kay-training.trycloudflare.com/ticket/5ryVaSo0
// https://fountain-zimbabwe-kay-training.trycloudflare.com/ticket/z8aiJmga
// https://fountain-zimbabwe-kay-training.trycloudflare.com/ticket/8wGsgPHa
// https://fountain-zimbabwe-kay-training.trycloudflare.com/ticket/rK0IQ0LJ
// https://fountain-zimbabwe-kay-training.trycloudflare.com/ticket/H6kwvuqg
// https://fountain-zimbabwe-kay-training.trycloudflare.com/ticket/eibElg0N
