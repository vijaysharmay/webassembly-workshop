Code for the meetup organized by [Mixed Nuts at Pramati](https://www.meetup.com/Mixed-Nuts-at-Pramati/) on WebAssembly on [23rd Nov 2019](https://www.meetup.com/Mixed-Nuts-at-Pramati/events/266487952/)

### Setup & Dependencies

- rust toolchain for WASM (cargo, rustup, wasm32-unknown-unknown, wasm-pack)
- emscripten
- nodejs
- wrangler
- wasmtime

Please run setup.sh on a Ubuntu machine (VM/Container/Host) for quickly setting up the environment

For wasm_bindgen_demo & slow_function_demo, please run below

```sh
wasm-pack build
```

### References

- [A Cartoon Intro to WebAssembly by Lin Clark](https://hacks.mozilla.org/2017/02/a-cartoon-intro-to-webassembly/)
- [A Cartoon Intro to WebAssembly by Lin Clark (Video)](https://www.youtube.com/watch?v=HktWin_LPf4)
- [Isolation without Containers by Tyler McMullen](https://www.youtube.com/watch?v=2EDH-TxSo6U)
- [WebAssembly Explorer](https://mbebenita.github.io/WasmExplorer/)
- [Rust CF Worker Template](https://github.com/cloudflare/rustwasm-worker-template.git)
- [An Interesting V8 bug](https://abiondo.me/2019/01/02/exploiting-math-expm1-v8/)
- [WASI Interface Types](https://hacks.mozilla.org/2019/08/webassembly-interface-types/)
- [Previous Session on WASM at Mixed Nuts](https://labs.imaginea.com/talk-the-nuts-and-bolts-of-webassembly/)
- [Source map speedup using WASM](https://medium.com/@JevanChan/speed-up-source-map-generation-with-webassembly-google-summer-of-code-2018-e67407ed2e49)
- [WASM Ecosystem](https://github.com/mbasso/awesome-wasm)
