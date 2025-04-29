import type { JSX, ReactNode } from 'react'
import { createContext, useContext, useMemo } from 'react'

import type { ServerPayload } from '../types'
import { SERVER_PAYLOAD_VARIABLE_NAME } from '../constants'

const isServerSide = typeof window === 'undefined'

interface TheMillContextValue {
  serverPayload: ServerPayload
}

const TheMillContext = createContext({} as TheMillContextValue)

interface TheMillContextProviderProps {
  serverPayload?: ServerPayload

  children: ReactNode
}

/**
 * @warning THIS SHOULD NOT BE EXPOSED TO USERLAND
 *
 * @see https://github.com/TheJackMan33/TheMill/issues/410
 */
export function TheMillContextProvider({
  serverPayload,
  children,
}: TheMillContextProviderProps): JSX.Element {
  const contextValue: TheMillContextValue = useMemo(() => {
    // At least one of these two should be defined
    const _serverPayload = (
      isServerSide ? serverPayload : window[SERVER_PAYLOAD_VARIABLE_NAME]
    ) as ServerPayload

    return {
      // Maybe this logic should be integrated using defaults
      serverPayload: _serverPayload,
    }
  }, [serverPayload])

  return <TheMillContext value={contextValue}>{children}</TheMillContext>
}

/**
 * @warning THIS SHOULD NOT BE EXPOSED TO USERLAND
 */
export function useTheMillContextServerPayload(): TheMillContextValue['serverPayload'] {
  return useContext(TheMillContext).serverPayload
}
