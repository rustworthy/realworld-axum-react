CREATE TABLE IF NOT EXISTS "articles" (
    article_id      UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id         UUID NOT NULL REFERENCES "users" (user_id) ON DELETE CASCADE,
    slug            TEXT UNIQUE NOT NULL,
    title           TEXT NOT NULL,
    description     TEXT NOT NULL,
    body            TEXT NOT NULL,
    tags            TEXT[] NOT NULL CHECK ( cardinality(tags) > 0 ),
    favorited_count INTEGER NOT NULL DEFAULT 0,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at      TIMESTAMPTZ
);

SELECT put_creation_mutation_timestamps_guard_on('articles');

CREATE INDEX articles_tags_gin ON "articles" USING gin(tags);

