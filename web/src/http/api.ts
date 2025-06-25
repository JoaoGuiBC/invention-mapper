import ky from 'ky'

import { API_URL } from 'astro:env/client'

export const api = ky.create({ prefixUrl: `${API_URL}/api` })
