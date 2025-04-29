import type { JSX } from 'react'
import { MDXProvider } from '@mdx-js/react'
import { TheMillScripts } from 'TheMill'
import type { TheMillLayoutProps } from 'TheMill'

import '../styles/global.css'

export default function RootLayout({
  children,
}: TheMillLayoutProps): JSX.Element {
  return (
    <html>
      <body>
        <main>
          <MDXProvider components={{}}>{children}</MDXProvider>
        </main>
        <TheMillScripts />
      </body>
    </html>
  )
}
