import type { TheMillConfig } from 'TheMill/config'
import mdx from '@mdx-js/rollup'

const config: TheMillConfig = {
  vite: {
    optimizeDeps: {
      exclude: ['@mdx-js/react'],
    },
    plugins: [
      { enforce: 'pre', ...mdx({ providerImportSource: '@mdx-js/react' }) },
    ],
  },
}

export default config
