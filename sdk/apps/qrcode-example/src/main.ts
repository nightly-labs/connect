import { generateQrCodeXml, generateQrCodeSvgElement } from '@nightlylabs/qr-code'
import './style.css'

document.querySelector<HTMLDivElement>('#app')!.innerHTML = `
  <div>
  qr directly from xml string: <br />
  ${generateQrCodeXml('DVXCjVUkz7yAvwufKgSBSieHjLtkDDc1jWanAZtSt3FP', {
    width: 200,
    height: 200
  })}
  <br />
  qr in appended svg node: <br />
  <div id="qr2"></div>
  </div>
`
document.getElementById('qr2')?.appendChild(
  generateQrCodeSvgElement('DVXCjVUkz7yAvwufKgSBSieHjLtkDDc1jWanAZtSt3FP', {
    width: 200,
    height: 200
  })
)
