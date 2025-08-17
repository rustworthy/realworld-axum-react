use crate::AppContext;
use std::sync::Arc;
use utoipa::ToSchema;
use utoipa_axum::router::OpenApiRouter;

mod comments;
mod crud;
mod favorite;

// ---------------------------- SHARED TYPES -----------------------------------
#[derive(Debug, Serialize, ToSchema)]
pub(crate) struct Article {
    /// Article's slug.
    #[schema(example = "how-to-train-your-dragon", format = "slug")]
    slug: String,
}

/// Container for all `article` related endpoints.
///
/// See <https://realworld-docs.netlify.app/specifications/backend/endpoints/>
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub(crate) struct ArticlePayload<U> {
    article: U,
}

// ------------------------------- ROUTER --------------------------------------
pub(crate) fn router(ctx: Arc<AppContext>) -> OpenApiRouter {
    let articles_router = OpenApiRouter::new().routes(routes!(crud::create_article,));

    OpenApiRouter::new()
        .nest("/articles", articles_router)
        .with_state(ctx)
}
