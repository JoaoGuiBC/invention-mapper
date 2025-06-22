import type { Invention } from '@store'

import { api } from '../api'

interface CreateInventionRequest {
  name: string
  text: string
  external_link: string
  lat: number
  lon: number
  file: File
}

export async function createInvention(data: CreateInventionRequest) {
  const formData = new FormData()

  for (const [key, value] of Object.entries(data)) {
    formData.append(key, value)
  }

  const result = await api
    .post('invention', { body: formData, throwHttpErrors: false })
    .json<Invention | string>()

  return result
}
