# mermaid-worker

Mermaid diagrams rendered to SVG on a Cloudflare Worker, using
[mermaid-rs-renderer](https://github.com/1jehuang/mermaid-rs-renderer) compiled to wasm.

- `GET /` – editor with live preview
- `POST /render` – Mermaid source in the body, SVG out (`400` + message on parse errors)

Uses the `wasm-support` branch of [our fork](https://github.com/Nachtalb/mermaid-rs-renderer/tree/wasm-support)
until it lands upstream.

```sh
npx wrangler dev      # local
npx wrangler deploy   # deploy
```
