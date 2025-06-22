import { defineAction } from 'astro:actions'
import { z } from 'astro:schema'

import { createInvention, createInventionWithWiki, updateInvention } from '@http/index'

export const server = {
  insertInvention: defineAction({
    accept: 'form',
    input: z.object({
      name: z
        .string({ message: 'CAMPO OBRIGATÓRIO' })
        .min(4, { message: 'FORNEÇA O NOME COMPLETO' }),
      text: z
        .string({ message: 'CAMPO OBRIGATÓRIO' })
        .min(20, { message: 'FORNEÇA UMA DESCRIÇÃO COMPLETA' }),
      external_link: z
        .string({ message: 'CAMPO OBRIGATÓRIO' })
        .url({ message: 'FORMATO DO LINK INVÁLIDO' }),
      lat: z.coerce.number({ message: 'CAMPO OBRIGATÓRIO' }),
      lon: z.coerce.number({ message: 'CAMPO OBRIGATÓRIO' }),
      file: z
        .instanceof(File, { message: 'FORNEÇA UMA IMAGEM' })
        .refine(file => file.size > 0, { message: 'FORNEÇA UMA IMAGEM' })
        .refine(
          file =>
            [
              'image/png',
              'image/jpeg',
              'image/jpg',
              'image/svg+xml',
              'image/gif',
            ].includes(file.type),
          { message: 'FORNEÇA UMA IMAGEM' }
        ),
    }),
    handler: async input => {
      const response = await createInvention(input)

      return response
    },
  }),
  insertInventionWithWiki: defineAction({
    accept: 'form',
    input: z.object({
      wikipedia_link: z
        .string({ message: 'CAMPO OBRIGATÓRIO' })
        .url({ message: 'FORMATO DO LINK INVÁLIDO' })
        .refine(url => url.includes('wikipedia'), {
          message: 'LINK DA WIKIPÉDIA INVÁLIDO',
        }),
      lat: z.coerce.number({ message: 'CAMPO OBRIGATÓRIO' }),
      lon: z.coerce.number({ message: 'CAMPO OBRIGATÓRIO' }),
    }),
    handler: async input => {
      const response = await createInventionWithWiki(input)

      return response
    },
  }),
  updateInvention: defineAction({
    accept: 'form',
    input: z.object({
      id: z
        .string({ message: 'CAMPO OBRIGATÓRIO' })
        .uuid({ message: 'FORMATO INVÁLIDO' }),
      name: z
        .string({ message: 'CAMPO OBRIGATÓRIO' })
        .min(4, { message: 'FORNEÇA O NOME COMPLETO' }),
      text: z
        .string({ message: 'CAMPO OBRIGATÓRIO' })
        .min(20, { message: 'FORNEÇA UMA DESCRIÇÃO COMPLETA' }),
      external_link: z
        .string({ message: 'CAMPO OBRIGATÓRIO' })
        .url({ message: 'FORMATO DO LINK INVÁLIDO' }),
      lat: z.coerce.number({ message: 'CAMPO OBRIGATÓRIO' }),
      lon: z.coerce.number({ message: 'CAMPO OBRIGATÓRIO' }),
      file: z
        .instanceof(File, { message: 'FORNEÇA UMA IMAGEM' })
        .transform(file => {
          if (file.size === 0) return undefined

          return file
        })
        .refine(
          file => {
            if (!file) {
              return true
            }

            return [
              'image/png',
              'image/jpeg',
              'image/jpg',
              'image/svg+xml',
              'image/gif',
            ].includes(file.type)
          },
          { message: 'FORNEÇA UMA IMAGEM' }
        ),
    }),
    handler: async input => {
      const response = await updateInvention(input)

      return response
    },
  }),
}
