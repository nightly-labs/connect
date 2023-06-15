import { unsafeCSS } from 'lit'

import style from './tailwind.global.css?inline'

export const tailwindElement = (customStyle: string) => [unsafeCSS(style), unsafeCSS(customStyle)]
