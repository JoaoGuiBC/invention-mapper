// .prettierrc.mjs
/** @type {import("prettier").Config} */
export default {
  plugins: ['prettier-plugin-astro'],
  overrides: [
    {
      files: '*.astro',
      options: {
        parser: 'astro',
        astroSkipFrontmatter: false,
        printWidth: 110,
        semi: false,
      },
    },
  ],
}
