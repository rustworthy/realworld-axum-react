CREATE TABLE IF NOT EXISTS comments (
  comment_id    UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
  body          TEXT NOT NULL,
  article_id    UUID NOT NULL REFERENCES "articles" (article_id) ON DELETE CASCADE,
  user_id       UUID NOT NULL REFERENCES "users" (user_id) ON DELETE CASCADE,
  created_at    TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  updated_at    TIMESTAMPTZ
);

SELECT put_creation_mutation_timestamps_guard_on('comments');
