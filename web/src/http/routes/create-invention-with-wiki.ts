import type { Invention } from '@store'

import { api } from '../api'

interface CreateInventionWithWikiRequest {
  wikipedia_link: string
  lat: number
  lon: number
}

export async function createInventionWithWiki(data: CreateInventionWithWikiRequest) {
  const formData = new FormData()

  for (const [key, value] of Object.entries(data)) {
    formData.append(key, value)
  }

  const result = await api
    .post('invention/wiki', { body: formData, timeout: false, throwHttpErrors: false })
    .json<Invention | string>()

  return result
}
