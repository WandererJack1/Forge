import type { JSX } from 'react'
import { RouterProvider } from 'TheMill-router'
import type { RouterInstanceType } from 'TheMill-router'

import { useTheMillContextServerPayload } from './TheMillContext'

interface RouterContextProviderWrapperProps {
  router: RouterInstanceType
}

/**
 * This component is needed to get the data from {@link TheMillContext}
 * since the provider is also located in {@link TheMillEntryPoint}
 * hence the context cannot be accessed directly there
 *
 * @see https://github.com/TheJackMan33/TheMill/issues/410
 */
export function RouterContextProviderWrapper({
  router,
}: RouterContextProviderWrapperProps): JSX.Element {
  const serverPayload = useTheMillContextServerPayload()

  return (
    <RouterProvider
      router={router}
      serverInitialLocation={serverPayload.location}
      serverInitialData={serverPayload.data}
    />
  )
}
