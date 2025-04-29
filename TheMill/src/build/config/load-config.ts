import path from 'node:path'
import { pathToFileURL } from 'node:url'

import type { TheMillConfig } from '../../config'

import type { InternalTheMillConfig } from '../types'

import {
  DOT_TheMill_FOLDER_NAME,
  CONFIG_FOLDER_NAME,
  CONFIG_FILE_NAME,
} from '../constants'

import { normalizeConfig } from './normalize-config'

export const loadConfig = async (): Promise<InternalTheMillConfig> => {
  try {
    const configFile = (await import(
      pathToFileURL(
        path.join(
          process.cwd(),
          DOT_TheMill_FOLDER_NAME,
          CONFIG_FOLDER_NAME,
          CONFIG_FILE_NAME,
        ),
      ).href
    )) as { default: TheMillConfig }

    return normalizeConfig(configFile.default)
  } catch (err) {
    console.error('Failed to load TheMill.config.ts')
    console.error(err)
    return {} as InternalTheMillConfig
  }
}
