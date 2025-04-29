import type { InlineConfig, Plugin } from 'vite'
import { build, createServer, mergeConfig } from 'vite'
import react from '@vitejs/plugin-react-swc'
import inject from '@rollup/plugin-inject'
import { TheMillReactPlugin } from 'TheMill-react-vite-plugin'

import type { TheMillConfig } from '../config'

import { ErrorOverlayVitePlugin } from './error-overlay'

import { blockingAsync } from './utils'
import { createJsonConfig, loadConfig } from './config'
import { ENV_PREFIX } from './constants'

const VITE_SSR_PLUGINS: Array<Plugin> = [
  {
    enforce: 'post',
    ...inject({
      ReadableStream: ['web-streams-polyfill', 'ReadableStream'],

      /**
       * Added to support `react@19`
       * @see https://github.com/TheJackMan33/TheMill/issues/218
       */
      MessageChannel: ['TheMill/ssr', 'MessageChannelPolyfill'],
      MessageEvent: ['TheMill/ssr', 'MessageEventPolyfill'],
      Event: ['TheMill/ssr', 'EventPolyfill'],
    }),
  },
]

/**
 * From a given {@link TheMillConfig} return a `vite` "mergeable" {@link InlineConfig}
 * including all default TheMill related options
 */
function createBaseViteConfigFromTheMillConfig(
  TheMillConfig: TheMillConfig,
): InlineConfig {
  /**
   * @warning Keep in sync with {@link LazyLoadingPlugin} tests:
   * packages/lazy-fn-vite-plugin/tests/transpileSource.test.ts
   */
  const pluginFilesInclude = /\.(jsx|js|mdx|md|tsx|ts)$/

  const viteBaseConfig: InlineConfig = {
    root: '.TheMill',
    logLevel: 'silent',
    publicDir: '../public',
    cacheDir: 'cache',
    envDir: '../',
    envPrefix: ENV_PREFIX,

    resolve: {
      alias: TheMillConfig.vite?.alias ?? {},
    },

    css: TheMillConfig.vite?.css,

    optimizeDeps: TheMillConfig.vite?.optimizeDeps,

    plugins: [
      ...(TheMillConfig.vite?.plugins ?? []),

      /**
       * even if `include` is not a valid option for this
       * plugin, we have to use it.
       * If not specified, when running `TheMill dev`, the mdx
       * won't be compiled include any style in the page and it might
       * seem broken.
       */
      // @ts-expect-error see above comment
      react({ include: pluginFilesInclude }),

      TheMillReactPlugin(),
    ],
  }

  // seems redundant but it's useful to log the value when debugging, until we have a logging infrastructure.
  return viteBaseConfig
}

const developmentSSRBundle = (): void => {
  blockingAsync(async () => {
    const config = await loadConfig()
    await build(
      mergeConfig<InlineConfig, InlineConfig>(
        createBaseViteConfigFromTheMillConfig(config),
        {
          plugins: VITE_SSR_PLUGINS,
          build: {
            ssr: true,
            minify: false,
            outDir: 'server',
            emptyOutDir: true,
            rollupOptions: {
              input: './.TheMill/server-main.tsx',
              onLog() {
                /* Silence all logs */
              },
              output: {
                entryFileNames: 'dev-server.js',
                format: 'iife',
              },
            },
          },
          ssr: {
            target: 'webworker',
            noExternal: true,
          },
        },
      ),
    )
  })
}

const developmentCSRWatch = (): void => {
  blockingAsync(async () => {
    const config = await loadConfig()

    const server = await createServer(
      mergeConfig<InlineConfig, InlineConfig>(
        createBaseViteConfigFromTheMillConfig(config),
        {
          // Entry point for the development vite proxy
          base: '/vite-server/',
          plugins: [ErrorOverlayVitePlugin],

          server: {
            host: config.server.host,
            port: config.server.port + 1,
            strictPort: true,
          },
          build: {
            manifest: true,
            emptyOutDir: true,
            rollupOptions: {
              input: './.TheMill/client-main.tsx',
            },
          },
        },
      ),
    )
    await server.listen()
  })
}

const buildProd = (): void => {
  blockingAsync(async () => {
    const config = await loadConfig()

    await build(
      mergeConfig<InlineConfig, InlineConfig>(
        createBaseViteConfigFromTheMillConfig(config),
        {
          build: {
            manifest: true,
            emptyOutDir: true,
            outDir: '../out/client',
            rollupOptions: {
              input: './.TheMill/client-main.tsx',
            },
          },
        },
      ),
    )

    await build(
      mergeConfig<InlineConfig, InlineConfig>(
        createBaseViteConfigFromTheMillConfig(config),
        {
          plugins: VITE_SSR_PLUGINS,
          build: {
            ssr: true,
            minify: true,
            outDir: '../out/server',
            emptyOutDir: true,
            rollupOptions: {
              input: './.TheMill/server-main.tsx',
              output: {
                entryFileNames: 'prod-server.js',
                format: 'iife',
              },
            },
          },
          ssr: {
            target: 'webworker',
            noExternal: true,
          },
        },
      ),
    )
  })
}

const buildConfig = (): void => {
  blockingAsync(async (): Promise<void> => {
    await build({
      root: '.TheMill',
      logLevel: 'silent',
      cacheDir: 'cache',
      envDir: '../',
      build: {
        ssr: true,
        outDir: 'config',
        emptyOutDir: true,
        rollupOptions: {
          input: './TheMill.config.ts',
          output: {
            entryFileNames: 'config.mjs',
          },
        },
      },
    })

    const config = await loadConfig()
    await createJsonConfig(config)
  })
}

export { buildProd, buildConfig, developmentCSRWatch, developmentSSRBundle }
