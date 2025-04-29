import type { TheMillConfig, TheMillConfigServer } from '../config'

export interface InternalTheMillConfig extends Omit<TheMillConfig, 'server'> {
  server: TheMillConfigServer
}
