## Technologies run-down

- Nuxt 3
- Tauri 2
- UnoCSS
- Typescript
- ESLint

## Setup

  - Before running this app, you need to configure your environment with Rust. Take a look at the [Tauri docs](https://v2.tauri.app/start/prerequisites).
  - This project enforces [pnpm](https://pnpm.io). In order to use another package manager you need to update `package.json` and `tauri.conf.json`
  - The frontend runs on the usual port `3000` of Nuxt, the Tauri server uses the port `3001`. This settings are customizable in the `nuxt.config.ts` and `tauri.conf.json`.
  - Once ready, follow these commands:

  ```sh
  # go into the folder
  $ cd label-printer-app

  # install dependencies
  $ pnpm install

  # start the project
  $ pnpm run tauri:dev
  ```

  This will run the Nuxt frontend and will launch the Tauri window.

## Build

  ```sh
  $ pnpm run tauri:build
  ```

This command will generate the Nuxt static output and bundle the project under `src-tauri/target`.

## Debug

  ```sh
  $ pnpm run tauri:build:debug
  ```

The same Tauri bundle will generate under `src-tauri/target`, but with the ability to open the console.

## Notes

- Tauri v2 brings some big refactors, such as packages names and permission management. New permissions have to be granted under `src-tauri/capabilities/main.json`
- Tauri js functions are auto imported with the help of a custom module, named like `useTauri<LibraryName>`. If another Tauri plugin is added, then the module has to be updated to support its functions under `src/modules/tauri.ts`
- As per documentation, Nuxt SSR must be disabled in order for Tauri to act as the backend. Still, all Nuxt goodies will be functional.

## License

MIT License © 2024 - Present [João Batista](https://github.com/jotapebatista)
