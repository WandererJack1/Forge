import type { ReactNode, JSX } from 'react'
import { TheMillScripts } from 'TheMill'

interface RootLayoutProps {
  children: ReactNode
}

export default function RootLayout({ children }: RootLayoutProps): JSX.Element {
  return (
    <html>
      <body>
        <main>{children}</main>
        <TheMillScripts />
      </body>
    </html>
  )
}
