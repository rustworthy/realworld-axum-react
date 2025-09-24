use super::{Article, ArticlePayload, Author};
use crate::http::errors::ResultExt as _;
use crate::http::extractors::MaybeUserID;
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
use axum::http::StatusCode;
use serde::Deserialize;
use std::sync::Arc;
use utoipa::ToSchema;
use validator::Validate;
use validator_derive::Validate;

// -------------------------------- CREATE ------------------------------------
#[derive(Debug, Deserialize, ToSchema, Validate)]
pub struct ArticleCreate {
    /// Article's title.
    ///
    /// This is will be used to generate a slug for this article.
    #[schema(
        examples("Your very own programming language", "Deploying with Kamal"),
        min_length = 1
    )]
    #[validate(length(min = 1, message = "title should be at least 1 character long"))]
    title: String,

    /// Article's description.
    #[schema(
        examples("This articles shares our knowledge on how to design a programming language",),
        min_length = 1
    )]
    #[validate(length(min = 1, message = "description should be at least 1 character long"))]
    description: String,

    /// Article's contents.
    #[schema(
        examples("Before we begin ... And that's pretty much it. Happy coding!",),
        min_length = 1
    )]
    #[validate(length(min = 1, message = "body should be at least 1 character long"))]
    body: String,

    /// Tags.
    #[schema(
        example = json!(vec!["programming".to_string(), "language design".to_string()]),
        min_items = 1,
    )]
    #[validate(length(min = 1, message = "tags list should contain at least 1 item"))]
    #[serde(rename = "tagList")]
    tags: Vec<String>,
}

/// Create article.
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
pub async fn create_article(
    ctx: State<Arc<AppContext>>,
    id: UserID,
    input: Result<Json<ArticlePayload<ArticleCreate>>, JsonRejection>,
) -> Result<(StatusCode, Json<ArticlePayload<Article>>), Error> {
    let ArticlePayload { article } = input?.0;
    article.validate()?;
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
    .await
    .on_constraint("articles_slug_key", |_| {
        Error::unprocessable_entity([("title", "article with this title already exists")])
    })?;

    let payload = ArticlePayload {
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
    };
    Ok((StatusCode::CREATED, Json(payload)))
}

// -------------------------------- UPDATE ------------------------------------
#[derive(Debug, Deserialize, ToSchema, Validate)]
pub struct ArticleUpdate {
    /// Article's title.
    ///
    /// This is will be used to generate a slug for this article.
    #[schema(
        examples("Your very own programming language", "Deploying with Kamal"),
        min_length = 1,
        nullable = false
    )]
    #[validate(length(min = 1, message = "title should be at least 1 character long"))]
    title: Option<String>,

    /// Article's description.
    #[schema(
        examples("This articles shares our knowledge on how to design a programming language",),
        min_length = 1,
        nullable = false
    )]
    #[validate(length(min = 1, message = "description should be at least 1 character long"))]
    description: Option<String>,

    /// Article's contents.
    #[schema(
        examples("Before we begin ... And that's pretty much it. Happy coding!",),
        min_length = 1,
        nullable = false
    )]
    #[validate(length(min = 1, message = "body should be at least 1 character long"))]
    body: Option<String>,

    /// Tags.
    #[schema(
        example = json!(vec!["programming".to_string(), "language design".to_string()]),
        min_items = 1,
        nullable = false
    )]
    #[validate(length(min = 1, message = "tags list should contain at least 1 item"))]
    #[serde(rename = "tagList")]
    tags: Option<Vec<String>>,
}

/// Update article.
///
/// This will update the existing article in the database. Note that if the title
/// in the update payload differs from the original title, the slug will be
/// re-calculated.
///
/// Note that (just like with user update endpoint) the method is `PUT` (as per
/// spec), but the payload can contain only a partial article (meaning it is
/// a patch operation).
//
/// Just like for all other mutation endpoints, authentication is required.
/// Moreover, only the article's author can perform this action.
#[utoipa::path(
    put,
    path = "/{slug}",
    tags = ["Articles"],
    params(
        (
            "slug" = String, Path,
            format = "slug",
            description = "Article's slug identifier.",
            example = "how-to-design-a-programming-language"
        ),
    ),
    responses(
        (status = 200, description = "Article successfully updated", body = ArticlePayload<Article>),
        (status = 401, description = "Token missing or invalid."),
        (status = 403, description = "User does not have permissions to delete this article."),
        (status = 404, description = "Article not found"),
        (status = 422, description = "Missing or invalid article attributes", body = Validation),
        (status = 500, description = "Internal server error."),
    ),
    security(("HttpAuthBearerJWT" = [])),
)]
#[instrument(name = "UPDATE ARTICLE", skip(ctx, input))]
pub async fn update_article(
    ctx: State<Arc<AppContext>>,
    Path(slug): Path<String>,
    uid: UserID,
    input: Result<Json<ArticlePayload<ArticleUpdate>>, JsonRejection>,
) -> Result<Json<ArticlePayload<Article>>, Error> {
    let ArticlePayload { article: patch } = input?.0;
    patch.validate()?;
    let new_slug = patch.title.as_deref().map(slug::slugify);
    let details = sqlx::query!(
        r#"
        WITH updated_article as (
            UPDATE articles
            SET
                slug = COALESCE($3, slug),
                title = COALESCE($4, title),
                description = COALESCE($5, description),
                body = COALESCE($6, body),
                tags = COALESCE($7, tags)
            WHERE slug = $1 AND user_id = $2
            RETURNING slug
        )
        SELECT
            EXISTS(SELECT article_id FROM articles WHERE slug = $1) "existed!",
            (SELECT slug FROM updated_article) "new_slug";
        "#,
        slug,
        *uid,
        new_slug,
        patch.title,
        patch.description,
        patch.body,
        patch.tags.as_deref()
    )
    .fetch_one(&ctx.db)
    .await
    .on_constraint("articles_slug_key", |_| {
        Error::unprocessable_entity([("title", "article with this title already exists")])
    })?;

    if let Some(slug) = details.new_slug {
        let article = db::read_article(&ctx, &slug, Some(&uid)).await?;
        return Ok(Json(ArticlePayload { article }));
    }

    let err = if details.existed {
        warn!("user tried to update article w/o proper permissions");
        Error::Forbidden
    } else {
        Error::NotFound
    };
    Err(err)
}

