import { persistentAtom } from '@nanostores/persistent'
import { atom } from 'nanostores'

export interface Invention {
  id: string
  name: string
  text: string
  external_link: string
  lat: number
  lon: number
}

interface ToastInfo {
  message: string
  type: 'error' | 'success'
}

interface InventionCoordinate {
  lat: number
  lon: number
}

export const isArticleHidden = atom(true)

export const inventionList = atom<Array<Invention>>([])

export const activeInvention = persistentAtom<Invention | undefined>(
  '@mapper/activeInvention',
  undefined,
  {
    encode: JSON.stringify,
    decode: JSON.parse,
  }
)

export const toastInfo = persistentAtom<ToastInfo | undefined>(
  '@mapper/toastInfo',
  undefined,
  {
    encode: JSON.stringify,
    decode: JSON.parse,
  }
)

export const newInventionCoordinates = atom<InventionCoordinate | null>(null)
