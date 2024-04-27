use axum::{
    body::{Body, Bytes},
    extract,
    http::{header, StatusCode},
    response::{Html, Response},
    routing::get,
    Router,
};
use include_dir::{include_dir, Dir};
use mime_guess::{self, Mime};

const ASSETS_DIR: Dir<'_> = include_dir!("assets/");

pub fn build_routes(router: Router<()>) -> Router<()> {
    router
        .route("/", get(landing))
        .route("/assets/*path", get(get_asset))
        .fallback(landing)
}

/* Methods to get frontend assets */
pub async fn landing() -> Html<&'static str> {
    let file = ASSETS_DIR
        .get_file("index.html")
        .expect("missing index.html")
        .contents_utf8()
        .unwrap();
    Html(file)
}

pub async fn get_asset(
    extract::Path(path): extract::Path<String>,
) -> Result<Response<Body>, StatusCode> {
    let file = match ASSETS_DIR.get_file(path.clone()) {
        Some(file) => file,
        None => {
            return Err(StatusCode::NOT_FOUND);
        }
    };

    let bytes = Bytes::from_static(file.contents());

    let mime_type = mime_guess::from_path(path)
        .first()
        .map(|m| m.to_string())
        .unwrap_or("application/octet-stream".to_string());

    Ok(Response::builder()
        .header(header::CONTENT_TYPE, mime_type)
        .body(bytes.into())
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?)
}
