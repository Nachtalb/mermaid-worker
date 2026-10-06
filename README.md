# mermaid-worker

Mermaid diagrams rendered on a Cloudflare Worker, using
[mermaid-rs-renderer](https://github.com/1jehuang/mermaid-rs-renderer) compiled to wasm.

- `GET /` – editor with live preview and download
- `POST /render?format=svg|png|webp&scale=2` – Mermaid source in the body, image out
  (`400` + message on parse errors). `scale` (0.25–8, default 2) only applies to raster formats;
  output is scaled down to at most 2 MP to fit the free plan's limits.

Raster output uses resvg with a bundled Liberation Sans (SIL OFL, see `fonts/`), since Workers
have no system fonts.

Uses the `worker` branch of [our fork](https://github.com/Nachtalb/mermaid-rs-renderer/tree/worker)
(wasm support + sequence-diagram fixes) until those land upstream.

```sh
npx wrangler dev      # local
npx wrangler deploy   # deploy
```
