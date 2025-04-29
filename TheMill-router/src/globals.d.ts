import type { Router } from './router'

declare global {
  interface Window {
    __TheMill__ROUTER__: Router
  }
}
