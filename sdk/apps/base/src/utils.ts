import { v4 as uuidv4 } from 'uuid'
export const getRandomId = () => uuidv4()
export function sleep(ms: number) {
  return new Promise((resolve) => setTimeout(resolve, ms))
}
