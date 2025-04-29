import type { TheMillConfig } from 'TheMill/config'
import tailwindcss from '@tailwindcss/vite'

const config: TheMillConfig = {
  vite: {
    plugins: [tailwindcss()],
  },
}

export default config
