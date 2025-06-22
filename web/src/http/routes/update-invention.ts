import type { Invention } from '@store'

import { api } from '../api'

interface UpdateInventionRequest {
  id: string
  name: string
  text: string
  external_link: string
  lat: number
  lon: number
  file?: File
}

export async function updateInvention(data: UpdateInventionRequest) {
  const formData = new FormData()

  for (const [key, value] of Object.entries(data)) {
    if (key === 'file' && !value) continue

    formData.append(key, value)
  }

  const result = await api
    .put('invention', { body: formData, throwHttpErrors: false })
    .json<string>()

  return result
}
