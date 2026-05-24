pub mod chat;
pub mod health;

use axum::Router;

pub fn router(state: super::AppState) -> Router<super::AppState> {
    let public = health::router();

    let protected = Router::<super::AppState>::new()
        .nest("/v1", chat::router())
        .route_layer(axum::middleware::from_fn_with_state(
            state.clone(),
            super::middleware::require_api_key,
        ));

    public.merge(protected)
}
