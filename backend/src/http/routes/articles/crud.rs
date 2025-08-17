use super::{Article, ArticlePayload, Author};
use crate::{
    http::{
        errors::{Error, Validation},
        extractors::UserID,
    },
    state::AppContext,
};
use axum::Json;
use axum::extract::{State, rejection::JsonRejection};
use chrono::Utc;
use std::sync::Arc;
use url::Url;
use utoipa::ToSchema;

#[allow(unused)]
#[derive(Debug, Deserialize, ToSchema)]
pub struct ArticleCreate {
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
}

/// Create new article.
///
/// This will create register a new article in the database assigning it a slug,
/// which uniquely identifies it among other articles and can used to fetch it.
#[utoipa::path(
    post,
    path = "",
    tags = ["Articles"],
    responses(
        (status = 201, description = "Article successfully created", body = ArticlePayload<Article>),
        (status = 422, description = "Missing or invalid article attributes", body = Validation),
        (status = 500, description = "Internal server error."),
    ),
    security(("HttpAuthBearerJWT" = [])),
)]
#[instrument(
    name = "CREATE ARTICLE",
    fields(slug = tracing::field::Empty)
    skip_all,
)]
#[allow(unused_variables)]
pub async fn create_article(
    ctx: State<Arc<AppContext>>,
    id: UserID,
    input: Result<Json<ArticlePayload<ArticleCreate>>, JsonRejection>,
) -> Result<Json<ArticlePayload<Article>>, Error> {
    let ArticlePayload { article } = input?.0;
    let user = sqlx::query!(
        r#"SELECT username, bio, image FROM "users" WHERE user_id = $1"#,
        &*id
    )
    .fetch_one(&ctx.db)
    .await?;
    let slug = slug::slugify(&article.title);

    let image = user
        .image
        .as_deref()
        .map(|v| Url::parse(v).map_err(|_| anyhow::anyhow!("asd")))
        .transpose()?;

    Ok(Json(ArticlePayload {
        article: Article {
            slug: slug::slugify(&article.title),
            title: article.title,
            body: article.body,
            description: article.description,
            tags: article.tags,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            favorited: false,
            favorited_count: 0,
            author: Author {
                bio: user.bio,
                image,
                username: user.username,
                following: false,
            },
        },
    }))
}
