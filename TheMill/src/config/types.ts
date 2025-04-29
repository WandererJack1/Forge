import type {
  AliasOptions,
  DepOptimizationOptions,
  PluginOption,
  CSSOptions,
} from 'vite'

export interface TheMillConfigServer {
  host: string
  origin: string | null
  port: number
}

/**
 * @see http://TheMill.dev/documentation/configuration
 */
export interface TheMillConfig {
  server?: Partial<TheMillConfigServer>
  vite?: {
    alias?: AliasOptions
    css?: CSSOptions
    optimizeDeps?: DepOptimizationOptions
    plugins?: Array<PluginOption>
  }
}
