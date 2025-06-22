// @ts-check
import tailwindcss from '@tailwindcss/vite'
import { defineConfig, envField } from 'astro/config'

// https://astro.build/config
export default defineConfig({
  vite: { plugins: [tailwindcss()] },
  env: {
    validateSecrets: true,
    schema: {
      /// API
      API_URL: envField.string({
        context: 'client',
        access: 'public',
        default: 'http://localhost:3333/api',
      }),

      /// CLOUDIARY
      CLOUDINARY_API_SECRET: envField.string({ context: 'server', access: 'secret' }),
      CLOUDINARY_API_KEY: envField.string({ context: 'server', access: 'secret' }),
      PUBLIC_CLOUDINARY_CLOUD_NAME: envField.string({
        context: 'server',
        access: 'secret',
      }),
      CLOUDINARY_IMAGES_FOLDER: envField.string({
        context: 'server',
        access: 'secret',
        optional: true,
      }),
    },
  },
})
