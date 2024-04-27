use axum::Router;

#[cfg(not(debug_assertions))]
mod assets;

#[shuttle_runtime::main]
async fn axum() -> shuttle_axum::ShuttleAxum {
    let router = Router::new();

    #[cfg(not(debug_assertions))]
    let router = assets::build_routes(router);

    Ok(router.into())
}
