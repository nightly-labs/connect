import type { InjectedAccount, InjectedAccounts, Unsubcall } from '@polkadot/extension-inject/types'

export class Accounts implements InjectedAccounts {
  activeAccounts: InjectedAccount[]
  sub: { [key in string]: (accounts: InjectedAccount[]) => void } = {}
  constructor() {
    this.activeAccounts = []
  }
  private _triggerSubs = () => {
    Object.values(this.sub).forEach((cb) => cb(this.activeAccounts))
  }
  addAccount = (account: InjectedAccount): void => {
    this.activeAccounts.push(account)
    this._triggerSubs()
  }
  updateActiveAccounts = (activeAccounts: InjectedAccount[]): void => {
    this.activeAccounts = activeAccounts
    this._triggerSubs()
  }
  // TODO: what does anyType do?
  get = (anyType?: boolean): Promise<InjectedAccount[]> => {
    return new Promise((resolve, reject) => {
      resolve(this.activeAccounts)
    })
  }

  subscribe = (cb: (accounts: InjectedAccount[]) => unknown): Unsubcall => {
    const id = Date.now().toString()
    this.sub[id] = cb
    // Debounce first response to allow the subscriber to set up
    setTimeout(() => {
      this._triggerSubs()
    }, 100)
    return (): void => {
      delete this.sub[id]
    }
  }
}
