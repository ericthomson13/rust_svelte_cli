# Rust Svelte CLI
This is a CLI designed specifically to help build out components in V2 svelte for a specific project structure.  This leverages WASM and will eventually allow for use via NPM

# Creating a New Component

- Start by running the following
``` npm run new-component ```

- You will then be prompted for several pieces of the component structure
``` Does this component need a presenter? (Y/n) ```
``` Is this component a global component? (Y/n) ```
  - If not global component will have more prompts
  ``` Which regional files does this component need? (global/{region}) ```
- This will allow the regionalCompilerConfigs to populate as needed
``` Which regions is this component turned on for? (global, {region} ) ```
- The cli will then run and create the necessary files including stubeed out unit and e2e tests.
