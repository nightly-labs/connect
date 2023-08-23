import { SupabaseClient, createClient } from '@supabase/supabase-js'
import { Database } from 'database.types'
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
enum WinnerType {
  FirstAll = 'FirstAll',
  FirstThree = 'FirstThree',
  Random = 'Random'
}
export const getFirstAllWinner = async (): Promise<Set<string>> => {
  const db = getDb()
  const user = await (
    await db.from('winners').select('name,addresses').eq('name', WinnerType.FirstAll)
  ).data?.[0]
  if (user) {
    return new Set(JSON.parse(user.addresses!))
  } else {
    return new Set()
  }
}
export const setFirstAllWinner = async (userAddress: string) => {
  const firstWinner = await getFirstAllWinner()
  // Only set the first winner
  if (firstWinner.size > 0) return
  const db = getDb()
  await db
    .from('winners')
    .upsert({ name: WinnerType.FirstAll, addresses: JSON.stringify([userAddress]) })
}

export const getFirstThreeWinner = async (): Promise<Set<string>> => {
  const db = getDb()
  const user = await (
    await db.from('winners').select('name,addresses').eq('name', WinnerType.FirstThree)
  ).data?.[0]
  if (user) {
    return new Set(JSON.parse(user.addresses!))
  } else {
    return new Set()
  }
}

export const setFirstThreeWinner = async (userAddress: string) => {
  const winners = await getFirstThreeWinner()
  // Only set the first winner
  if (winners.size >= 50) return
  const db = getDb()
  winners.add(userAddress)
  await db
    .from('winners')
    .upsert({ name: WinnerType.FirstThree, addresses: JSON.stringify(Array.from(winners)) })
}

export const getRandomWinner = async (): Promise<Set<string>> => {
  const db = getDb()
  const user = await (
    await db.from('winners').select('name,addresses').eq('name', WinnerType.Random)
  ).data?.[0]
  if (user) {
    return new Set(JSON.parse(user.addresses!))
  } else {
    return new Set()
  }
}
export const setRandomWinner = async (userAddress: string) => {
  const winners = await getRandomWinner()
  // Only set the first winner
  if (winners.size >= 5) return
  const db = getDb()
  winners.add(userAddress)
  await db
    .from('winners')
    .upsert({ name: WinnerType.Random, addresses: JSON.stringify(Array.from(winners)) })
}

// createEffect(async () => {
//   await setFirstAllWinner('first winner')
//   await setFirstAllWinner('2nd winner')
//   await setFirstAllWinner('3rd winner')
//   const first = await getFirstAllWinner()
//   console.log('first', first.has('first winner'))
// })

// createEffect(async () => {
//   await setFirstThreeWinner('1st winner')
//   await setFirstThreeWinner('2nd winner')
//   await setFirstThreeWinner('3rd winner')
//   await setFirstThreeWinner('4rd winner')
//   await setFirstThreeWinner('5rd winner')
//   const first3 = await getFirstThreeWinner()
//   console.log('first3', first3)
// })

// createEffect(async () => {
//   await setRandomWinner('1st winner')
//   await setRandomWinner('2nd winner')
//   await setRandomWinner('3rd winner')
//   await setRandomWinner('4rd winner')
//   await setRandomWinner('5rd winner')
//   await setRandomWinner('6rd winner')
//   await setRandomWinner('7rd winner')
//   const firstrandom = await getRandomWinner()
//   console.log('firstrandom', firstrandom)
// })