// --------------------------------- READ -------------------------------------
/// Read article by slug.
///
/// This will fetch an article by its unique slug identifier.
/// Authentication is _optional_, but needed to learn if the article has been
/// favorited (a.k.a. liked) by the user, or whether the user is following
/// the article's author.
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
        (status = 401, description = "Token missing or invalid (in case authenicated access has been used)"),
        (status = 404, description = "Article not found"),
        (status = 500, description = "Internal server error."),
    ),
    security(
        (),
        ("HttpAuthBearerJWT" = []),
    ),
)]
#[instrument(name = "READ ARTICLE", skip(ctx))]
pub async fn read_article(
    ctx: State<Arc<AppContext>>,
    Path(slug): Path<String>,
    uid: MaybeUserID,
) -> Result<Json<ArticlePayload<Article>>, Error> {
    let uid = uid.0.as_deref();
    let article = db::read_article(&ctx, &slug, uid).await?;
    Ok(Json(ArticlePayload { article }))
}

// -------------------------------- DELETE ------------------------------------
/// Delete article by slug.
///
/// This will delete the article with the specified unique slug identifier.
/// Authentication _is_ required to delete articles.
#[utoipa::path(
    delete,
    path = "/{slug}",
    tags = ["Articles"],
    params(
        (
            "slug" = String, Path, 
            format = "slug",
            description = "Article's slug identifier.",
            example = "how-to-design-a-programming-language"
        ),
    ),
    responses(
        (status = 204, description = "Article successfully deleted."),
        (status = 401, description = "Token missing or invalid."),
        (status = 403, description = "User does not have permissions to delete this article."),
        (status = 404, description = "Article not found"),
        (status = 500, description = "Internal server error."),
    ),
)]
#[instrument(name = "DELETE ARTICLE", skip(ctx))]
pub async fn delete_article(
    ctx: State<Arc<AppContext>>,
    Path(slug): Path<String>,
    uid: UserID,
) -> Result<StatusCode, Error> {
    let details = sqlx::query!(
        r#"
        WITH deleted_article as (
            DELETE FROM articles
            WHERE slug = $1 AND user_id = $2
            RETURNING article_id
        )
        SELECT 
            EXISTS(SELECT article_id FROM articles WHERE slug = $1) "existed!",
            EXISTS(SELECT article_id FROM deleted_article) "deleted!";
        "#,
        slug,
        *uid,
    )
    .fetch_one(&ctx.db)
    .await?;

    if details.deleted {
        return Ok(StatusCode::NO_CONTENT);
    }

    let err = if details.existed {
        warn!("user tried to delete article w/o proper permissions");
        Error::Forbidden
    } else {
        Error::NotFound
    };
    Err(err)
}

