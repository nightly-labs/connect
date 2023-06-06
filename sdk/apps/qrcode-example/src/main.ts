import { getQrInXml } from '@nightlylabs/qr-code/src/xml'
import './style.css'

document.querySelector<HTMLDivElement>('#app')!.innerHTML = `
  <div>
  ${getQrInXml('DVXCjVUkz7yAvwufKgSBSieHjLtkDDc1jWanAZtSt3FP', {
    width: 200,
    height: 200
  })}
  </div>
`
