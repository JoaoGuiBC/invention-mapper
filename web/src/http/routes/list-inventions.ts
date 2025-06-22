import type { Invention } from '@store'

import { api } from '../api'

export async function listInventions() {
  const result = await api.get('invention').json<Array<Invention>>()

  return result
}
