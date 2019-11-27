# WASI Demo

A simple demo showcasing WASI using a rust program. This program takes two arguments - an input file and an output file. It copies the contents of the input file and adds a random text, in this case, "Also mixed nuts is awesome" is added in a new line.

You need wasmtime to run this and the WASM file was generated using the below command(ignore warnings, if any),

```sh
rustup target add wasm32-wasi
cargo build --target wasm32-wasi
cp target/wasm32-wasi/debug/wasi_demo.wasm .
```

and then run the below to execute it

```sh
wasmtime --dir=. wasi_demo.wasm <file1> <file2>
```
