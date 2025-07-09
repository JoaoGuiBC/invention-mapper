# Invention Mapper

Frontend do projeto **Invention Mapper**, desenvolvido com [Astro](https://astro.build/) e focado em visualização e cadastro de invenções.

## Principais Tecnologias

- **Astro**: Framework principal para construção de interfaces.
- **TailwindCSS**: Utilizado para estilização via classes utilitárias.
- **Leaflet**: Biblioteca para mapas interativos.
- **Nanostores**: Gerenciamento de estado reativo e persistente.
- **Ky**: Cliente HTTP minimalista para requisições à API.
- **Astro Cloudinary**: Otimização e manipulação de imagens.
- **TanStack Query**: Gerenciamento de cache e requisições assíncronas.
- **TypeScript**: Tipagem estática e segurança no desenvolvimento.

## Padrões de Projeto

- **Componentização**: Componentes reutilizáveis em `src/components`.
- **Store centralizada**: Estado global com Nanostores em `src/store.ts`.
- **Rotas e ações separadas**: Lógica de API em `src/http/routes`.
- **Configuração de caminhos (alias)**: Definidos em `tsconfig.json` para facilitar imports.

## Configuração e Setup

1. **Pré-requisitos**:
   - Node.js 18+
   - pnpm (ou npm/yarn)

2. **Instalação**:
   ```bash
   pnpm install
   ```

3. **Variáveis de ambiente**:
   - Configure as variáveis no arquivo `.env` conforme o schema do `astro.config.mjs`:
     - `API_URL` (padrão: http://localhost:3333/api)
     - `CLOUDINARY_API_SECRET`, `CLOUDINARY_API_KEY`, `PUBLIC_CLOUDINARY_CLOUD_NAME`, `CLOUDINARY_IMAGES_FOLDER` (opcional)

4. **Rodando em modo desenvolvimento**:
   ```bash
   pnpm dev
   ```

5. **Build de produção**:
   ```bash
   pnpm build
   pnpm preview
   ```

## Scripts Úteis

- `pnpm dev` — Inicia o servidor de desenvolvimento.
- `pnpm build` — Gera o build de produção.
- `pnpm preview` — Visualiza o build localmente. 