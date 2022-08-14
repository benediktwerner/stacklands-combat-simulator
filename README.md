# Stacklands Combat Simulator

Try it out on <https://benediktwerner.github.io/stacklands-combat-simulator/>.

Simulates arbitrary combat scenarios for [Stacklands](https://sokpop.itch.io/stacklands).

## Build

- `./build.sh` to build the Simulator component (Rust -> WebAssembly)
- `cd svelte`
  - `npm install` dependencies
  - `npm run dev` to run the front-end dev server
  - `npm run build` to build the front-end for production
- `./deploy.sh` to deploy to GitHub Pages
