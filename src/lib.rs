use std::sync::{Arc, OnceLock};

use resvg::{tiny_skia, usvg};
use worker::*;

const PAGE: &str = include_str!("index.html");
const FONT_REGULAR: &[u8] = include_bytes!("../fonts/LiberationSans-Regular.ttf");
const FONT_BOLD: &[u8] = include_bytes!("../fonts/LiberationSans-Bold.ttf");
// Free-plan Workers get ~128 MB and little CPU per request; larger outputs are scaled down to fit.
const MAX_PIXELS: f32 = 2_000_000.0;

fn fontdb() -> Arc<usvg::fontdb::Database> {
    static DB: OnceLock<Arc<usvg::fontdb::Database>> = OnceLock::new();
    DB.get_or_init(|| {
        let mut db = usvg::fontdb::Database::new();
        db.load_font_data(FONT_REGULAR.to_vec());
        db.load_font_data(FONT_BOLD.to_vec());
        db.set_sans_serif_family("Liberation Sans");
        Arc::new(db)
    })
    .clone()
}

fn rasterize(svg: &str, scale: f32) -> std::result::Result<tiny_skia::Pixmap, String> {
    let opt = usvg::Options {
        font_family: "Liberation Sans".into(),
        fontdb: fontdb(),
        ..Default::default()
    };
    let tree = usvg::Tree::from_str(svg, &opt).map_err(|e| e.to_string())?;
    let base = tree.size();
    let scale = scale.min((MAX_PIXELS / (base.width() * base.height())).sqrt());
    let size = base.to_int_size().scale_by(scale).ok_or("bad size")?;
    let mut pixmap = tiny_skia::Pixmap::new(size.width(), size.height()).ok_or("bad size")?;
    // Opaque background keeps premultiplied == straight RGBA for the webp encoder.
    pixmap.fill(tiny_skia::Color::WHITE);
    resvg::render(
        &tree,
        tiny_skia::Transform::from_scale(scale, scale),
        &mut pixmap.as_mut(),
    );
    Ok(pixmap)
}

fn encode(pixmap: &tiny_skia::Pixmap, format: &str) -> std::result::Result<Vec<u8>, String> {
    let (w, h) = (pixmap.width(), pixmap.height());
    match format {
        "png" => pixmap.encode_png().map_err(|e| e.to_string()),
        "webp" => {
            let mut out = Vec::new();
            image_webp::WebPEncoder::new(&mut out)
                .encode(pixmap.data(), w, h, image_webp::ColorType::Rgba8)
                .map_err(|e| e.to_string())?;
            Ok(out)
        }
        _ => Err(format!("unsupported format '{format}' (svg, png, webp)")),
    }
}

fn render(src: &str, format: &str, scale: f32) -> std::result::Result<(Vec<u8>, String), String> {
    let svg = mermaid_rs_renderer::render(src).map_err(|e| format!("{e:#}"))?;
    if format == "svg" {
        return Ok((svg.into_bytes(), "image/svg+xml".into()));
    }
    let pixmap = rasterize(&svg, scale)?;
    Ok((encode(&pixmap, format)?, format!("image/{format}")))
}

#[event(fetch)]
async fn fetch(mut req: Request, _env: Env, _ctx: Context) -> Result<Response> {
    if req.method() != Method::Post || req.path() != "/render" {
        return Response::from_html(PAGE);
    }
    let url = req.url()?;
    let q = |k: &str| {
        url.query_pairs()
            .find(|(n, _)| n == k)
            .map(|(_, v)| v.into_owned())
    };
    let format = q("format").unwrap_or_else(|| "svg".into());
    let scale = q("scale")
        .and_then(|s| s.parse::<f32>().ok())
        .unwrap_or(2.0)
        .clamp(0.25, 8.0);
    let src = req.text().await?;
    match render(&src, &format, scale) {
        Ok((body, ctype)) => {
            let mut r = Response::from_bytes(body)?;
            r.headers_mut().set("content-type", &ctype)?;
            Ok(r)
        }
        Err(e) => Response::error(e, 400),
    }
}
