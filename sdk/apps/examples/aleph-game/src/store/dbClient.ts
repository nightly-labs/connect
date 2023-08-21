import { SupabaseClient, createClient } from '@supabase/supabase-js'
import { Database } from 'database.types'
import { createEffect } from 'solid-js'
import { TICKETS_MAP, TicketId, TicketsMapType } from './ticketsMap'

// Create a single supabase client for interacting with your database
let _db: SupabaseClient<Database> | undefined
export function getDb() {
  if (_db) return _db
  else {
    _db = createClient(
      'https://wedgjpxnorpumhixwxkz.supabase.co',
      'eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpc3MiOiJzdXBhYmFzZSIsInJlZiI6IndlZGdqcHhub3JwdW1oaXh3eGt6Iiwicm9sZSI6InNlcnZpY2Vfcm9sZSIsImlhdCI6MTY5MjYyNzcyNiwiZXhwIjoyMDA4MjAzNzI2fQ.kt_OrHfO87xvx9LXtGY_rV6yH_3-F7ZVoznQ0_9fBvM'
    )
    return _db
  }
}
export interface User {
  address: string
  tickets: { [key: string]: number }
}
export const addUserTicket = async (userAddress: string, ticketId: TicketId) => {
  const db = getDb()
  const user = await (
    await db.from('tickets').select('address,tickets').eq('address', userAddress)
  ).data?.[0]
  if (user) {
    const tickets = user.tickets as TicketsMapType
    // @ts-expect-error ignore any
    tickets[ticketId] = TICKETS_MAP[ticketId]
    await db.from('tickets').update({ tickets }).eq('address', userAddress)
  } else {
    await db
      .from('tickets')
      .insert({ address: userAddress, tickets: { [ticketId]: TICKETS_MAP[ticketId] } })
  }
}
export const getUserTickets = async (userAddress: string): Promise<TicketsMapType> => {
  const db = getDb()
  const user = await (
    await db.from('tickets').select('address,tickets').eq('address', userAddress)
  ).data?.[0]
  if (user) {
    return user.tickets as TicketsMapType
  } else {
    return {}
  }
}
createEffect(async () => {
  await addUserTicket('33', 'uh3Rccil')
  await addUserTicket('33', 'NFKZcvMB')
  await addUserTicket('44', 'z8aiJmga')
  const user = await getUserTickets('33')
  console.log(user)
})
