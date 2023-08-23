export const timeFormatter = (timestamp: number, dayOnly = false) => {
  const time: string = new Date(timestamp).toLocaleTimeString('en-GB', {
    hour: 'numeric',
    minute: 'numeric',
    second: 'numeric'
  })
  if (dayOnly) {
    return time
  }

  return time
}
