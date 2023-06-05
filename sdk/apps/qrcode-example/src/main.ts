import { getQrInXml } from '@nightlylabs/qr-code/src/xml'
import './style.css'

document.querySelector<HTMLDivElement>('#app')!.innerHTML = `
  <div>
  ${getQrInXml('sfdfghgdfgfgdffsfdfgfggfggdfhggdfgxhchfjfjcgjfchfchfch', {
    width: 400,
    height: 400
  })}
  </div>
`
