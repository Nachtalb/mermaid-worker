# mermaid-worker

Mermaid diagrams rendered on a Cloudflare Worker, using
[mermaid-rs-renderer](https://github.com/1jehuang/mermaid-rs-renderer) compiled to wasm.

**Live:** https://mmd.naa.gg

- `GET /` – editor with live preview and download
- `POST /render?format=svg|png|webp&scale=2` – Mermaid source in the body, image out
  (`400` + message on parse errors). `scale` (0.25–8, default 2) only applies to raster formats;
  output is scaled down to at most 2 MP to fit the free plan's limits.

```sh
# from a file
curl --data-binary @diagram.mmd 'https://mmd.naa.gg/render?format=png&scale=2' -o diagram.png

# inline, no file needed
curl --data-binary @- 'https://mmd.naa.gg/render?format=svg' -o diagram.svg <<'EOF'
flowchart LR
  A[Start] --> B{OK?}
  B -->|yes| C[Done]
EOF
```

Raster output uses resvg with a bundled Liberation Sans (SIL OFL, see `fonts/`), since Workers
have no system fonts.

Uses the `worker` branch of [our fork](https://github.com/Nachtalb/mermaid-rs-renderer/tree/worker)
(wasm support + sequence-diagram fixes) until those land upstream.

## Deploy your own

Prebuilt, no Rust needed: grab `mermaid-worker-*.tar.gz` from the
[latest release](https://github.com/Nachtalb/mermaid-worker/releases/latest), then

```sh
tar xzf mermaid-worker-*.tar.gz && cd mermaid-worker
npx wrangler deploy
```

From source (Rust with the `wasm32-unknown-unknown` target; change or drop the `routes` line in
`wrangler.toml` first):

```sh
npx wrangler dev      # local
npx wrangler deploy   # deploy
```
