use super::{Article, ArticlePayload, Author};
use crate::http::routes::users;
use crate::{
    http::{
        errors::{Error, Validation},
        extractors::UserID,
    },
    state::AppContext,
};
use axum::Json;
use axum::extract::{Path, State, rejection::JsonRejection};
use serde::Deserialize;
use std::sync::Arc;
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
    let slug = slug::slugify(&article.title);
    let details = sqlx::query!(
        r#"
        WITH article as (
            INSERT INTO "articles" (user_id, slug, title, description, body, tags)
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING user_id, created_at, updated_at
        )
        SELECT
            article.created_at as article_created_at,
            article.updated_at as article_updated_at,
            author.username as author_username,
            author.bio as author_bio,
            author.image as author_image 
        FROM "article" JOIN "users" author USING (user_id);
        "#,
        &*id,
        slug,
        article.title,
        article.description,
        article.body,
        &article.tags,
    )
    .fetch_one(&ctx.db)
    .await?;

    Ok(Json(ArticlePayload {
        article: Article {
            slug,
            title: article.title,
            body: article.body,
            description: article.description,
            tags: article.tags,
            created_at: details.article_created_at,
            updated_at: details
                .article_updated_at
                .unwrap_or(details.article_created_at),
            favorited: false,
            favorited_count: 0,
            author: Author {
                bio: details.author_bio,
                image: users::utils::parse_image_url(details.author_image.as_deref())?,
                username: details.author_username,
                following: false,
            },
        },
    }))
}

/// Read article by slug.
///
/// This will fetch an article by its unique slug identifier.
/// No authentication is required to read articles.
#[utoipa::path(
    get,
    path = "/{slug}",
    tags = ["Articles"],
    params(
        (
            "slug" = String, Path, 
            format = "slug",
            description = "Article slug identifier",
            example = "how-to-design-a-programming-language"
        ),
    ),
    responses(
        (status = 200, description = "Article successfully retrieved", body = ArticlePayload<Article>),
        (status = 404, description = "Article not found"),
        (status = 500, description = "Internal server error."),
    ),
)]
#[instrument(
    name = "READ ARTICLE",
    fields(slug = %slug),
    skip_all,
)]
pub async fn read_article(
    ctx: State<Arc<AppContext>>,
    Path(slug): Path<String>,
) -> Result<Json<ArticlePayload<Article>>, Error> {
    let details = sqlx::query!(
        r#"
        SELECT
            article.slug,
            article.title,
            article.description,
            article.body,
            article.tags,
            article.created_at,
            article.updated_at,
            article.favorited_count,
            author.username as author_username,
            author.bio as author_bio,
            author.image as author_image
        FROM "articles" article
        JOIN "users" author USING (user_id)
        WHERE slug = $1;
        "#,
        slug,
    )
    .fetch_optional(&ctx.db)
    .await?
    .ok_or(Error::NotFound)?;

    Ok(Json(ArticlePayload {
        article: Article {
            slug: details.slug,
            title: details.title,
            body: details.body,
            description: details.description,
            tags: details.tags,
            created_at: details.created_at,
            updated_at: details.updated_at.unwrap_or(details.created_at),
            // since this endpoint does not require authentication, there is
            // no way to tell if the article has been favorited by them
            favorited: false,
            favorited_count: details.favorited_count.try_into().unwrap_or(0),
            author: Author {
                bio: details.author_bio,
                image: users::utils::parse_image_url(details.author_image.as_deref())?,
                username: details.author_username,
                // Similat to `favorited`, we cannot tell if they are following
                // the author and we are defaulting this to `false`
                following: false,
            },
        },
    }))
}
