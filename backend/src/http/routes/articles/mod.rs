use crate::AppContext;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use url::Url;
use utoipa::ToSchema;
use utoipa_axum::router::OpenApiRouter;

mod comments;
mod crud;
mod favorite;
mod list;

// ---------------------------- SHARED TYPES -----------------------------------

#[derive(Debug, Serialize, ToSchema)]
pub(crate) struct Author {
    /// User's name or nickname.
    ///
    /// This is  - just like the user's `email` - case-insensitively unique
    /// in the system.
    #[schema(example = "rob.pike1984")]
    username: String,

    /// User's biography.
    ///
    /// Empty string means biography has never been provided.
    bio: String,

    /// Location of user's image (if any).
    #[schema(required = true)]
    image: Option<Url>,

    /// If the current user is following the author.
    following: bool,
}

#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Article {
    /// Article's slug.
    #[schema(example = "how-to-train-your-dragon", format = "slug")]
    slug: String,

    /// Article's title.
    ///
    /// This is will be used to generate a slug for this article.
    #[schema(
        examples("Your very own programming language", "Deploying with Kamal"),
        min_length = 1
    )]
    title: String,

    /// Article's description.
    #[schema(
        examples("This articles shares our knowledge on how to design a programming language",),
        min_length = 1
    )]
    description: String,

    /// Article's contents.
    #[schema(
        examples("Before we begin ... And that's pretty much it. Happy coding!",),
        min_length = 1
    )]
    body: String,

    /// Tags.
    #[schema(
        example = json!(vec!["programming".to_string(), "language design".to_string()]),
        min_items = 1,
    )]
    #[serde(rename = "tagList")]
    tags: Vec<String>,

    /// When this article was created.
    created_at: DateTime<Utc>,

    /// When this article was last update.
    updated_at: DateTime<Utc>,

    /// If this article is favorited by the current user.
    favorited: bool,

    /// How many users favorited this article.
    #[serde(rename = "favoritesCount")]
    favorited_count: usize,

    /// The article's author details.
    author: Author,
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
    let articles_router = OpenApiRouter::new()
        .routes(routes!(
            crud::create_article,
            crud::update_article,
            crud::delete_article,
        ))
        .routes(routes!(crud::favorite_article, crud::unfavorite_article,))
        .routes(routes!(crud::read_article,))
        .routes(routes!(list::list_articles,))
        .routes(routes!(list::personal_feed,));

    OpenApiRouter::new()
        .nest("/articles", articles_router)
        .with_state(ctx)
}
