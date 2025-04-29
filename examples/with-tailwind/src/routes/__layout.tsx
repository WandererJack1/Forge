import type { JSX } from 'react'
import { TheMillScripts } from 'TheMill'
import type { TheMillLayoutProps } from 'TheMill'

import '../styles/global.css'

export default function RootLayout({
  children,
}: TheMillLayoutProps): JSX.Element {
  return (
    <html>
      <head>
        <meta name="viewport" content="width=device-width, initial-scale=1" />
      </head>
      <body>
        <main>{children}</main>
        <TheMillScripts />
      </body>
    </html>
  )
}