// ------------------------ FAVORITE / UNFAVORITE -----------------------------
/// Favorite article.
///
/// Note that this operation is idempotent: if this user already liked
/// the article in question, a successful response will be returned.
#[utoipa::path(
    post,
    path = "/{slug}/favorite",
    tags = ["Articles"],
    params(
        (
            "slug" = String, Path,
            format = "slug",
            description = "Article's slug identifier.",
            example = "how-to-design-a-programming-language"
        ),
    ),
    responses(
        (status = 200, description = "Article successfully updated", body = ArticlePayload<Article>),
        (status = 401, description = "Token missing or invalid."),
        (status = 404, description = "Article not found"),
        (status = 500, description = "Internal server error."),
    ),
    security(("HttpAuthBearerJWT" = [])),
)]
#[instrument(name = "FAVORITE ARTICLE", skip(ctx))]
pub async fn favorite_article(
    ctx: State<Arc<AppContext>>,
    Path(slug): Path<String>,
    uid: UserID,
) -> Result<Json<ArticlePayload<Article>>, Error> {
    let _article_id = sqlx::query_scalar!(
        r#"
        WITH
            existing_article AS (SELECT article_id FROM articles WHERE slug = $1),
            _favorite_action AS (
                INSERT INTO favorites (article_id, user_id)
                SELECT article_id, $2 FROM existing_article
                ON CONFLICT DO NOTHING
            )
        SELECT article_id FROM existing_article
        "#,
        slug,
        *uid
    )
    .fetch_optional(&ctx.db)
    .await
    // edge-case: a user has been deleted (or their ID has been changed which is
    // less likely) but the token is still valid; we are signalling that they
    // are not unathenticated so that we client-side app navigates they to try
    // and login which should make things more clear
    .on_constraint("favorites_user_id_fkey", |_| Error::Unauthorized)?
    .ok_or(Error::NotFound)?;
    let article = db::read_article(&ctx, &slug, Some(&uid)).await?;
    Ok(Json(ArticlePayload { article }))
}

/// Unfavorite article.
///
///
/// This is essentially revoking the previously given "like" (see the `favorite`
/// endpoint).
///
/// Similar to the `favorite` endpoint, this operation is idempotent: if this
/// user already revoked their like (or never liked in the first place), a successful
/// response will be returned.
///
/// Note that in the docs a generic example of a returned `article` may have
/// `"favited": true`, but in reality this will havae `false` since they are
/// revoking their "like".
#[utoipa::path(
    delete,
    path = "/{slug}/favorite",
    tags = ["Articles"],
    params(
        (
            "slug" = String, Path,
            format = "slug",
            description = "Article's slug identifier.",
            example = "how-to-design-a-programming-language"
        ),
    ),
    responses(
        (status = 200, description = "Article successfully updated", body = ArticlePayload<Article>),
        (status = 401, description = "Token missing or invalid."),
        (status = 404, description = "Article not found"),
        (status = 500, description = "Internal server error."),
    ),
    security(("HttpAuthBearerJWT" = [])),
)]
#[instrument(name = "UNFAVORITE ARTICLE", skip(ctx))]
pub async fn unfavorite_article(
    ctx: State<Arc<AppContext>>,
    Path(slug): Path<String>,
    uid: UserID,
) -> Result<Json<ArticlePayload<Article>>, Error> {
    let _article_id = sqlx::query_scalar!(
        r#"
        WITH
            existing_article AS (SELECT article_id FROM articles WHERE slug = $1),
            _unfavorite_action AS (
                DELETE FROM favorites
                WHERE article_id = (SELECT article_id FROM existing_article) AND user_id = $2
            )
        SELECT article_id FROM existing_article
        "#,
        slug,
        *uid
    )
    .fetch_optional(&ctx.db)
    .await
    // edge-case: a user has been deleted (or their ID has been changed which is
    // less likely) but the token is still valid; we are signalling that they
    // are not unathenticated so that we client-side app navigates they to try
    // and login which should make things more clear
    .on_constraint("favorites_user_id_fkey", |_| Error::Unauthorized)?
    .ok_or(Error::NotFound)?;
    let article = db::read_article(&ctx, &slug, Some(&uid)).await?;
    Ok(Json(ArticlePayload { article }))
}

mod db {
    use crate::AppContext;
    use crate::http::errors::Error;
    use crate::http::routes::articles::Article;
    use crate::http::routes::articles::Author;
    use crate::http::routes::users::utils as users_utils;
    use uuid::Uuid;

    #[instrument(name = "FETCH ARTICLE FROM DATABASE", skip(ctx))]
    pub async fn read_article(
        ctx: &AppContext,
        slug: &str,
        user_id: Option<&Uuid>,
    ) -> Result<Article, Error> {
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
                (
                    $2::UUID IS NOT NULL AND
                    EXISTS(
                        SELECT 1 FROM favorites
                        WHERE article_id = article.article_id AND user_id = $2::UUID
                    )
                ) AS "favorited!",
                (SELECT COUNT(*) FROM favorites WHERE article_id = article.article_id) AS favorited_count,
                author.username AS author_username,
                author.bio AS author_bio,
                author.image AS author_image
            FROM "articles" article
            JOIN "users" author USING (user_id)
            WHERE slug = $1;
            "#,
            slug,
            user_id
        )
        .fetch_optional(&ctx.db)
        .await?
        .ok_or(Error::NotFound)?;

        Ok(Article {
            slug: details.slug,
            title: details.title,
            body: details.body,
            description: details.description,
            tags: details.tags,
            created_at: details.created_at,
            updated_at: details.updated_at.unwrap_or(details.created_at),
            favorited: details.favorited,
            favorited_count: details.favorited_count.unwrap_or_default() as usize,
            author: Author {
                bio: details.author_bio,
                image: users_utils::parse_image_url(details.author_image.as_deref())?,
                username: details.author_username,
                // TODO: update once the follow/unfollow logic is there
                following: false,
            },
        })
    }
}
