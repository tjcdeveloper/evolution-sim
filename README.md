# Evolution Sim

An attempt to use Rust and WebAssembly to simulate evolution. Modify or add genes to change creatures behaviours, then run the sandbox to see how they live and evolve over generations.

## Getting started
To begin compile the rust code to WebAssembly using

```
wasm-pack build
```

Then change into the npm directory and start the webpack dev server using

```
cd npm
npm start
```