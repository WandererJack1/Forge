import { StrictMode } from 'react'
import type { JSX } from 'react'
import type { RouterInstanceType } from 'TheMill-router'

import type { ServerPayload } from '../types'

import { TheMillContextProvider } from './TheMillContext'
import { RouterContextProviderWrapper } from './RouterContextProviderWrapper'

interface TheMillEntryPointProps {
  router: RouterInstanceType
  serverPayload?: ServerPayload
}

export function TheMillEntryPoint({
  router,
  serverPayload,
}: TheMillEntryPointProps): JSX.Element {
  return (
    <StrictMode>
      <TheMillContextProvider serverPayload={serverPayload}>
        <RouterContextProviderWrapper router={router} />
      </TheMillContextProvider>
    </StrictMode>
  )
}
