# Cloudflare Worker Demo

There are many examples online on how to use cloudflare workers online, but very few demonstrate how to use them with a HTML template engine (in this case the [typed-html](https://github.com/bodil/typed-html) crate, which maynot be a full blown template engine, but does provide some useful macros).

This project was generated using the [rustwasm-worker-template](https://github.com/cloudflare/rustwasm-worker-template.git) project.

This template is designed for compiling Rust libraries into WebAssembly and
publishing the resulting worker to Cloudflare's worker infrastructure.

## Usage

To preview on the local machine, use below command

```
wrangler preview
```

To publish on cloudflare's infrastructure (need a valid account_id), use below command

```
wrangler publish
```

