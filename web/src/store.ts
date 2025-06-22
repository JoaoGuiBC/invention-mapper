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

export const inventionList = atom<Array<Invention>>([
  // {
  //   id: 'ablubluble',
  //   external_link: 'http://localhost:4321',
  //   lat: 1,
  //   lon: 1,
  //   name: 'Que porra ein',
  //   text: 'Lorem ipsum dolor sit, amet consectetur adipisicing elit. Iure maiores quasi accusantium saepe laudantium ad, ratione a velit illo reprehenderit eligendi. Labore debitis doloremque neque eum fugit. Placeat, expedita commodi. Lorem ipsum dolor sit amet consectetur adipisicing elit. Quidem consequuntur molestiae ratione illum, quo error natus libero unde accusantium, a ipsa dignissimos odit fugit earum totam? Quas accusamus quo distinctio? Lorem ipsum dolor sit amet consectetur adipisicing elit. Sequi veniam, numquam quam laboriosam, cumque ducimus inventore quaerat voluptatem excepturi doloremque nulla expedita molestias. Sequi sapiente incidunt facere consequatur. Corporis, ratione?',
  // },
])

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
