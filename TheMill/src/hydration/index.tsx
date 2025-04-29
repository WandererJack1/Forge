import { hydrateRoot } from 'react-dom/client'
import { createRouter } from 'TheMill-router'
import type { createRoute } from 'TheMill-router'

import { TheMillEntryPoint } from '../shared/TheMillEntryPoint'

type RouteTree = ReturnType<typeof createRoute>

export function hydrate(routeTree: RouteTree): void {
  // Create a new router instance
  const router = createRouter({ routeTree })

  hydrateRoot(document, <TheMillEntryPoint router={router} />)
}
