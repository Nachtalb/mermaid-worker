use worker::*;

const PAGE: &str = r#"<!doctype html><meta charset=utf-8><title>mmdr</title>
<style>body{display:flex;gap:1em;font-family:sans-serif;margin:1em}textarea{width:40vw;height:90vh}#o{flex:1;overflow:auto}</style>
<textarea id=i>flowchart LR
  A[Start] --> B{Ok?}
  B -->|yes| C[Done]
  B -->|no| A</textarea><div id=o></div>
<script>const i=document.getElementById('i'),o=document.getElementById('o');let t;
async function r(){const res=await fetch('/render',{method:'POST',body:i.value});o.innerHTML=res.ok?await res.text():'<pre>'+(await res.text())+'</pre>'}
i.oninput=()=>{clearTimeout(t);t=setTimeout(r,250)};r()</script>"#;

#[event(fetch)]
async fn fetch(mut req: Request, _env: Env, _ctx: Context) -> Result<Response> {
    match (req.method(), req.path().as_str()) {
        (Method::Post, "/render") => {
            let src = req.text().await?;
            match mermaid_rs_renderer::render(&src) {
                Ok(svg) => {
                    let mut r = Response::ok(svg)?;
                    r.headers_mut().set("content-type", "image/svg+xml")?;
                    Ok(r)
                }
                Err(e) => Response::error(format!("{e:#}"), 400),
            }
        }
        _ => Response::from_html(PAGE),
    }
}
