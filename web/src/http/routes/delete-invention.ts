import { api } from '../api'

interface DeleteInventionRequest {
  id: string
}

export async function deleteInvention({ id }: DeleteInventionRequest) {
  const result = await api
    .delete(`invention/${id}`, { throwHttpErrors: false })
    .json<string>()

  return result
}
