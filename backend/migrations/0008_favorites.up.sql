CREATE TABLE IF NOT EXISTS "favorites" (
    article_id      UUID NOT NULL REFERENCES "articles" (article_id) ON DELETE CASCADE,
    user_id         UUID NOT NULL REFERENCES "users" (user_id) ON DELETE CASCADE,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at      TIMESTAMPTZ,

    PRIMARY KEY (article_id, user_id)
);

SELECT put_creation_mutation_timestamps_guard_on('favorites');

ALTER TABLE "articles" DROP COLUMN favorited_count;

