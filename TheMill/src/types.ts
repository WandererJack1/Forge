import type { ReactNode } from 'react'

import type { TheMillConfigServer } from './config'

/**
 * Provided by the rust server and used in the ssr env
 * @see TheMill-router {@link ServerInitialLocation}
 */
export interface ServerPayloadLocation {
  href: string
  pathname: string
  searchStr: string
}

/**
 * @see crates/TheMill_lib/src/payload.rs
 */
export type ServerPayload<TData = unknown> = {
  location: ServerPayloadLocation

  data: TData
} & (
  | {
      mode: 'Prod'
      jsBundles: Array<string>
      cssBundles: Array<string>
    }
  | {
      mode: 'Dev'
      devServerConfig?: TheMillConfigServer
    }
)

export type TheMillRouteProps<TData> =
  | {
      data: null
      isLoading: true
    }
  | {
      data: TData
      isLoading: false
    }

export interface TheMillLayoutProps {
  children: ReactNode
}
