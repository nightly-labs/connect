import type {
  SolanaChain,
  SolanaSignAndSendTransactionFeature,
  SolanaSignAndSendTransactionMethod,
  SolanaSignMessageFeature,
  SolanaSignMessageMethod,
  SolanaSignTransactionFeature,
  SolanaSignTransactionMethod
} from '@solana/wallet-standard'
import { SOLANA_CHAINS, getEndpointForChain } from '@solana/wallet-standard'
import type {
  StandardConnectFeature,
  StandardConnectMethod,
  StandardEventsFeature,
  Wallet,
  StandardEventsNames,
  StandardEventsListeners,
  WalletAccount,
  StandardEventsOnMethod,
  IdentifierArray
} from '@wallet-standard/core'
import { AppSolana } from '@nightlylabs/connect-solana/src/app'
import { Connection, PublicKey, VersionedTransaction } from '@solana/web3.js'

export class NightlyConnectSolanaWallet implements Wallet {
  #app: AppSolana

  #listeners: { [E in StandardEventsNames]: StandardEventsListeners[E][] } = {
    change: []
  }

  get version() {
    return '1.0.0' as const
  }

  get accounts() {
    return this.#accounts.slice()
  }

  #on: StandardEventsOnMethod = (event, listener) => {
    this.#listeners[event].push(listener)

    return () => this.#off(event, listener)
  }

  #off<E extends StandardEventsNames>(event: E, listener: StandardEventsListeners[E]): void {
    this.#listeners[event] = this.#listeners[event]?.filter(
      (existingListener) => listener !== existingListener
    )
  }

  #accounts: WalletAccount[]

  #name = 'Nightly Connect' as const
  #icon =
    'data:image/svg+xml;base64,PD94bWwgdmVyc2lvbj0iMS4wIiBlbmNvZGluZz0idXRmLTgiPz4NCjwhLS0gR2VuZXJhdG9yOiBBZG9iZSBJbGx1c3RyYXRvciAyNi4zLjEsIFNWRyBFeHBvcnQgUGx1Zy1JbiAuIFNWRyBWZXJzaW9uOiA2LjAwIEJ1aWxkIDApICAtLT4NCjxzdmcgdmVyc2lvbj0iMS4xIiB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHhtbG5zOnhsaW5rPSJodHRwOi8vd3d3LnczLm9yZy8xOTk5L3hsaW5rIiB4PSIwcHgiIHk9IjBweCINCgkgdmlld0JveD0iMCAwIDEwODAgMTA4MCIgc3R5bGU9ImVuYWJsZS1iYWNrZ3JvdW5kOm5ldyAwIDAgMTA4MCAxMDgwOyIgeG1sOnNwYWNlPSJwcmVzZXJ2ZSI+DQo8c3R5bGUgdHlwZT0idGV4dC9jc3MiPg0KCS5zdDB7ZmlsbDojNjA2N0Y5O30NCgkuc3Qxe2ZpbGw6I0ZGRkZGRjt9DQo8L3N0eWxlPg0KPGcgaWQ9IldhcnN0d2FfMiI+DQoJPGc+DQoJCTxjaXJjbGUgY2xhc3M9InN0MCIgY3g9IjU0MCIgY3k9IjU0MCIgcj0iNTQwIi8+DQoJCTxnPg0KCQkJPGc+DQoJCQkJPHBhdGggY2xhc3M9InN0MCIgZD0iTTQ3Ny45LDczNC42Yy0yLjEtNjguNC02OS41LTkwLjQtMTE1LjUtNjEuNmMwLDAsNy4yLDI5LjQsNDAuOCw0NC4yQzQzMC42LDcyOS4zLDQ0Mi40LDcxMy4xLDQ3Ny45LDczNC42eg0KCQkJCQkiLz4NCgkJCQk8cGF0aCBjbGFzcz0ic3QwIiBkPSJNMjUzLjEsMjU5LjhjLTI3LjEsOTUuMS0xNS40LDIxNS40LDI3LjQsMzAzYzQzLjgtMzEsODkuOC03OC45LDExNC4zLTEyOS42DQoJCQkJCUMzMzcuMSwzOTAuNSwyODguNywzNDkuOSwyNTMuMSwyNTkuOHoiLz4NCgkJCQk8cGF0aCBjbGFzcz0ic3QwIiBkPSJNNjAyLjEsNzM0LjZjMi4xLTY4LjQsNjkuNS05MC40LDExNS41LTYxLjZjMCwwLTcuMiwyOS40LTQwLjgsNDQuMkM2NDkuNCw3MjkuMyw2MzcuNiw3MTMuMSw2MDIuMSw3MzQuNnoiDQoJCQkJCS8+DQoJCQkJPHBhdGggY2xhc3M9InN0MCIgZD0iTTgyNi45LDI1OS44YzI3LjEsOTUuMSwxNS40LDIxNS40LTI3LjQsMzAzYy00My44LTMxLTg5LjgtNzguOS0xMTQuMy0xMjkuNg0KCQkJCQlDNzQyLjksMzkwLjUsNzkxLjMsMzQ5LjksODI2LjksMjU5Ljh6Ii8+DQoJCQkJPGVsbGlwc2UgY2xhc3M9InN0MCIgY3g9IjU0MCIgY3k9IjkyMS4zIiByeD0iNDMuNiIgcnk9IjM4LjQiLz4NCgkJCTwvZz4NCgkJPC9nPg0KCTwvZz4NCjwvZz4NCjxnIGlkPSJXYXJzdHdhXzMiPg0KCTxwYXRoIGNsYXNzPSJzdDEiIGQ9Ik01NDAuMyw5NTkuOGMtMC40LDAtMC44LDAtMSwwYy0wLjIsMC0wLjUsMC0xLDBjLTUuNywwLTI2LTAuOC00OC44LTEwLjhjLTI0LjQtMTAuNy01Ni42LTM0LjUtNzEuMi04Nw0KCQljLTEzLjEtNDYuOC00My4zLTY3LjUtNjYuMy03Ni42Yy0xNC4xLTUuNi0yOS44LTguNS00NS42LTguNWMtMTcuNSwwLTM0LjIsMy43LTQ0LjUsMTBsLTUuMiwzLjFsLTIuNi01LjQNCgkJYy0yMy43LTQ5LTM2LjQtMTAyLjEtMjEuNi0yMDJjLTEuMy01LjEtMy41LTExLjctNi4yLTIwYy05LjktMzAuMi0yNi40LTgwLjgtMzAuOC0xNDUuNmMtNS4zLTc4LjMsOC42LTE1Ni43LDQxLjQtMjMzbDQtOS4zDQoJCWw1LjYsOC40YzM3LjgsNTYuNyw5Mi4xLDk5LjIsMTMxLjEsMTI0LjhjMzYuOCwyNC4zLDY3LjYsMzguNiw3NC42LDQxLjhjOC42LTIsNDcuOC0xMC41LDgxLjMtMTEuNXYtMC4xbDExLjMsMHYwLjENCgkJYzMzLjUsMC45LDcyLjcsOS41LDgxLjMsMTEuNWM3LjEtMy4yLDM3LjgtMTcuNSw3NC42LTQxLjhjMzktMjUuNiw5My4zLTY4LjEsMTMxLjEtMTI0LjhsNS42LTguNWw0LDkuMw0KCQljMzIuOCw3Ni4zLDQ2LjcsMTU0LjcsNDEuNCwyMzNjLTQuNCw2NC44LTIwLjksMTE1LjQtMzAuOCwxNDUuNmMtMi43LDguMy00LjksMTQuOS02LjIsMjBjMTQuOCw5OS45LDIuMSwxNTMuMS0yMS42LDIwMmwtMi42LDUuNA0KCQlsLTUuMi0zLjFjLTEwLjMtNi4zLTI3LTEwLTQ0LjUtMTBjLTE1LjcsMC0zMS41LDIuOS00NS42LDguNWMtMjMsOS4xLTUzLjIsMjkuOC02Ni4zLDc2LjZjLTE0LjYsNTIuNC00Ni44LDc2LjItNzEuMiw4Nw0KCQlDNTY2LjQsOTU5LDU0Niw5NTkuOCw1NDAuMyw5NTkuOHogTTUzOS4zLDg3OS4zYy0yMSwwLjEtMzcuMywxMy43LTM3LjksMzEuN2MtMC4zLDcuNiwyLjksMTUuMiw5LDIxLjVjNy42LDcuOSwxOC40LDEyLjYsMjksMTIuNg0KCQljMTAuNSwwLDIxLjMtNC44LDI4LjktMTIuNmM2LjEtNi4zLDkuMi0xMy45LDktMjEuNUM1NzYuNiw4OTMsNTYwLjMsODc5LjQsNTM5LjMsODc5LjN6IE00MDQuNSw4MTAuNGMxMSwxMy42LDE5LjMsMjkuOSwyNC41LDQ4LjcNCgkJYzEzLjUsNDguMyw0Mi44LDcwLjEsNjUsNzkuOGM0LjIsMS44LDguMiwzLjMsMTIuMSw0LjVjLTEuMy0xLjEtMi41LTIuMi0zLjYtMy40Yy04LjEtOC40LTEyLjQtMTguOS0xMi4xLTI5LjUNCgkJYzAuNC0xMiw2LTIyLjksMTUuNC0zMC44Yy0xNC0yNC41LTI0LTUxLTI5LjYtNzguNmMtNC41LTIyLjItNi4yLTQ0LjktNC45LTY3LjVjLTEzLjUsNS4zLTMzLjEsMTYtNDguMywzNi40DQoJCUM0MTQuMSw3ODIsNDA3LjksNzk1LjYsNDA0LjUsODEwLjR6IE01NzIuOCw4NzkuOGM5LjQsNy44LDE1LDE4LjgsMTUuNCwzMC44YzAuNCwxMC42LTMuOSwyMS0xMi4xLDI5LjVjLTEuMSwxLjItMi4zLDIuMy0zLjYsMy40DQoJCWMzLjktMS4yLDgtMi43LDEyLjEtNC41YzIyLjItOS44LDUxLjUtMzEuNiw2NS03OS44YzUuMi0xOC43LDEzLjUtMzUuMSwyNC41LTQ4LjdjLTMuNC0xNC44LTkuNy0yOC40LTE4LjYtNDAuMw0KCQljLTE1LjItMjAuNC0zNC44LTMxLjEtNDguMy0zNi40YzEuMywyMi43LTAuNCw0NS4zLTQuOSw2Ny41QzU5Ni44LDgyOC44LDU4Ni45LDg1NS4zLDU3Mi44LDg3OS44eiBNNDgyLjcsNzI4LjENCgkJYy0xLjksMjMuOC0wLjUsNDcuNiw0LjMsNzAuOWM1LjQsMjYuMywxNC44LDUxLjQsMjguMSw3NC44YzUuNy0yLjgsMTItNC42LDE4LjYtNS4yVjcwOS4zQzUyNC40LDcwOS4zLDUwMiw3MTEuMyw0ODIuNyw3MjguMXoNCgkJIE01NDUsODY4LjZjNi42LDAuNywxMi45LDIuNCwxOC42LDUuMmMxMy4zLTIzLjQsMjIuOC00OC41LDI4LjEtNzQuOGM0LjctMjMuMyw2LjItNDcuMiw0LjMtNzAuOWMtMTkuMy0xNi43LTQxLjctMTguOC01MC45LTE4LjgNCgkJVjg2OC42eiBNNjEzLjYsNzI0LjNjMTQuNyw2LjEsMzQuOSwxNy44LDUwLjksMzkuMWM4LjQsMTEuMiwxNC42LDIzLjcsMTguNiwzNy4yYzExLjItMTEsMjQuNS0xOS42LDM5LjYtMjUuNQ0KCQljMTUuMy02LjEsMzIuNS05LjMsNDkuNi05LjNjMTUsMCwyOS4yLDIuNSw0MC41LDYuOWMtNy44LTQ4LjItMzIuMy03My41LTUyLTg2LjRjLTE2LjktMTEtMzMuMi0xNS4xLTQwLjUtMTYuNA0KCQljLTIuNCw1LjQtNy41LDE1LjItMTYuMiwyNC42Yy0xMC41LDExLjMtMjguOSwyNC43LTU3LjUsMjQuN2MtMywwLTYuMS0wLjItOS4yLTAuNWMtMC45LTAuMS0xLjktMC4xLTIuOC0wLjENCgkJQzYyNi43LDcxOC43LDYxOS4yLDcyMS41LDYxMy42LDcyNC4zeiBNMzA2LjUsNzY1LjljMTcuMSwwLDM0LjIsMy4yLDQ5LjYsOS4zYzE1LjEsNiwyOC40LDE0LjUsMzkuNiwyNS41DQoJCWM0LTEzLjUsMTAuMi0yNiwxOC42LTM3LjJjMTUuOS0yMS40LDM2LjEtMzMsNTAuOS0zOS4xYy01LjYtMi44LTEzLjItNS43LTIwLjktNS43Yy0wLjksMC0xLjksMC0yLjgsMC4xYy0zLjEsMC4zLTYuMiwwLjUtOS4yLDAuNQ0KCQljLTI4LjUsMC00Ny0xMy40LTU3LjUtMjQuN2MtOC43LTkuMy0xMy44LTE5LjEtMTYuMi0yNC42Yy03LjMsMS4zLTIzLjYsNS40LTQwLjUsMTYuNWMtMTkuNywxMi45LTQ0LjIsMzguMi01Miw4Ni40DQoJCUMyNzcuMyw3NjguMywyOTEuNSw3NjUuOSwzMDYuNSw3NjUuOXogTTI0My4yLDU4NS43Yy0xMi40LDg1LjMtNC4zLDEzNC44LDEzLjMsMTc3LjRjOS44LTQ3LDM0LjktNzIuNSw1NS40LTg1LjkNCgkJYzExLjktNy44LDIzLjUtMTIuNCwzMi42LTE1LjFjLTE3LjMtOS4xLTM5LjYtMjUuNC01NC41LTUzLjFjLTcuNC0xMy42LTEyLjEtMjguNC0xNC00NEMyNjUuMyw1NzIuMywyNTQuMyw1NzkuMiwyNDMuMiw1ODUuN3oNCgkJIE03MzQuMSw2NjJjOS4xLDIuNywyMC42LDcuMywzMi42LDE1LjFjMjAuNSwxMy40LDQ1LjYsMzguOSw1NS40LDg1LjljMTcuNi00Mi42LDI1LjgtOTIuMSwxMy4zLTE3Ny40DQoJCWMtMTEuMS02LjQtMjIuMS0xMy40LTMyLjgtMjAuN2MtMiwxNS42LTYuNywzMC40LTE0LDQ0Qzc3My43LDYzNi43LDc1MS40LDY1Mi45LDczNC4xLDY2MnogTTQ0NC4yLDcwNy43YzEwLjUsMCwyMC41LDQsMjcuNCw3LjYNCgkJYy0yLjQtMTQuOC0xMi4xLTM1LjQtMjkuNS00Ny41Yy0xMC4xLTctMjEuNy0xMC42LTM0LjUtMTAuNmMtMTEuOSwwLTI0LjksMy4xLTM4LjUsOS40YzIuMiw0LjgsNi42LDEyLjgsMTMuNywyMC41DQoJCWM5LDkuNywyNC44LDIxLjIsNDkuNCwyMS4yYzIuNywwLDUuNC0wLjEsOC4yLTAuNEM0NDEuNiw3MDcuNyw0NDIuOSw3MDcuNyw0NDQuMiw3MDcuN3ogTTY3MS4xLDY1Ny4yYy0xMi44LDAtMjQuNCwzLjYtMzQuNSwxMC42DQoJCWMtMTcuNCwxMi4xLTI3LjIsMzIuNy0yOS41LDQ3LjVjNi45LTMuNywxNi44LTcuNiwyNy40LTcuNmMxLjMsMCwyLjYsMC4xLDMuOSwwLjJjMi44LDAuMyw1LjUsMC40LDguMiwwLjQNCgkJYzI0LjYsMCw0MC40LTExLjUsNDkuNC0yMS4yYzcuMS03LjcsMTEuNS0xNS42LDEzLjctMjAuNUM2OTYsNjYwLjQsNjgzLjEsNjU3LjIsNjcxLjEsNjU3LjJ6IE01NDUsNjk4LjNjOS40LDAsMzAuOSwxLjksNTEuMSwxNi4xDQoJCWMyLjQtMTYuNiwxMi4xLTQwLjIsMzQuMi01NS42YzEyLTguMywyNS43LTEyLjUsNDAuNy0xMi41YzEyLjMsMCwyNS40LDIuOSwzOS4xLDguNmMtMy0yMi41LTEyLjQtNTcuOC00MS04OC44DQoJCWMtNDMuMS00Ni43LTEwMC45LTUxLjgtMTIzLjgtNTEuOGMtMC4yLDAtMC4zLDAtMC41LDBWNjk4LjN6IE00MDcuNSw2NDYuMmMxNS4xLDAsMjguOCw0LjIsNDAuNywxMi41YzIyLjEsMTUuNCwzMS45LDM5LDM0LjIsNTUuNg0KCQljMjAuMy0xNC4yLDQxLjgtMTYsNTEuMS0xNi4xVjUxNC4zYy0wLjIsMC0wLjMsMC0wLjUsMGMtMjIuOSwwLTgwLjcsNS0xMjMuNyw1MS44Yy0yOC42LDMxLTM4LDY2LjItNDEsODguOA0KCQlDMzgyLjEsNjQ5LjEsMzk1LjIsNjQ2LjIsNDA3LjUsNjQ2LjJ6IE01NDUuOSw1MDMuM2MxMC41LDAsMjcuMSwxLDQ2LjIsNS43YzM0LDguNCw2Mi43LDI1LjEsODUuMyw0OS42DQoJCWMzMS42LDM0LjMsNDEuMyw3My40LDQ0LjEsOTcuM2MxNi43LTcuNCw0MS42LTIyLjgsNTcuNS01Mi4yYzcuNi0xNC4xLDEyLjEtMjkuNiwxMy40LTQ2Yy0zMS41LTIyLjUtNjAuNi00OC41LTg2LjYtNzcuMQ0KCQljLTEzLjMtMTQuNy0yNi0zMC40LTM3LjgtNDYuN2MtMTcuNy04LjMtMzYuMy0xNC44LTU1LjQtMTkuMmMtMjAuNi00LjgtNDEuOS03LjItNjMuMS03LjJjLTEuNSwwLTIuOSwwLTQuNCwwdjk1LjgNCgkJQzU0NS4zLDUwMy4zLDU0NS42LDUwMy4zLDU0NS45LDUwMy4zeiBNMjg2LjQsNTU3LjhjMS4zLDE2LjQsNS44LDMxLjgsMTMuNCw0NmMxNS44LDI5LjQsNDAuOCw0NC44LDU3LjUsNTIuMg0KCQljMi45LTIzLjksMTIuNS02My4xLDQ0LjEtOTcuM2MyMi42LTI0LjUsNTEuMy00MS4xLDg1LjMtNDkuNmMxOS4xLTQuNywzNS42LTUuNyw0Ni4xLTUuN2MwLjMsMCwwLjYsMCwwLjksMHYtOTUuOA0KCQljLTEuNSwwLTIuOSwwLTQuNCwwYy0yMS4zLDAtNDIuNSwyLjQtNjMuMiw3LjJjLTE5LDQuNC0zNy42LDEwLjktNTUuNCwxOS4yYy0xMS44LDE2LjMtMjQuNSwzMi0zNy44LDQ2LjcNCgkJQzM0Nyw1MDkuMywzMTcuOSw1MzUuMiwyODYuNCw1NTcuOHogTTgwNC44LDU1My4xYzEwLjUsNy4zLDIxLjMsMTQuMywzMi4zLDIwLjdjMS4zLTQuMywyLjktOS4zLDQuNy0xNC44DQoJCWMxOC40LTU2LjMsNjAuNS0xODUuMS0xLjItMzQ4LjJjLTAuNCwzLjgtMSw3LjUtMS42LDExLjNjLTEuNyw5LjctNC4xLDE5LjMtNy4yLDI4LjdDODU4LDM0My41LDg0OC4yLDQ1My40LDgwNC44LDU1My4xeg0KCQkgTTIzOCwyMTAuOWMtNjEuNiwxNjMuMS0xOS42LDI5Mi0xLjIsMzQ4LjJjMS44LDUuNSwzLjQsMTAuNCw0LjcsMTQuOGMxMC45LTYuNSwyMS44LTEzLjQsMzIuMy0yMC43DQoJCWMtNDMuMy05OS43LTUzLjItMjA5LjYtMjctMzAyLjNjLTMuMS05LjQtNS41LTE5LTcuMi0yOC43QzIzOSwyMTguNCwyMzguNCwyMTQuNiwyMzgsMjEwLjl6IE0yNTMuNywyNjguMQ0KCQljLTIwLjUsODYuNS0xMCwxODYuOCwyOS4zLDI3OC41YzI5LjctMjEuNiw1Ny4yLTQ2LjIsODEuOC03My40YzEyLTEzLjMsMjMuNi0yNy40LDM0LjMtNDIuMUMzMzAuOSwzOTAsMjgzLjIsMzM2LjUsMjUzLjcsMjY4LjF6DQoJCSBNNjc5LjUsNDMxLjJjMTAuOCwxNC42LDIyLjMsMjguOCwzNC4zLDQyLjFjMjQuNiwyNy4xLDUyLjEsNTEuOCw4MS44LDczLjRjMzkuMy05MS43LDQ5LjgtMTkyLDI5LjMtMjc4LjUNCgkJQzc5NS41LDMzNi41LDc0Ny44LDM5MCw2NzkuNSw0MzEuMnogTTI1Ny43LDI0OC44YzI4LDczLjcsNzYuNCwxMzAuNSwxNDcuOCwxNzMuNGMxNC40LTIwLjQsMjcuNC00MiwzOC44LTY0LjINCgkJYy0xMS4yLTUuMi0zOS40LTE5LjEtNzIuNi00MWMtMzYuMS0yMy44LTg1LjUtNjEuOS0xMjMuMy0xMTIuNWMwLjUsNS4zLDEuMiwxMC41LDIuMSwxNS43QzI1Mi4yLDIyOS45LDI1NC42LDIzOS41LDI1Ny43LDI0OC44eg0KCQkgTTYzNC4zLDM1OGMxMS40LDIyLjIsMjQuNCw0My44LDM4LjgsNjQuMmM3MS41LTQzLDExOS45LTk5LjcsMTQ3LjgtMTczLjRjMy4xLTkuMyw1LjYtMTguOSw3LjMtMjguNmMwLjktNS4yLDEuNi0xMC40LDIuMS0xNS43DQoJCWMtMzcuOSw1MC42LTg3LjIsODguNy0xMjMuMywxMTIuNUM2NzMuOCwzMzguOSw2NDUuNSwzNTIuOCw2MzQuMywzNTh6IE01NDkuNCwzOTYuNWMyMi4xLDAsNDQuMiwyLjUsNjUuNiw3LjUNCgkJYzEzLjksMy4yLDI3LjYsNy41LDQxLDEyLjhjLTEyLjEtMTguMi0yMy4yLTM3LjItMzMtNTYuN2MtOS41LTIuMS00Ni42LTEwLjEtNzguMS0xMXY0Ny40QzU0Ni41LDM5Ni41LDU0OCwzOTYuNSw1NDkuNCwzOTYuNXoNCgkJIE00NTUuNiwzNjAuMmMtOS44LDE5LjUtMjAuOSwzOC41LTMzLDU2LjdjMTMuNC01LjMsMjcuMS05LjYsNDEtMTIuOGMyMS40LTUsNDMuNS03LjUsNjUuNi03LjVjMS41LDAsMi45LDAsNC40LDB2LTQ3LjQNCgkJQzUwMi4yLDM1MC4xLDQ2NS4xLDM1OCw0NTUuNiwzNjAuMnoiLz4NCjwvZz4NCjxnIGlkPSJXYXJzdHdhXzQiPg0KPC9nPg0KPC9zdmc+DQo=' as const

  get name() {
    return this.#name
  }

  get icon() {
    return this.#icon
  }

  get chains() {
    return SOLANA_CHAINS.slice()
  }

  get features(): StandardConnectFeature &
    StandardEventsFeature &
    SolanaSignTransactionFeature &
    SolanaSignAndSendTransactionFeature &
    SolanaSignMessageFeature {
    return {
      'standard:connect': {
        version: '1.0.0',
        connect: this.#connect
      },
      'standard:events': {
        version: '1.0.0',
        on: this.#on
      },
      'solana:signTransaction': {
        version: '1.0.0',
        supportedTransactionVersions: ['legacy'],
        signTransaction: this.#signTransaction
      },
      'solana:signAndSendTransaction': {
        version: '1.0.0',
        supportedTransactionVersions: ['legacy'],
        signAndSendTransaction: this.#signAndSendTransaction
      },
      'solana:signMessage': {
        version: '1.0.0',
        signMessage: this.#signMessage
      }
    }
  }

  constructor(app: AppSolana, publicKey: PublicKey) {
    this.#app = app
    this.#accounts = [
      {
        address: publicKey.toString(),
        publicKey: publicKey.toBytes(),
        chains: this.chains,
        features: Object.keys(this.features) as IdentifierArray
      }
    ]
  }

  #connect: StandardConnectMethod = async () => {
    return { accounts: this.accounts }
  }

  #signTransaction: SolanaSignTransactionMethod = async (...inputs) => {
    return await Promise.all(
      inputs.map(async ({ transaction }) => {
        const signed = await this.#app.signVersionedTransaction(
          VersionedTransaction.deserialize(transaction)
        )

        return {
          signedTransaction: signed.serialize()
        }
      })
    )
  }

  #signAndSendTransaction: SolanaSignAndSendTransactionMethod = async (...inputs) => {
    return await Promise.all(
      inputs.map(async ({ transaction, chain, options }) => {
        if (!this.chains.includes(chain as SolanaChain)) {
          throw new Error('invalid chain')
        }

        const endpoint = getEndpointForChain(chain as SolanaChain)
        const signedTx = await this.#app.signVersionedTransaction(
          VersionedTransaction.deserialize(transaction)
        )

        const connection = new Connection(endpoint, options?.commitment ?? 'confirmed')

        const signature = await connection.sendRawTransaction(signedTx.serialize(), options)

        return {
          signature: new TextEncoder().encode(signature)
        }
      })
    )
  }

  #signMessage: SolanaSignMessageMethod = async (...inputs) => {
    return await Promise.all(
      inputs.map(async ({ message }) => {
        const signature = await this.#app.signMessage(new TextDecoder().decode(message))

        return {
          signedMessage: message,
          signature
        }
      })
    )
  }
}
